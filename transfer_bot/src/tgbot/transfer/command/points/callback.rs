// 积分流水 callback 入口。
// 这里只处理按钮回调，渲染和分页按钮拼装交给 `render.rs`。

use crate::tgbot::send;
use crate::tgbot::send::send_interaction_error_card;

use super::render::render_ledger_panel;

/// 积分流水 callback 入口。
pub(in crate::tgbot::transfer::command) async fn points_callback_query(
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    actor: crate::config::RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let payload = match update.payload {
        tdlib_rs::enums::CallbackQueryPayload::Data(data) => data.data,
        _ => {
            send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
            return Ok(());
        }
    };

    let Some((action, request)) = super::parse_points_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("积分流水参数无效"), client_id).await?;
        return Ok(());
    };
    let allowed = match request.kind {
        super::LedgerCommandKind::Balance => actor.is_admin() || request.user_id == actor.user_id,
        super::LedgerCommandKind::Points => actor.is_admin(),
    };
    if !allowed {
        send::answer_callback_query(update.id, Some("没有权限查看该用户流水"), client_id).await?;
        return Ok(());
    }

    let callback_tip = match action {
        super::LedgerCallbackAction::Refresh => Some("已刷新"),
        super::LedgerCallbackAction::Page => None,
    };
    send::answer_callback_query(update.id, callback_tip, client_id).await?;

    let rendered = match render_ledger_panel(
        request.kind,
        request.user_id,
        request.limit,
        request.page,
        matches!(request.kind, super::LedgerCommandKind::Points),
    )
    .await
    {
        Ok(panel) => panel,
        Err(err) => {
            send_points_callback_error(update.chat_id, client_id, &err).await?;
            return Err(err);
        }
    };
    let (text, keyboard) = rendered.into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        update.chat_id,
        update.message_id,
        keyboard,
        client_id,
        "积分流水刷新失败",
        "流水页已生成，但原消息编辑失败；请复制错误或重新发送流水命令。",
    )
    .await
}

/// 积分流水 callback 失败提示。
async fn send_points_callback_error(
    request_chat_id: i64,
    client_id: i32,
    err: &anyhow::Error,
) -> anyhow::Result<()> {
    send_interaction_error_card(
        request_chat_id,
        client_id,
        "积分流水刷新失败",
        "流水页未刷新，请检查日志或复制错误信息。",
        err,
    )
    .await
}
