// 积分流水 callback 入口。
// 这里只处理按钮回调，渲染和分页按钮拼装交给 `render.rs`。

use crate::tgbot::send;
use crate::tgbot::send::send_interaction_error_card;
use crate::tgbot::transfer::command::menu::AdminInputAction;

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

    if let Some((action, user_id)) = parse_points_adjust_callback_data(&payload) {
        crate::tgbot::transfer::command::menu::start_points_adjust_input_callback(
            update.id,
            update.chat_id,
            update.message_id,
            update.sender_user_id,
            action,
            user_id,
            actor,
            client_id,
        )
        .await?;
        return Ok(());
    }

    let Some((action, request)) = super::parse_points_callback_data(&payload) else {
        send::answer_callback_query(update.id, Some("积分流水参数无效"), client_id).await?;
        return Ok(());
    };
    let effective_user_id =
        if matches!(request.kind, super::LedgerCommandKind::Balance) && request.user_id == 0 {
            actor.user_id
        } else {
            request.user_id
        };
    let allowed = match request.kind {
        super::LedgerCommandKind::Balance => actor.is_admin() || effective_user_id == actor.user_id,
        super::LedgerCommandKind::Points => actor.is_admin(),
    };
    if !allowed {
        send::answer_callback_query(update.id, Some("没有权限查看该用户流水"), client_id).await?;
        return Ok(());
    }

    let callback_tip = match action {
        super::LedgerCallbackAction::Refresh => Some("已刷新"),
        super::LedgerCallbackAction::BalanceHome => Some("已返回余额"),
        super::LedgerCallbackAction::Page => None,
    };
    send::answer_callback_query(update.id, callback_tip, client_id).await?;

    let rendered = match action {
        super::LedgerCallbackAction::BalanceHome => {
            let account =
                match crate::tgbot::transfer::store::get_user_account(effective_user_id).await {
                    Ok(Some(account)) => account,
                    Ok(None) => {
                        let err = anyhow::anyhow!("user account not found: {}", effective_user_id);
                        send_points_callback_error(update.chat_id, client_id, &err).await?;
                        return Err(err);
                    }
                    Err(err) => {
                        send_points_callback_error(update.chat_id, client_id, &err).await?;
                        return Err(err);
                    }
                };
            let mut panel = send::ReplyPanel::card(super::format_balance_text(&account));
            if matches!(request.kind, super::LedgerCommandKind::Points) {
                panel = panel
                    .row(vec![
                        send::build_callback_button(
                            "查看流水",
                            &super::build_points_history_home_callback_data(
                                effective_user_id,
                                10,
                                1,
                            ),
                            tdlib_rs::enums::ButtonStyle::Primary,
                        ),
                        send::build_callback_button(
                            "菜单",
                            &super::build_menu_home_button_data(),
                            tdlib_rs::enums::ButtonStyle::Default,
                        ),
                    ])
                    .row(vec![
                        send::build_callback_button(
                            "加分",
                            &super::build_points_adjust_home_callback_data(effective_user_id, true),
                            tdlib_rs::enums::ButtonStyle::Default,
                        ),
                        send::build_callback_button(
                            "扣分",
                            &super::build_points_adjust_home_callback_data(effective_user_id, false),
                            tdlib_rs::enums::ButtonStyle::Default,
                        ),
                    ]);
            } else {
                panel = panel
                    .row(vec![
                        send::build_callback_button(
                            "查看流水",
                            &super::build_balance_history_home_callback_data(10, 1),
                            tdlib_rs::enums::ButtonStyle::Primary,
                        ),
                        send::build_callback_button(
                            "菜单",
                            &super::build_menu_home_button_data(),
                            tdlib_rs::enums::ButtonStyle::Default,
                        ),
                    ])
                    .row(vec![send::build_callback_button(
                        "帮助",
                        &crate::tgbot::transfer::command::help::build_help_callback_data(Some("points")),
                        tdlib_rs::enums::ButtonStyle::Default,
                    )]);
            }
            Ok(panel)
        }
        _ => {
            render_ledger_panel(
                request.kind,
                effective_user_id,
                request.limit,
                request.page,
                matches!(request.kind, super::LedgerCommandKind::Points),
            )
            .await
        }
    };
    let rendered = match rendered {
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

fn parse_points_adjust_callback_data(data: &str) -> Option<(AdminInputAction, i64)> {
    let payload = data.strip_prefix("pta:")?;
    let mut parts = payload.split(':');
    let action = match parts.next()? {
        "add" => AdminInputAction::PointsAddUser,
        "sub" => AdminInputAction::PointsSubUser,
        _ => return None,
    };
    let user_id = parts.next()?.parse::<i64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((action, user_id))
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

#[cfg(test)]
mod tests {
    use super::parse_points_adjust_callback_data;
    use crate::tgbot::transfer::command::menu::AdminInputAction;

    #[test]
    fn test_parse_points_adjust_callback_data() {
        assert_eq!(
            parse_points_adjust_callback_data("pta:add:42"),
            Some((AdminInputAction::PointsAddUser, 42))
        );
        assert_eq!(
            parse_points_adjust_callback_data("pta:sub:42"),
            Some((AdminInputAction::PointsSubUser, 42))
        );
        assert_eq!(parse_points_adjust_callback_data("pta:bad:42"), None);
        assert_eq!(parse_points_adjust_callback_data("pt:add:42"), None);
    }
}
