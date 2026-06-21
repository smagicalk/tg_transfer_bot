// `/menu` 目标选择和确认卡片。
// 这个 module 把“目标如何展示/验证”集中起来，输入 handler 只负责推进流程。

use std::collections::HashSet;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::super::common::resolve_target_chat_id_on;
use super::super::callback;
use super::super::text::{
    build_confirm_command_preview, build_menu_context_lines, build_menu_step_state_line,
    build_menu_target_step_state_line,
};
use super::state::{MenuInputKind, last_target};

/// 目标选择页发送上下文。
///
/// 这类页面总是同时需要聊天、用户、客户端和运行时上下文，收拢成一个小结构可以避免函数参数继续膨胀。
#[derive(Clone, Copy)]
pub(super) struct TargetPromptContext<'a> {
    pub(super) app: &'a crate::app_context::AppContext,
    pub(super) request_chat_id: i64,
    pub(super) sender_user_id: i64,
    pub(super) client_id: i32,
}

/// 发送目标选择卡片。
pub(super) async fn send_target_choice_prompt(
    config: &BotConfig,
    ctx: TargetPromptContext<'_>,
    kind: MenuInputKind,
    source_link: &str,
) -> anyhow::Result<()> {
    send::ReplyPanel::card(build_target_choice_text(kind, source_link))
        .rows(build_target_choice_buttons_on(
            ctx.app,
            config,
            ctx.request_chat_id,
            ctx.sender_user_id,
            kind,
        ))
        .send(ctx.request_chat_id, ctx.client_id)
        .await
}

/// 发送带提示说明的目标选择卡片。
pub(super) async fn send_target_choice_prompt_with_detail(
    config: &BotConfig,
    ctx: TargetPromptContext<'_>,
    kind: MenuInputKind,
    source_link: &str,
    detail: &str,
) -> anyhow::Result<()> {
    send::ReplyPanel::card(build_target_choice_text_with_detail(
        kind,
        source_link,
        detail,
    ))
    .rows(build_target_choice_buttons_on(
        ctx.app,
        config,
        ctx.request_chat_id,
        ctx.sender_user_id,
        kind,
    ))
    .send(ctx.request_chat_id, ctx.client_id)
    .await
}

