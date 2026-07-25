// TDLib 发送消息 ID 对齐。
//
// sendMessage 可能先返回临时 message_id，真正发送成功后再通过
// updateMessageSendSucceeded 给出最终 message_id。进度面板必须使用最终 ID，
// 否则 editMessageText 会报 `Message not found`。

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, MutexGuard};
use std::time::Duration;

use tokio::sync::oneshot;

type SendKey = (i32, i64, i64);
type SendResult = Result<SentMessageReceipt, String>;

/// 文本发送后业务层需要的轻量回执。
///
/// 生成的 TDLib `Message` 包含非常大的枚举；只为读取消息 ID 而复制该类型会在
/// Windows debug worker 的深异步调用栈上触发栈溢出。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SentMessageReceipt {
    pub id: i64,
    pub chat_id: i64,
    pub is_temporary: bool,
}

/// 最多缓存多少条“先收到成功 update，后注册等待者”的消息。
const COMPLETED_CACHE_LIMIT: usize = 256;
/// 等待普通文本消息发送成功的时间。超时后仍返回临时消息，避免命令入口卡死。
const SEND_SUCCEEDED_WAIT_TIMEOUT: Duration = Duration::from_secs(10);

static SEND_STATE: LazyLock<Mutex<SendState>> = LazyLock::new(|| Mutex::new(SendState::default()));

#[derive(Default)]
struct SendState {
    /// 临时消息键 -> 最终发送成功的轻量回执。
    completed: HashMap<SendKey, SentMessageReceipt>,
    /// 临时消息键 -> 正在等待最终发送结果的调用方。
    waiters: HashMap<SendKey, Vec<oneshot::Sender<SendResult>>>,
}

