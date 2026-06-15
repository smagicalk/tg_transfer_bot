// `/menu` 目标选择和确认卡片。
// 这个 module 把“目标如何展示/验证”集中起来，输入 handler 只负责推进流程。

use std::collections::HashSet;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::super::common::resolve_target_chat_id;
use super::super::callback;
use super::super::text::{
    build_confirm_command_preview, build_menu_context_lines, build_menu_step_state_line,
    build_menu_target_step_state_line,
};
use super::state::{MenuInputKind, last_target};

/// 发送目标选择卡片。
pub(super) async fn send_target_choice_prompt(
    config: &BotConfig,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
    kind: MenuInputKind,
    source_link: &str,
) -> anyhow::Result<()> {
    send::ReplyPanel::card(build_target_choice_text(kind, source_link))
        .rows(build_target_choice_buttons(
            config,
            request_chat_id,
            sender_user_id,
            kind,
        ))
        .send(request_chat_id, client_id)
        .await
}

/// 发送带提示说明的目标选择卡片。
pub(super) async fn send_target_choice_prompt_with_detail(
    config: &BotConfig,
    request_chat_id: i64,
    sender_user_id: i64,
    client_id: i32,
    kind: MenuInputKind,
    source_link: &str,
    detail: &str,
) -> anyhow::Result<()> {
    send::ReplyPanel::card(build_target_choice_text_with_detail(
        kind,
        source_link,
        detail,
    ))
    .rows(build_target_choice_buttons(
        config,
        request_chat_id,
        sender_user_id,
        kind,
    ))
    .send(request_chat_id, client_id)
    .await
}

/// 编辑当前消息为目标选择卡片。
pub(super) async fn edit_target_choice_prompt(
    config: &BotConfig,
    request_chat_id: i64,
    sender_user_id: i64,
    message_id: i64,
    client_id: i32,
    kind: MenuInputKind,
    source_link: &str,
) -> anyhow::Result<()> {
    let (text, keyboard) = send::ReplyPanel::card(build_target_choice_text(kind, source_link))
        .rows(build_target_choice_buttons(
            config,
            request_chat_id,
            sender_user_id,
            kind,
        ))
        .into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        request_chat_id,
        message_id,
        keyboard,
        client_id,
        "目标选择刷新失败",
        "目标选择页已生成，但原消息编辑失败；请复制错误或重新打开 /menu。",
    )
    .await
}

/// 发送确认卡片。
pub(super) async fn send_confirm_prompt(
    kind: MenuInputKind,
    source_link: &str,
    target_chat_id: i64,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    send::ReplyPanel::card(build_confirm_text(kind, source_link, target_chat_id))
        .rows(confirm_button_rows())
        .send(request_chat_id, client_id)
        .await
}

/// 编辑当前消息为确认卡片。
pub(super) async fn edit_confirm_prompt(
    kind: MenuInputKind,
    source_link: &str,
    target_chat_id: i64,
    request_chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, keyboard) =
        send::ReplyPanel::card(build_confirm_text(kind, source_link, target_chat_id))
            .rows(confirm_button_rows())
            .into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        request_chat_id,
        message_id,
        keyboard,
        client_id,
        "确认页刷新失败",
        "确认页已生成，但原消息编辑失败；请复制错误或重新打开 /menu。",
    )
    .await
}