/// 编辑当前消息为目标选择卡片。
pub(super) async fn edit_target_choice_prompt(
    config: &BotConfig,
    ctx: TargetPromptContext<'_>,
    message_id: i64,
    kind: MenuInputKind,
    source_link: &str,
) -> anyhow::Result<()> {
    let (text, keyboard) = send::ReplyPanel::card(build_target_choice_text(kind, source_link))
        .rows(build_target_choice_buttons_on(
            ctx.app,
            config,
            ctx.request_chat_id,
            ctx.sender_user_id,
            kind,
        ))
        .into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        ctx.request_chat_id,
        message_id,
        keyboard,
        ctx.client_id,
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

/// 在指定上下文上构造目标选择按钮。
pub(super) fn build_target_choice_buttons_on(
    app: &crate::app_context::AppContext,
    config: &BotConfig,
    request_chat_id: i64,
    sender_user_id: i64,
    kind: MenuInputKind,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let _ = config;
    let targets_runtime = crate::tgbot::transfer::targets_runtime_config_on(app);
    let mut rows = Vec::new();
    let mut seen_targets = HashSet::new();

    if let Some(target_chat_id) = last_target(request_chat_id, sender_user_id)
        && resolve_target_by_id_on(app, target_chat_id, config, request_chat_id).is_ok()
    {
        seen_targets.insert(target_chat_id);
        rows.push(vec![send::build_callback_button(
            "上次目标",
            &callback::target_alias_callback_data(target_chat_id),
            tdlib_rs::enums::ButtonStyle::Primary,
        )]);
    }

    if let Some(default_target_chat_id) = resolve_default_target_on(app, config, request_chat_id)
        && seen_targets.insert(default_target_chat_id)
    {
        rows.push(vec![send::build_callback_button(
            default_target_button_label(kind),
            &callback::target_default_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        )]);
    }

    let mut alias_buttons = targets_runtime
        .aliases
        .iter()
        .filter_map(|(alias, chat_id)| {
            if resolve_target_by_id_on(app, *chat_id, config, request_chat_id).is_err() {
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

/// 在指定上下文上解析用户输入的目标。
pub(super) fn resolve_target_input_on(
    app: &crate::app_context::AppContext,
    input: &str,
    config: &BotConfig,
    request_chat_id: i64,
) -> Option<i64> {
    if input.eq_ignore_ascii_case("default") {
        return resolve_default_target_on(app, config, request_chat_id);
    }
    resolve_target_chat_id_on(app, &["/menu-input", "placeholder", input], request_chat_id).ok()
}

/// 在指定上下文上用数字 chat_id 走同一套目标白名单校验。
pub(super) fn resolve_target_by_id_on(
    app: &crate::app_context::AppContext,
    target_chat_id: i64,
    _config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    let target = target_chat_id.to_string();
    resolve_target_chat_id_on(
        app,
        &["/menu-input", "placeholder", &target],
        request_chat_id,
    )
}

/// 在指定上下文上解析菜单“快速转存/查询”使用的默认目标。
pub(super) fn resolve_default_target_on(
    app: &crate::app_context::AppContext,
    _config: &BotConfig,
    request_chat_id: i64,
) -> Option<i64> {
    resolve_target_chat_id_on(app, &["/menu-input", "placeholder"], request_chat_id).ok()
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
    use crate::app_context::app_context;
    use std::sync::{LazyLock, Mutex, MutexGuard};

    static TARGET_RUNTIME_TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    fn lock_target_runtime_tests() -> MutexGuard<'static, ()> {
        match TARGET_RUNTIME_TEST_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn test_app_context() -> std::sync::Arc<crate::app_context::AppContext> {
        app_context()
    }

    fn install_target_runtime(
        targets: crate::config::TargetsConfig,
        access_control: crate::config::AccessControlConfig,
    ) {
        super::super::state::clear_last_targets();
        let app = test_app_context();
        app.targets_runtime.update_runtime_config(targets);
        app.access_control_runtime
            .update_runtime_config(access_control);
    }

    fn resolve_default_target_for_test(config: &BotConfig, request_chat_id: i64) -> Option<i64> {
        let app = test_app_context();
        resolve_default_target_on(app.as_ref(), config, request_chat_id)
    }

    fn test_build_target_choice_buttons(
        config: &BotConfig,
        request_chat_id: i64,
        sender_user_id: i64,
        kind: MenuInputKind,
    ) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
        let app = test_app_context();
        build_target_choice_buttons_on(app.as_ref(), config, request_chat_id, sender_user_id, kind)
    }

    // 快速转存应优先使用当前请求 chat 的默认目标，再使用全局兜底目标。
    #[test]
    fn test_resolve_default_target() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(
            crate::config::TargetsConfig::default(),
            crate::config::AccessControlConfig::default(),
        );
        assert_eq!(resolve_default_target_for_test(&config, 1), Some(1));

        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: -100,
                by_request_chat_id: Default::default(),
                aliases: Default::default(),
            },
            crate::config::AccessControlConfig::default(),
        );
        assert_eq!(resolve_default_target_for_test(&config, 1), Some(-100));

        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: -100,
                by_request_chat_id: std::collections::HashMap::from([(1, -200)]),
                aliases: Default::default(),
            },
            crate::config::AccessControlConfig::default(),
        );
        assert_eq!(resolve_default_target_for_test(&config, 1), Some(-200));
    }

    // 快速转存的默认目标也必须遵守 allowed_target_chat_ids。
    #[test]
    fn test_resolve_default_target_respects_allowed_targets() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: -200,
                by_request_chat_id: Default::default(),
                aliases: Default::default(),
            },
            crate::config::AccessControlConfig {
                allowed_target_chat_ids: vec![-100],
                ..Default::default()
            },
        );

        assert_eq!(resolve_default_target_for_test(&config, 1), None);
    }

    // 目标选择页应优先提供快速目标、常用目标、Telegram 原生选群和手动输入。
    #[test]
    fn test_build_target_choice_buttons_layout() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: -100,
                by_request_chat_id: Default::default(),
                aliases: std::collections::HashMap::from([("archive".to_owned(), -200)]),
            },
            crate::config::AccessControlConfig {
                allowed_target_chat_ids: vec![-100, -200],
                ..Default::default()
            },
        );

        let rows = test_build_target_choice_buttons(&config, 61001, 62001, MenuInputKind::Transfer);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "快速转存");
        assert!(labels.contains(&"archive"));
        assert!(labels.contains(&"选择群组"));
        assert!(labels.contains(&"手动输入"));
        assert_eq!(rows.last().expect("should have cancel row")[0].text, "取消");
    }

    // 查询流程里的默认目标按钮应显示“快速查询”，避免和转存动作混淆。
    #[test]
    fn test_build_target_choice_buttons_lookup_label() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: -100,
                by_request_chat_id: Default::default(),
                aliases: Default::default(),
            },
            crate::config::AccessControlConfig {
                allowed_target_chat_ids: vec![-100],
                ..Default::default()
            },
        );

        let rows = test_build_target_choice_buttons(&config, 61002, 62002, MenuInputKind::Lookup);

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
        let _guard = lock_target_runtime_tests();
        install_target_runtime(
            crate::config::TargetsConfig::default(),
            crate::config::AccessControlConfig::default(),
        );
        let config = BotConfig::default();
        install_target_runtime(
            crate::config::TargetsConfig {
                default_chat_id: -100,
                by_request_chat_id: Default::default(),
                aliases: Default::default(),
            },
            crate::config::AccessControlConfig {
                allowed_target_chat_ids: vec![-100],
                ..Default::default()
            },
        );
        super::super::state::remember_last_target(101, 202, -100);

        let rows = test_build_target_choice_buttons(&config, 101, 202, MenuInputKind::Transfer);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"上次目标"));
        assert_eq!(
            labels.iter().filter(|label| **label == "快速转存").count(),
            0
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
