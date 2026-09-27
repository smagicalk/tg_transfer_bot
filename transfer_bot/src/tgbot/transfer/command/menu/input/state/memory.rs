//! `/menu` 菜单输入状态的纯内存辅助模块。
//!
//! 负责维护无需持久化到数据库的瞬态交互优化数据与进程内并发互斥控制：
//! 1. 记忆用户最近一次确认的目标聊天 ID（`last_target`），用于简化后续快速转存操作；
//! 2. 基于会话键（`DraftKey = (chat_id, user_id)`）的细粒度异步锁机制，确保单个会话内的并发输入与草稿修改串行化，防止数据竞争。

use std::collections::{HashMap, HashSet};
use std::sync::MutexGuard;

use super::{DraftKey, MENU_DRAFT_ACTIVE_KEYS, MENU_LAST_TARGETS};

/// 在内存中记录指定用户在当前会话下最近一次确认转存/操作的目标聊天 ID。
///
/// # 参数说明
/// - `chat_id`: 发起交互的群组/频道或私聊会话 ID
/// - `user_id`: 发起操作的用户 Telegram ID
/// - `target_chat_id`: 选定的目标聊天会话 ID
pub(in crate::tgbot::transfer::command::menu) fn remember_last_target(
    chat_id: i64,
    user_id: i64,
    target_chat_id: i64,
) {
    let mut targets = lock_menu_last_targets();
    targets.insert((chat_id, user_id), target_chat_id);
    tracing::debug!(
        chat_id,
        user_id,
        target_chat_id,
        "menu last target remembered"
    );
}

/// 读取指定用户在当前会话下最近一次使用过的目标聊天 ID。
///
/// # 参数说明
/// - `chat_id`: 会话聊天 ID
/// - `user_id`: 用户 ID
///
/// # 返回值
/// 若之前记录过目标聊天 ID 则返回 `Some(target_chat_id)`，否则返回 `None`。
pub(in crate::tgbot::transfer::command::menu) fn last_target(
    chat_id: i64,
    user_id: i64,
) -> Option<i64> {
    let targets = lock_menu_last_targets();
    targets.get(&(chat_id, user_id)).copied()
}

/// 清空内存中记录的所有最近目标聊天映射表。
///
/// 仅用于单元测试或集成测试前后，避免多个测试用例之间因共享进程内静态状态而产生污染。
#[cfg(test)]
pub(in crate::tgbot::transfer::command::menu) fn clear_last_targets() {
    let mut targets = lock_menu_last_targets();
    targets.clear();
}

/// 获取最近目标映射表的互斥锁守卫。
///
/// 当锁发生中毒（Poisoned）时，捕获异常并提取内部数据进行恢复，防止偶发的 panic 导致全局菜单功能永久不可用。
fn lock_menu_last_targets() -> MutexGuard<'static, HashMap<DraftKey, i64>> {
    match MENU_LAST_TARGETS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("recover poisoned menu last target mutex");
            poisoned.into_inner()
        }
    }
}

/// 异步申请指定草稿键（`DraftKey`）的进程内细粒度并发锁。
///
/// 设计原理：
/// - 该函数不跨越 `.await` 持有底层的同步 `MutexGuard`，仅在锁保护下检查并向活跃集合插入键；
/// - 若当前键已被其他协程持有，则释放锁并短暂休眠（20ms）后轮询重试；
/// - 成功占用后返回 RAII 守卫 `MenuDraftKeyGuard`，守卫离开作用域被 Drop 时自动从活跃集合移除该键。
///
/// # 参数说明
/// - `key`: 会话草稿唯一键 `(chat_id, user_id)`
///
/// # 返回值
/// 绑定了该会话键的 `MenuDraftKeyGuard` 守卫实例。
pub(in crate::tgbot::transfer::command::menu) async fn acquire_draft_key_guard(
    key: DraftKey,
) -> MenuDraftKeyGuard {
    loop {
        {
            let mut keys = lock_menu_draft_active_keys();
            // 若成功插入，说明当前没有其他协程正在处理该用户的草稿
            if keys.insert(key) {
                return MenuDraftKeyGuard { key };
            }
        }
        // 存在并发操作，短暂挂起等待后重试
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
}

/// 草稿键并发互斥 RAII 守卫结构体。
///
/// 当该守卫被销毁（Drop）时，自动释放对对应 `DraftKey` 的独占占用。
pub(in crate::tgbot::transfer::command::menu) struct MenuDraftKeyGuard {
    /// 被保护的草稿键元组 `(chat_id, user_id)`
    key: DraftKey,
}

impl Drop for MenuDraftKeyGuard {
    fn drop(&mut self) {
        let mut keys = lock_menu_draft_active_keys();
        // 从当前活跃处理集合中移除本会话键
        keys.remove(&self.key);
    }
}

/// 获取当前处于处理中的草稿键集合的互斥锁守卫。
///
/// 具备自动从锁中毒状态中恢复集合的能力，确保高可用性。
fn lock_menu_draft_active_keys() -> MutexGuard<'static, HashSet<DraftKey>> {
    match MENU_DRAFT_ACTIVE_KEYS.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::error!("recover poisoned menu draft key mutex");
            poisoned.into_inner()
        }
    }
}