/// 目标选择按钮。
///
/// 交互优先级：
/// 1. 上次目标、快速目标和常用别名，减少重复选择。
/// 2. Telegram 原生选群，适合临时目标。
/// 3. 手动输入，作为兜底。
/// 4. 取消，明确退出流程。
pub(super) fn build_target_choice_buttons(
    config: &BotConfig,
    request_chat_id: i64,
    sender_user_id: i64,
    kind: MenuInputKind,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = Vec::new();
    let mut seen_targets = HashSet::new();

    if let Some(target_chat_id) = last_target(request_chat_id, sender_user_id)
        && resolve_target_by_id(target_chat_id, config, request_chat_id).is_ok()
    {
        seen_targets.insert(target_chat_id);
        rows.push(vec![send::build_callback_button(
            "上次目标",
            &callback::target_alias_callback_data(target_chat_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        )]);
    }

    if let Some(default_target_chat_id) = resolve_default_target(config, request_chat_id)
        && seen_targets.insert(default_target_chat_id)
    {
        rows.push(vec![send::build_callback_button(
            default_target_button_label(kind),
            &callback::target_default_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        )]);
    }

    let mut alias_buttons = config
        .target_aliases
        .iter()
        .filter_map(|(alias, chat_id)| {
            if resolve_target_by_id(*chat_id, config, request_chat_id).is_err() {
                return None;
            }
            if !seen_targets.insert(*chat_id) {
                return None;
            }
            Some(send::build_callback_button(
                alias,
                &callback::target_alias_callback_data(*chat_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ))
        })
        .collect::<Vec<_>>();
    alias_buttons.sort_by(|left, right| left.text.cmp(&right.text));
    rows.extend(alias_buttons.chunks(2).map(<[_]>::to_vec));

    rows.push(vec![
        send::build_callback_button(
            "选择群组",
            &callback::target_request_chat_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
        send::build_callback_button(
            "手动输入",
            &callback::target_manual_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]);
    rows.push(vec![send::build_callback_button(
        "取消",
        &callback::cancel_input_callback_data(),
        tdlib_rs::enums::ButtonStyle::Danger,
    )]);
    rows
}

/// 确认页按钮。
pub(super) fn confirm_button_rows() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        vec![send::build_callback_button(
            "执行",
            &callback::target_confirm_callback_data(),
            tdlib_rs::enums::ButtonStyle::Success,
        )],
        vec![
            send::build_callback_button(
                "重选目标",
                &callback::target_back_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
            send::build_callback_button(
                "取消",
                &callback::cancel_input_callback_data(),
                tdlib_rs::enums::ButtonStyle::Danger,
            ),
        ],
    ]
}

/// 解析用户输入的目标。
pub(super) fn resolve_target_input(
    input: &str,
    config: &BotConfig,
    request_chat_id: i64,
) -> Option<i64> {
    if input.eq_ignore_ascii_case("default") {
        return resolve_default_target(config, request_chat_id);
    }
    resolve_target_chat_id(
        &["/menu-input", "placeholder", input],
        config,
        request_chat_id,
    )
    .ok()
}

/// 用数字 chat_id 走同一套目标白名单校验。
pub(super) fn resolve_target_by_id(
    target_chat_id: i64,
    config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    let target = target_chat_id.to_string();
    resolve_target_chat_id(
        &["/menu-input", "placeholder", &target],
        config,
        request_chat_id,
    )
}

/// 解析菜单“快速转存/查询”使用的默认目标。
///
/// 这里提前解析是为了在缺少默认目标时继续引导输入目标，而不是让复用的命令入口直接报错。
pub(super) fn resolve_default_target(config: &BotConfig, request_chat_id: i64) -> Option<i64> {
    resolve_target_chat_id(&["/menu-input", "placeholder"], config, request_chat_id).ok()
}

/// 目标选择卡片正文。
fn build_target_choice_text(kind: MenuInputKind, source_link: &str) -> String {
    build_target_choice_text_lines(kind, source_link, None).join("\n")
}

/// 目标选择卡片正文，附带一条额外说明。
fn build_target_choice_text_with_detail(
    kind: MenuInputKind,
    source_link: &str,
    detail: &str,
) -> String {
    build_target_choice_text_lines(kind, source_link, Some(detail)).join("\n")
}

/// 构造目标选择卡片的正文行。
fn build_target_choice_text_lines(
    kind: MenuInputKind,
    source_link: &str,
    detail: Option<&str>,
) -> Vec<String> {
    let mut lines = vec![
        kind.target_choice_title().to_owned(),
        build_menu_step_state_line("waiting-target", "2/3"),
        crate::tgbot::transfer::card::DIVIDER.to_owned(),
    ];
    lines.extend(build_menu_context_lines(Some(source_link), None));
    if let Some(detail) = detail {
        lines.push(crate::tgbot::transfer::card::note(detail));
    }
    lines.extend([
        crate::tgbot::transfer::card::section("目标方式"),
        "可以点常用目标、使用 Telegram 原生选群，或手动输入 chat_id/alias。".to_owned(),
        format!("取消：{}", crate::tgbot::transfer::card::code("/cancel")),
    ]);
    lines
}

/// 确认卡片正文。
fn build_confirm_text(kind: MenuInputKind, source_link: &str, target_chat_id: i64) -> String {
    let mut lines = vec![
        kind.confirm_title().to_owned(),
        build_menu_target_step_state_line("waiting-confirm", target_chat_id, "3/3"),
        crate::tgbot::transfer::card::DIVIDER.to_owned(),
    ];
    lines.extend(build_menu_context_lines(
        Some(source_link),
        Some(target_chat_id),
    ));
    lines.extend([
        crate::tgbot::transfer::card::section("命令预览"),
        crate::tgbot::transfer::card::code(build_confirm_command_preview(
            kind,
            source_link,
            target_chat_id,
        )),
        crate::tgbot::transfer::card::section("下一步"),
        "确认无误后点击“执行”；如果目标不对，返回重新选择。".to_owned(),
        format!("取消：{}", crate::tgbot::transfer::card::code("/cancel")),
    ]);
    lines.join("\n")
}

/// 默认目标按钮文案。
///
/// 同一个默认目标在转存和查询里语义不同，所以按钮文案按场景分别显示。
fn default_target_button_label(kind: MenuInputKind) -> &'static str {
    kind.default_target_button_label()
}

#[cfg(test)]
mod tests {
    use super::*;

    // 快速转存应优先使用当前请求 chat 的默认目标，再使用全局兜底目标。
    #[test]
    fn test_resolve_default_target() {
        let mut config = BotConfig::default();
        assert_eq!(resolve_default_target(&config, 1), None);

        config.target_map.insert(0, -100);
        assert_eq!(resolve_default_target(&config, 1), Some(-100));

        config.target_map.insert(1, -200);
        assert_eq!(resolve_default_target(&config, 1), Some(-200));
    }

    // 快速转存的默认目标也必须遵守 allowed_target_chat_ids。
    #[test]
    fn test_resolve_default_target_respects_allowed_targets() {
        let mut config = BotConfig::default();
        config.target_map.insert(0, -200);
        config.allowed_target_chat_ids = vec![-100];

        assert_eq!(resolve_default_target(&config, 1), None);
    }

    // 目标选择页应优先提供快速目标、常用目标、Telegram 原生选群和手动输入。
    #[test]
    fn test_build_target_choice_buttons_layout() {
        let mut config = BotConfig::default();
        config.target_map.insert(0, -100);
        config.allowed_target_chat_ids = vec![-100, -200];
        config.target_aliases.insert("archive".to_owned(), -200);

        let rows = build_target_choice_buttons(&config, 1, 2, MenuInputKind::Transfer);

        assert_eq!(rows[0][0].text, "快速转存");
        assert_eq!(rows[1][0].text, "archive");
        assert_eq!(rows[2][0].text, "选择群组");
        assert_eq!(rows[2][1].text, "手动输入");
        assert_eq!(rows[3][0].text, "取消");
    }

    // 查询流程里的默认目标按钮应显示“快速查询”，避免和转存动作混淆。
    #[test]
    fn test_build_target_choice_buttons_lookup_label() {
        let mut config = BotConfig::default();
        config.target_map.insert(0, -100);
        config.allowed_target_chat_ids = vec![-100];

        let rows = build_target_choice_buttons(&config, 1, 2, MenuInputKind::Lookup);

        assert_eq!(rows[0][0].text, "快速查询");
    }

    // 快速入口仍应使用和实际命令一致的目标标题、确认标题和默认按钮文案。
    #[test]
    fn test_menu_input_kind_labels_do_not_panic_for_quick_entries() {
        assert_eq!(
            MenuInputKind::TransferDefault.target_choice_title(),
            "选择转存目标"
        );
        assert_eq!(MenuInputKind::LookupDefault.confirm_title(), "确认查询");
        assert_eq!(
            default_target_button_label(MenuInputKind::LookupDefault),
            "快速查询"
        );
    }

    // 已确认过的目标应作为上次目标优先展示，并避免和默认目标重复出现。
    #[test]
    fn test_build_target_choice_buttons_prefers_last_target() {
        super::super::state::remember_last_target(101, 202, -100);
        let mut config = BotConfig::default();
        config.target_map.insert(0, -100);
        config.allowed_target_chat_ids = vec![-100];

        let rows = build_target_choice_buttons(&config, 101, 202, MenuInputKind::Transfer);

        assert_eq!(rows[0][0].text, "上次目标");
        assert!(
            !rows
                .iter()
                .flatten()
                .any(|button| button.text == "快速转存")
        );
    }

    // 确认页第一行只放“执行”，降低误触取消或重选的概率。
    #[test]
    fn test_confirm_button_rows_layout() {
        let rows = confirm_button_rows();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[0][0].text, "执行");
        assert_eq!(rows[1][0].text, "重选目标");
        assert_eq!(rows[1][1].text, "取消");
    }

    // 目标选择卡片在回退或提示时应保留来源上下文，并额外显示说明。
    #[test]
    fn test_build_target_choice_text_with_detail_keeps_context() {
        let text = build_target_choice_text_with_detail(
            MenuInputKind::Transfer,
            "https://t.me/c/1/2",
            "当前没有默认目标，请选择其他目标。",
        );

        assert!(text.contains("选择转存目标"));
        assert!(text.contains("来源：‹https://t.me/c/1/2›"));
        assert!(text.contains("当前没有默认目标，请选择其他目标。"));
    }

    // 带提示的目标选择卡片也应保留“目标方式”与取消提示，避免回退时信息缩水。
    #[test]
    fn test_build_target_choice_text_with_detail_keeps_action_hints() {
        let text = build_target_choice_text_with_detail(
            MenuInputKind::Lookup,
            "https://t.me/c/1/2",
            "目标不可用，请重新选择。",
        );

        assert!(text.contains("■ 目标方式"));
        assert!(text.contains("手动输入"));
        assert!(text.contains("‹/cancel›"));
    }
}
