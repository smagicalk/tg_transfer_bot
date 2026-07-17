// `/menu` 目标流程相关的 inline callback。
// 这里集中处理“选目标 -> 确认 -> 返回”的多步向导按钮。

use std::sync::Arc;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::text::{
    build_menu_recovery_text, build_step_prompt_with_context, build_target_input_prompt_text,
};
use super::flow::{ExistingCommandContext, run_existing_command};
use super::state::{
    ConfirmContextTakeResult, DraftKey, TargetContext, TargetContextAdvanceResult,
    TargetDraftAdvance, advance_target_context, remember_last_target, take_confirm_context,
};
use super::target::{
    TargetPromptContext, edit_confirm_prompt, edit_target_choice_prompt, resolve_default_target_on,
};

/// 目标选择 callback 的公共上下文。
///
/// 多个目标按钮都需要同一组 TDLib 消息坐标；收拢后目标推进逻辑更容易保持一致。
#[derive(Clone)]
struct TargetCallbackContext {
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
}

impl TargetCallbackContext {
    /// 构造 callback 共享上下文。
    fn new(
        callback_query_id: i64,
        chat_id: i64,
        message_id: i64,
        sender_user_id: i64,
        client_id: i32,
    ) -> Self {
        Self {
            callback_query_id,
            chat_id,
            message_id,
            sender_user_id,
            client_id,
        }
    }

    /// 当前草稿的隔离键。
    fn draft_key(&self) -> DraftKey {
        (self.chat_id, self.sender_user_id)
    }

    /// 向 Telegram ACK 当前按钮点击。
    async fn answer(&self, text: &'static str) -> anyhow::Result<()> {
        send::answer_callback_query(self.callback_query_id, Some(text), self.client_id).await
    }
}

/// 处理“使用默认目标”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_default_callback_query(
    app: &crate::app_context::AppContext,
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    let target_chat_id = resolve_default_target_on(app, &config, chat_id);

    select_target_for_callback_on(
        ctx,
        target_chat_id,
        default_target_selected_tip(target_chat_id, chat_id),
    )
    .await
}

/// 默认目标 callback 的提示应描述最终解析到的位置。
fn default_target_selected_tip(target_chat_id: i64, request_chat_id: i64) -> &'static str {
    if target_chat_id == request_chat_id {
        "已选择当前私聊"
    } else {
        "已选择默认目标"
    }
}

/// 把当前目标选择草稿推进到确认页，并编辑原 inline 卡片。
async fn select_target_for_callback_on(
    ctx: TargetCallbackContext,
    target_chat_id: i64,
    selected_tip: &'static str,
) -> anyhow::Result<()> {
    let Some(context) = advance_target_context_for_callback(
        ctx.draft_key(),
        TargetDraftAdvance::Confirm { target_chat_id },
        ctx.callback_query_id,
        ctx.chat_id,
        "没有等待选择目标的输入",
        ctx.client_id,
    )
    .await?
    else {
        return Ok(());
    };
    ctx.answer(selected_tip).await?;
    edit_confirm_prompt(
        context.kind,
        &context.source_link,
        target_chat_id,
        ctx.chat_id,
        ctx.message_id,
        ctx.client_id,
    )
    .await
}

/// 处理“常用目标”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_alias_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    target_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    select_target_for_callback_on(ctx, target_chat_id, "已选择目标").await
}

/// 处理“手动输入目标”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_manual_callback_query(
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    let Some(context) = advance_target_context_for_callback(
        ctx.draft_key(),
        TargetDraftAdvance::TargetChat,
        callback_query_id,
        chat_id,
        "没有等待选择目标的输入",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };

    ctx.answer("请输入目标").await?;
    edit_target_input_waiting_card(
        ctx.chat_id,
        ctx.message_id,
        ctx.client_id,
        "2/3",
        "等待手动输入",
        "请回复目标 chat_id、配置别名，或回复 default。",
        &context.source_link,
    )
    .await;
    send::send_card_message_with_force_reply_returning(
        build_target_input_prompt_text(
            &context.source_link,
            "输入目标",
            "请回复数字 chat_id、配置里的目标别名，或回复 default 使用配置默认目标。",
        ),
        ctx.chat_id,
        "输入目标 chat_id、别名或 default",
        ctx.client_id,
    )
    .await?;
    Ok(())
}