/// 等待 TDLib 把临时 message_id 替换成最终 message_id。
///
/// 如果消息没有 sending_state，说明已经是最终消息，直接返回。
pub async fn wait_for_sent_message(
    message: tdlib_rs::types::Message,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::Message> {
    wait_for_sent_message_with_timeout(message, client_id, SEND_SUCCEEDED_WAIT_TIMEOUT).await
}

/// 等待 TDLib 把临时 message_id 替换成最终 message_id，并允许业务场景指定等待窗口。
///
/// 媒体上传可能在服务端处理较慢，上传调用方应使用更长窗口；普通机器人文本仍使用
/// `wait_for_sent_message` 的短窗口，避免单条回复长时间阻塞。
pub async fn wait_for_sent_message_with_timeout(
    message: tdlib_rs::types::Message,
    client_id: i32,
    timeout: Duration,
) -> anyhow::Result<tdlib_rs::types::Message> {
    if message.sending_state.is_none() {
        return Ok(message);
    }

    let temporary = SentMessageReceipt {
        id: message.id,
        chat_id: message.chat_id,
        is_temporary: true,
    };
    let final_receipt =
        wait_for_sent_message_receipt_with_timeout(temporary, client_id, timeout).await?;
    if final_receipt.is_temporary || final_receipt.id == message.id {
        return Ok(message);
    }

    tdlib_rs::functions::get_message(final_receipt.chat_id, final_receipt.id, client_id)
        .await
        .map(|message| {
            let tdlib_rs::enums::Message::Message(message) = message;
            message
        })
        .map_err(|error| {
            anyhow::anyhow!(
                "get final sent message failed: code={}, message={}",
                error.code,
                error.message
            )
        })
}

/// 等待文本消息从临时 ID 对齐到最终 ID，全程只传递轻量回执。
pub async fn wait_for_sent_message_receipt(
    receipt: SentMessageReceipt,
    client_id: i32,
) -> anyhow::Result<SentMessageReceipt> {
    wait_for_sent_message_receipt_with_timeout(receipt, client_id, SEND_SUCCEEDED_WAIT_TIMEOUT)
        .await
}

async fn wait_for_sent_message_receipt_with_timeout(
    receipt: SentMessageReceipt,
    client_id: i32,
    timeout: Duration,
) -> anyhow::Result<SentMessageReceipt> {
    if !receipt.is_temporary {
        return Ok(receipt);
    }

    let key = (client_id, receipt.chat_id, receipt.id);
    let rx = {
        let mut state = lock_send_state();
        if let Some(final_receipt) = state.completed.get(&key) {
            return Ok(*final_receipt);
        }

        let (tx, rx) = oneshot::channel();
        state.waiters.entry(key).or_default().push(tx);
        rx
    };

    match tokio::time::timeout(timeout, rx).await {
        Ok(Ok(Ok(final_receipt))) => Ok(final_receipt),
        Ok(Ok(Err(error))) => {
            anyhow::bail!("message send failed after initial response: {error}")
        }
        Ok(Err(_closed)) => {
            tracing::warn!(
                chat_id = receipt.chat_id,
                temporary_message_id = receipt.id,
                "message send waiter dropped, use temporary id"
            );
            Ok(receipt)
        }
        Err(_elapsed) => {
            prune_closed_waiters(key);
            tracing::warn!(
                chat_id = receipt.chat_id,
                temporary_message_id = receipt.id,
                "wait message send succeeded timeout, use temporary id"
            );
            Ok(receipt)
        }
    }
}

/// 根据临时 message_id 等待最终 message_id。
///
/// 编辑消息遇到 `Message not found` 时使用这个兜底：如果之前用的是临时 ID，
/// 这里会等到 `updateMessageSendSucceeded` 后返回最终 ID，再让调用方重试编辑。
pub async fn wait_for_sent_message_id(
    client_id: i32,
    chat_id: i64,
    temporary_message_id: i64,
    timeout: Duration,
) -> Option<i64> {
    wait_for_sent_message_receipt_with_timeout(
        SentMessageReceipt {
            id: temporary_message_id,
            chat_id,
            is_temporary: true,
        },
        client_id,
        timeout,
    )
    .await
    .ok()
    .filter(|receipt| !receipt.is_temporary)
    .map(|receipt| receipt.id)
}

/// 记录指定 TDLib client 的发送成功 update。
///
/// 双 client 模式下 user 和 bot 可能在同一个 chat 中同时发送消息，TDLib 临时
/// message_id 不能假设跨 client 唯一，因此缓存键必须包含 client_id。
pub fn observe_message_send_succeeded_for_client(
    update: tdlib_rs::types::UpdateMessageSendSucceeded,
    client_id: i32,
) {
    let key = (client_id, update.message.chat_id, update.old_message_id);
    let receipt = SentMessageReceipt {
        id: update.message.id,
        chat_id: update.message.chat_id,
        is_temporary: false,
    };
    let waiters = {
        let mut state = lock_send_state();
        // 即使当前已有等待者，也缓存最终 ID。等待者可能已经因为超时被丢弃，
        // 后续编辑进度消息时仍需要通过临时 ID 找回最终 ID。
        state.completed.insert(key, receipt);
        trim_completed_cache(&mut state);
        state.waiters.remove(&key)
    };

    if let Some(waiters) = waiters {
        for waiter in waiters {
            let _ = waiter.send(Ok(receipt));
        }
    }
}

/// 记录指定 TDLib client 的发送失败 update。
pub fn observe_message_send_failed_for_client(
    update: tdlib_rs::types::UpdateMessageSendFailed,
    client_id: i32,
) {
    let key = (client_id, update.message.chat_id, update.old_message_id);
    let waiters = {
        let mut state = lock_send_state();
        state.waiters.remove(&key)
    };

    if let Some(waiters) = waiters {
        let error = format!(
            "code={}, message={}",
            update.error.code, update.error.message
        );
        for waiter in waiters {
            let _ = waiter.send(Err(error.clone()));
        }
    }
}

/// 防止极端竞态下 completed 缓存无界增长。
fn trim_completed_cache(state: &mut SendState) {
    while state.completed.len() > COMPLETED_CACHE_LIMIT {
        let Some(key) = state.completed.keys().next().copied() else {
            return;
        };
        state.completed.remove(&key);
    }
}

/// 清理已经超时或取消的等待者，避免没有后续 TDLib update 时内存表持续增长。
fn prune_closed_waiters(key: SendKey) {
    let mut state = lock_send_state();
    let Some(waiters) = state.waiters.get_mut(&key) else {
        return;
    };
    waiters.retain(|waiter| !waiter.is_closed());
    if waiters.is_empty() {
        state.waiters.remove(&key);
    }
}

/// 获取发送状态锁。
///
/// 发送状态只是进度卡片 message_id 对齐缓存；如果某个异步任务 panic 导致锁中毒，
/// 继续恢复缓存比让所有后续消息发送都 panic 更安全。
fn lock_send_state() -> MutexGuard<'static, SendState> {
    match SEND_STATE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("recover poisoned send message state mutex");
            poisoned.into_inner()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicI64, Ordering};

    use super::*;

    static NEXT_CHAT_ID: AtomicI64 = AtomicI64::new(-900_000);
    const TEST_CLIENT_ID: i32 = 100;

    /// 每个测试使用独立 chat，避免全局发送状态缓存互相影响。
    fn next_chat_id() -> i64 {
        NEXT_CHAT_ID.fetch_sub(1, Ordering::SeqCst)
    }

    /// 构造最小可用的文本消息，测试只关心 chat_id、message_id 和 sending_state。
    fn test_message(
        chat_id: i64,
        message_id: i64,
        sending_state: Option<tdlib_rs::enums::MessageSendingState>,
    ) -> tdlib_rs::types::Message {
        tdlib_rs::types::Message {
            id: message_id,
            sender_id: tdlib_rs::enums::MessageSender::User(tdlib_rs::types::MessageSenderUser {
                user_id: 1,
            }),
            chat_id,
            sending_state,
            scheduling_state: None,
            is_outgoing: true,
            is_pinned: false,
            is_from_offline: false,
            can_be_saved: true,
            has_timestamped_media: false,
            is_channel_post: false,
            is_paid_star_suggested_post: false,
            is_paid_ton_suggested_post: false,
            contains_unread_mention: false,
            date: 0,
            edit_date: 0,
            forward_info: None,
            import_info: None,
            interaction_info: None,
            unread_reactions: vec![],
            fact_check: None,
            suggested_post_info: None,
            reply_to: None,
            topic_id: None,
            self_destruct_type: None,
            self_destruct_in: 0.0,
            auto_delete_in: 0.0,
            via_bot_user_id: 0,
            sender_business_bot_user_id: 0,
            sender_boost_count: 0,
            sender_tag: String::new(),
            paid_message_star_count: 0,
            author_signature: String::new(),
            media_album_id: 0,
            effect_id: 0,
            restriction_info: None,
            summary_language_code: String::new(),
            content: tdlib_rs::enums::MessageContent::MessageText(tdlib_rs::types::MessageText {
                text: tdlib_rs::types::FormattedText {
                    text: "test".to_owned(),
                    entities: vec![],
                },
                link_preview: None,
                link_preview_options: None,
            }),
            reply_markup: None,
        }
    }

    #[tokio::test]
    async fn test_completed_message_cache_is_not_consumed_by_first_waiter() {
        let chat_id = next_chat_id();
        let temporary_id = -10;
        let final_id = 20;
        let final_message = test_message(chat_id, final_id, None);

        observe_message_send_succeeded_for_client(
            tdlib_rs::types::UpdateMessageSendSucceeded {
                message: final_message,
                old_message_id: temporary_id,
            },
            TEST_CLIENT_ID,
        );

        let resolved = wait_for_sent_message_receipt(
            SentMessageReceipt {
                id: temporary_id,
                chat_id,
                is_temporary: true,
            },
            TEST_CLIENT_ID,
        )
        .await
        .expect("cached final message should resolve");
        assert_eq!(resolved.id, final_id);

        let resolved_id = wait_for_sent_message_id(
            TEST_CLIENT_ID,
            chat_id,
            temporary_id,
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(resolved_id, Some(final_id));
    }

    #[tokio::test]
    async fn test_success_update_after_waiter_timeout_keeps_final_message_id() {
        let chat_id = next_chat_id();
        let temporary_id = -11;
        let final_id = 21;

        let timed_out = wait_for_sent_message_id(
            TEST_CLIENT_ID,
            chat_id,
            temporary_id,
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(timed_out, None);

        observe_message_send_succeeded_for_client(
            tdlib_rs::types::UpdateMessageSendSucceeded {
                message: test_message(chat_id, final_id, None),
                old_message_id: temporary_id,
            },
            TEST_CLIENT_ID,
        );

        let resolved_id = wait_for_sent_message_id(
            TEST_CLIENT_ID,
            chat_id,
            temporary_id,
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(resolved_id, Some(final_id));
    }

    /// 上传场景使用更长的自定义等待窗口时，应接住稍后到达的最终消息 ID。
    #[tokio::test]
    async fn test_custom_wait_timeout_resolves_delayed_final_message() {
        let chat_id = next_chat_id();
        let temporary_id = -14;
        let final_id = 34;

        let observer = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            observe_message_send_succeeded_for_client(
                tdlib_rs::types::UpdateMessageSendSucceeded {
                    message: test_message(chat_id, final_id, None),
                    old_message_id: temporary_id,
                },
                TEST_CLIENT_ID,
            );
        });

        let resolved = wait_for_sent_message_receipt_with_timeout(
            SentMessageReceipt {
                id: temporary_id,
                chat_id,
                is_temporary: true,
            },
            TEST_CLIENT_ID,
            Duration::from_millis(100),
        )
        .await
        .expect("delayed successful send should resolve within custom timeout");
        observer.await.expect("observer task should finish");

        assert_eq!(resolved.id, final_id);
    }

    #[tokio::test]
    async fn test_timed_out_waiter_is_pruned_from_state() {
        let chat_id = next_chat_id();
        let temporary_id = -12;

        let timed_out = wait_for_sent_message_id(
            TEST_CLIENT_ID,
            chat_id,
            temporary_id,
            Duration::from_millis(1),
        )
        .await;
        assert_eq!(timed_out, None);

        let state = lock_send_state();
        assert!(
            !state
                .waiters
                .contains_key(&(TEST_CLIENT_ID, chat_id, temporary_id))
        );
    }

    /// 双 client 模式下，两个 TDLib client 可能出现相同 chat_id + 临时 message_id。
    /// 发送状态缓存必须按 client_id 隔离，否则 bot/user 可能误用对方的最终 message_id。
    #[tokio::test]
    async fn test_message_cache_is_isolated_by_client_id() {
        let chat_id = next_chat_id();
        let temporary_id = -13;

        observe_message_send_succeeded_for_client(
            tdlib_rs::types::UpdateMessageSendSucceeded {
                message: test_message(chat_id, 31, None),
                old_message_id: temporary_id,
            },
            201,
        );
        observe_message_send_succeeded_for_client(
            tdlib_rs::types::UpdateMessageSendSucceeded {
                message: test_message(chat_id, 32, None),
                old_message_id: temporary_id,
            },
            202,
        );

        let client_201 =
            wait_for_sent_message_id(201, chat_id, temporary_id, Duration::from_millis(1)).await;
        let client_202 =
            wait_for_sent_message_id(202, chat_id, temporary_id, Duration::from_millis(1)).await;

        assert_eq!(client_201, Some(31));
        assert_eq!(client_202, Some(32));
    }
}