/// 处理“确认页执行”按钮。
#[allow(clippy::too_many_arguments)]
pub(in crate::tgbot::transfer::command::menu) async fn target_confirm_callback_query(
    app: &crate::app_context::AppContext,
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let key = (chat_id, sender_user_id);
    let confirm = match confirm_callback_decision(take_confirm_context(key).await?) {
        ConfirmCallbackDecision::Run(confirm) => confirm,
        ConfirmCallbackDecision::Recover {
            callback_tip,
            title,
            status,
            detail,
        } => {
            send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
            send_input_recovery_card(chat_id, client_id, title, status, detail).await?;
            return Ok(());
        }
        ConfirmCallbackDecision::WaitForTarget { callback_tip } => {
            send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
            return Ok(());
        }
    };

    remember_last_target(chat_id, sender_user_id, confirm.target_chat_id);
    send::answer_callback_query(callback_query_id, Some("开始执行"), client_id).await?;
    run_existing_command(
        confirm.kind,
        vec![
            confirm.kind.command_name().to_owned(),
            confirm.source_link,
            confirm.target_chat_id.to_string(),
        ],
        config,
        ExistingCommandContext {
            // 确认按钮要把执行上下文 move 给现有转存/查询命令入口；这里局部取一次全局 Arc
            // 比在确认状态结构里长期持有整份运行态更简单，也能维持状态表纯净。
            app: std::sync::Arc::new(app.clone()),
            request_chat_id: chat_id,
            request_message_id: message_id,
            actor,
            client_id,
        },
    )
    .await
}

/// 确认按钮消费结果对应的后续动作。
#[derive(Debug, Clone, PartialEq, Eq)]
enum ConfirmCallbackDecision {
    Run(super::state::ConfirmContext),
    Recover {
        callback_tip: &'static str,
        title: &'static str,
        status: &'static str,
        detail: &'static str,
    },
    WaitForTarget {
        callback_tip: &'static str,
    },
}

/// 把状态层结果映射为 UI 层动作，避免确认按钮入口混杂多段提示文案。
fn confirm_callback_decision(result: ConfirmContextTakeResult) -> ConfirmCallbackDecision {
    match result {
        ConfirmContextTakeResult::Active(confirm) => ConfirmCallbackDecision::Run(confirm),
        ConfirmContextTakeResult::Expired => ConfirmCallbackDecision::Recover {
            callback_tip: "输入已过期",
            title: "输入已过期",
            status: "expired",
            detail: "上一次确认已超过有效时间，请重新打开菜单发起操作。",
        },
        ConfirmContextTakeResult::None => ConfirmCallbackDecision::Recover {
            callback_tip: "没有待执行的输入",
            title: "没有待执行的输入",
            status: "empty",
            detail: "当前没有可确认的菜单输入，请重新打开菜单发起操作。",
        },
        ConfirmContextTakeResult::WrongStep => ConfirmCallbackDecision::WaitForTarget {
            callback_tip: "请先选择目标",
        },
    }
}

/// 目标推进结果对应的 callback UI 动作。
#[derive(Debug, Clone, PartialEq, Eq)]
enum TargetAdvanceCallbackDecision {
    Continue(TargetContext),
    Recover {
        callback_tip: &'static str,
        title: &'static str,
        status: &'static str,
        detail: &'static str,
    },
    WaitForSource {
        callback_tip: &'static str,
    },
}

/// 把状态层的目标推进结果映射为统一的 callback 提示和恢复动作。
fn target_advance_callback_decision(
    result: TargetContextAdvanceResult,
    missing_tip: &'static str,
) -> TargetAdvanceCallbackDecision {
    match result {
        TargetContextAdvanceResult::Active(context) => {
            TargetAdvanceCallbackDecision::Continue(context)
        }
        TargetContextAdvanceResult::Expired => TargetAdvanceCallbackDecision::Recover {
            callback_tip: "输入已过期",
            title: "输入已过期",
            status: "expired",
            detail: "上一次菜单输入已超过有效时间，请重新打开 /menu。",
        },
        TargetContextAdvanceResult::None => TargetAdvanceCallbackDecision::Recover {
            callback_tip: missing_tip,
            title: missing_tip,
            status: "empty",
            detail: "当前按钮对应的输入流程已经不存在，请重新打开菜单。",
        },
        TargetContextAdvanceResult::WrongStep => TargetAdvanceCallbackDecision::WaitForSource {
            callback_tip: "请先发送源链接",
        },
    }
}

/// 处理“返回选择目标”按钮。
pub(in crate::tgbot::transfer::command::menu) async fn target_back_callback_query(
    app: &crate::app_context::AppContext,
    callback_query_id: i64,
    chat_id: i64,
    message_id: i64,
    sender_user_id: i64,
    config: Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<()> {
    let ctx = TargetCallbackContext::new(
        callback_query_id,
        chat_id,
        message_id,
        sender_user_id,
        client_id,
    );
    let Some(context) = advance_target_context_for_callback(
        ctx.draft_key(),
        TargetDraftAdvance::TargetChoice,
        callback_query_id,
        chat_id,
        "没有可返回的目标选择",
        client_id,
    )
    .await?
    else {
        return Ok(());
    };
    ctx.answer("已返回目标选择").await?;
    edit_target_choice_prompt(
        &config,
        TargetPromptContext {
            app,
            request_chat_id: ctx.chat_id,
            sender_user_id: ctx.sender_user_id,
            client_id: ctx.client_id,
        },
        ctx.message_id,
        context.kind,
        &context.source_link,
    )
    .await?;
    Ok(())
}

/// 从 callback 原子推进目标上下文。
pub(super) async fn advance_target_context_for_callback(
    key: DraftKey,
    advance: TargetDraftAdvance,
    callback_query_id: i64,
    chat_id: i64,
    missing_tip: &'static str,
    client_id: i32,
) -> anyhow::Result<Option<TargetContext>> {
    match target_advance_callback_decision(advance_target_context(key, advance).await?, missing_tip)
    {
        TargetAdvanceCallbackDecision::Continue(context) => Ok(Some(context)),
        TargetAdvanceCallbackDecision::Recover {
            callback_tip,
            title,
            status,
            detail,
        } => {
            send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
            send_input_recovery_card(chat_id, client_id, title, status, detail).await?;
            Ok(None)
        }
        TargetAdvanceCallbackDecision::WaitForSource { callback_tip } => {
            send::answer_callback_query(callback_query_id, Some(callback_tip), client_id).await?;
            Ok(None)
        }
    }
}

/// 输入流程无法继续时给出可点击恢复入口。
///
/// 只弹 callback 提示容易被 Telegram 客户端很快收起；额外发一张短卡片能让用户明确知道下一步。
pub(super) async fn send_input_recovery_card(
    chat_id: i64,
    client_id: i32,
    title: &str,
    status: &str,
    detail: &str,
) -> anyhow::Result<()> {
    send::ReplyPanel::card(build_menu_recovery_text(title, status, detail))
        .row(vec![send::build_callback_button(
            "重新打开菜单",
            &super::super::callback::menu_page_callback_data(
                super::super::callback::MenuPage::Home,
            ),
            tdlib_rs::enums::ButtonStyle::Primary,
        )])
        .send(chat_id, client_id)
        .await
}

/// 把旧目标选择卡片改成等待状态。
///
/// ForceReply / reply keyboard 需要单独消息承载；旧 inline 卡片如果继续保留所有目标按钮，
/// 用户容易重复点击造成流程跳转。因此这里原地收敛为“等待 + 取消”。
pub(super) async fn edit_input_waiting_card(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    step: &str,
    title: &str,
    detail: &str,
) {
    edit_input_waiting_card_with_navigation(
        chat_id, message_id, client_id, step, title, detail, None, false,
    )
    .await;
}

/// 把手动目标输入卡片改成等待状态，并保留返回目标选择的入口。
async fn edit_target_input_waiting_card(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    step: &str,
    title: &str,
    detail: &str,
    source_link: &str,
) {
    edit_input_waiting_card_with_navigation(
        chat_id,
        message_id,
        client_id,
        step,
        title,
        detail,
        Some(source_link),
        true,
    )
    .await;
}

/// 编辑等待输入卡片，并按当前流程提供必要的返回动作。
#[allow(clippy::too_many_arguments)]
async fn edit_input_waiting_card_with_navigation(
    chat_id: i64,
    message_id: i64,
    client_id: i32,
    step: &str,
    title: &str,
    detail: &str,
    source_link: Option<&str>,
    can_return_to_target_choice: bool,
) {
    let prompt_text =
        build_step_prompt_with_context("waiting-input", step, title, detail, source_link, None);
    let Ok((text, keyboard)) = send::ReplyPanel::card(prompt_text)
        .rows(build_input_waiting_button_rows(can_return_to_target_choice))
        .into_card_parts()
    else {
        tracing::warn!(chat_id, message_id, "build waiting input card failed");
        return;
    };

    if let Err(err) =
        send::edit_card_message_with_inline_keyboard(text, chat_id, message_id, keyboard, client_id)
            .await
    {
        tracing::warn!(
            chat_id,
            message_id,
            error = %err,
            "edit waiting input card failed"
        );
    }
}

/// 等待输入卡片的按钮布局。
fn build_input_waiting_button_rows(
    can_return_to_target_choice: bool,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut row = Vec::new();
    if can_return_to_target_choice {
        row.push(send::build_callback_button(
            "返回目标选择",
            &super::super::callback::target_back_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ));
    }
    row.push(send::build_callback_button(
        "取消",
        &super::super::callback::cancel_input_callback_data(),
        tdlib_rs::enums::ButtonStyle::Danger,
    ));
    vec![row]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RequestActor;
    use crate::tgbot::transfer::command::menu::input::MenuJobAction;

    async fn prepare_schema() -> anyhow::Result<tokio::sync::MutexGuard<'static, ()>> {
        let guard = crate::db::TEST_DB_LOCK.lock().await;
        let db = crate::db::get_db().await?;
        crate::db::ensure_test_schema_current(db).await?;
        Ok(guard)
    }

    // 没有确认草稿时，执行按钮应走“empty”分支而不是 panic。
    #[tokio::test]
    async fn test_take_confirm_context_without_draft_returns_none() -> anyhow::Result<()> {
        let _guard = prepare_schema().await?;
        let key = (990_003, 990_004);
        let result = take_confirm_context(key).await?;

        assert!(matches!(result, ConfirmContextTakeResult::None));
        Ok(())
    }

    // 单步任务动作仍应保持公开长动作名称，不因 callback 模块继续分裂语义。
    #[test]
    fn test_menu_job_action_public_names_stay_stable() {
        assert_eq!(MenuJobAction::Status.command_action(), "status");
        assert_eq!(MenuJobAction::Pause.command_action(), "pause");
        assert_eq!(MenuJobAction::Resume.command_action(), "resume");
        assert_eq!(MenuJobAction::Stop.command_action(), "stop");
    }

    // 目标确认执行路径复用的 actor 必须仍是请求者本人，避免后续测试误用默认值。
    #[test]
    fn test_confirm_path_actor_fixture_is_explicit() {
        let actor = RequestActor {
            request_chat_id: 100,
            user_id: 200,
        };

        assert_eq!(actor.request_chat_id, 100);
        assert_eq!(actor.user_id, 200);
    }

    // 目标 callback 上下文必须用 chat + user 隔离草稿，不能把 message_id 混进主键。
    #[test]
    fn test_target_callback_context_draft_key() {
        let ctx = TargetCallbackContext {
            callback_query_id: 1,
            chat_id: 10,
            message_id: 20,
            sender_user_id: 30,
            client_id: 40,
        };

        assert_eq!(ctx.draft_key(), (10, 30));
    }

    // 默认目标回落当前私聊时，callback 提示应与按钮文案一致。
    #[test]
    fn test_default_target_selected_tip_describes_actual_target() {
        assert_eq!(default_target_selected_tip(100, 100), "已选择当前私聊");
        assert_eq!(default_target_selected_tip(-100, 100), "已选择默认目标");
    }

    // 确认按钮的状态结果应映射为稳定 UI 动作，入口函数只负责执行副作用。
    #[test]
    fn test_confirm_callback_decision_maps_state_results() {
        let active = confirm_callback_decision(ConfirmContextTakeResult::Active(
            super::super::state::ConfirmContext {
                kind: crate::tgbot::transfer::command::menu::input::MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
                target_chat_id: -100,
            },
        ));
        assert!(matches!(active, ConfirmCallbackDecision::Run(_)));

        assert!(matches!(
            confirm_callback_decision(ConfirmContextTakeResult::Expired),
            ConfirmCallbackDecision::Recover {
                callback_tip: "输入已过期",
                status: "expired",
                ..
            }
        ));
        assert!(matches!(
            confirm_callback_decision(ConfirmContextTakeResult::None),
            ConfirmCallbackDecision::Recover {
                callback_tip: "没有待执行的输入",
                status: "empty",
                ..
            }
        ));
        assert_eq!(
            confirm_callback_decision(ConfirmContextTakeResult::WrongStep),
            ConfirmCallbackDecision::WaitForTarget {
                callback_tip: "请先选择目标"
            }
        );
    }

    // 目标推进按钮也应先规整状态层返回，避免过期/空草稿/错阶段在入口各写一遍。
    #[test]
    fn test_target_advance_callback_decision_maps_state_results() {
        let active = target_advance_callback_decision(
            TargetContextAdvanceResult::Active(TargetContext {
                kind: crate::tgbot::transfer::command::menu::input::MenuInputKind::Transfer,
                source_link: "https://t.me/c/1/2".to_owned(),
            }),
            "没有等待选择目标的输入",
        );
        assert!(matches!(
            active,
            TargetAdvanceCallbackDecision::Continue(TargetContext {
                kind: crate::tgbot::transfer::command::menu::input::MenuInputKind::Transfer,
                ..
            })
        ));

        assert!(matches!(
            target_advance_callback_decision(
                TargetContextAdvanceResult::Expired,
                "没有等待选择目标的输入"
            ),
            TargetAdvanceCallbackDecision::Recover {
                callback_tip: "输入已过期",
                status: "expired",
                ..
            }
        ));
        assert!(matches!(
            target_advance_callback_decision(
                TargetContextAdvanceResult::None,
                "没有等待选择目标的输入"
            ),
            TargetAdvanceCallbackDecision::Recover {
                callback_tip: "没有等待选择目标的输入",
                status: "empty",
                ..
            }
        ));
        assert_eq!(
            target_advance_callback_decision(
                TargetContextAdvanceResult::WrongStep,
                "没有等待选择目标的输入"
            ),
            TargetAdvanceCallbackDecision::WaitForSource {
                callback_tip: "请先发送源链接"
            }
        );
    }

    // 手动目标输入等待态应允许返回目标选择，其他输入等待态仍只提供取消。
    #[test]
    fn test_input_waiting_button_rows_support_target_back() {
        use base64::{Engine as _, engine::general_purpose};

        let target_rows = build_input_waiting_button_rows(true);
        assert_eq!(target_rows.len(), 1);
        assert_eq!(target_rows[0][0].text, "返回目标选择");
        assert_eq!(target_rows[0][1].text, "取消");

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) =
            &target_rows[0][0].r#type
        else {
            panic!("target back must be callback");
        };
        let decoded = String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap())
            .expect("callback should be utf8");
        assert_eq!(decoded, "m:tb");

        let default_rows = build_input_waiting_button_rows(false);
        assert_eq!(default_rows.len(), 1);
        assert_eq!(default_rows[0].len(), 1);
        assert_eq!(default_rows[0][0].text, "取消");
    }
}
