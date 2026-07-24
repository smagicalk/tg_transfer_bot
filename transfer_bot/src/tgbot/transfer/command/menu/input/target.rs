// `/menu` 目标选择和确认卡片。
// 这个 module 把“目标如何展示/验证”集中起来，输入 handler 只负责推进流程。

use std::collections::HashSet;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::super::common::resolve_target_chat_id_on;
use super::super::callback;
use super::super::text::{
    build_menu_context_lines, build_menu_step_state_line, build_menu_target_step_state_line,
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
        ))
        .into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        ctx.request_chat_id,
        message_id,
        keyboard,
        ctx.client_id,
        "目标选择刷新失败",
        "目标选择页已生成，但原消息编辑失败；请使用错误卡片上的“菜单”按钮重新进入。",
    )
    .await
}

/// 发送确认卡片。
pub(super) async fn send_confirm_prompt(
    kind: MenuInputKind,
    source_link: &str,
    target_chat_id: i64,
    target_chat_title: Option<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let target_chat_title =
        resolve_target_chat_title(target_chat_id, target_chat_title, client_id).await;
    send::ReplyPanel::card(build_confirm_text(
        kind,
        source_link,
        target_chat_id,
        target_chat_title.as_deref(),
    ))
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
    let target_chat_title = resolve_target_chat_title(target_chat_id, None, client_id).await;
    let (text, keyboard) = send::ReplyPanel::card(build_confirm_text(
        kind,
        source_link,
        target_chat_id,
        target_chat_title.as_deref(),
    ))
    .rows(confirm_button_rows())
    .into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        request_chat_id,
        message_id,
        keyboard,
        client_id,
        "确认页刷新失败",
        "确认页已生成，但原消息编辑失败；请使用错误卡片上的“菜单”按钮重新进入。",
    )
    .await
}

/// 解析确认页使用的聊天标题；原生选聊返回值优先，TDLib 查询作为其他入口的补充。
async fn resolve_target_chat_title(
    target_chat_id: i64,
    preferred_title: Option<&str>,
    client_id: i32,
) -> Option<String> {
    if let Some(title) = preferred_title
        .map(str::trim)
        .filter(|title| !title.is_empty())
    {
        return Some(title.to_owned());
    }

    let chat = match tdlib_rs::functions::get_chat(target_chat_id, client_id).await {
        Ok(chat) => chat,
        Err(err) => {
            tracing::debug!(
                target_chat_id,
                error_code = err.code,
                error_message = %err.message,
                "target chat title is unavailable"
            );
            return None;
        }
    };
    let tdlib_rs::enums::Chat::Chat(chat) = chat;
    let title = chat.title.trim();
    (!title.is_empty()).then(|| title.to_owned())
}

/// 在指定上下文上构造目标选择按钮。
pub(super) fn build_target_choice_buttons_on(
    app: &crate::app_context::AppContext,
    config: &BotConfig,
    request_chat_id: i64,
    sender_user_id: i64,
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

    let default_target_chat_id = resolve_default_target_on(app, config, request_chat_id);
    if seen_targets.insert(default_target_chat_id) {
        rows.push(vec![send::build_callback_button(
            default_target_button_label(default_target_chat_id, request_chat_id),
            &callback::target_default_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        )]);
    }

    if seen_targets.insert(request_chat_id) {
        rows.push(vec![send::build_callback_button(
            "当前私聊",
            &callback::target_alias_callback_data(request_chat_id),
            tdlib_rs::enums::ButtonStyle::Default,
        )]);
    }

    let mut aliases = targets_runtime.aliases.iter().collect::<Vec<_>>();
    aliases.sort_by_key(|(alias, _)| *alias);
    let alias_buttons = aliases
        .into_iter()
        .filter_map(|(alias, chat_id)| {
            if !seen_targets.insert(*chat_id)
                || resolve_target_by_id_on(app, *chat_id, config, request_chat_id).is_err()
            {
                return None;
            }
            Some(send::build_callback_button(
                alias,
                &callback::target_alias_callback_data(*chat_id),
                tdlib_rs::enums::ButtonStyle::Default,
            ))
        })
        .collect::<Vec<_>>();
    rows.extend(alias_buttons.chunks(2).map(<[_]>::to_vec));

    rows.push(vec![
        send::build_callback_button(
            "选择聊天",
            &callback::target_request_chat_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
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
                "修改来源",
                &callback::target_source_back_callback_data(),
                tdlib_rs::enums::ButtonStyle::Default,
            ),
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
        return Some(resolve_default_target_on(app, config, request_chat_id));
    }
    resolve_target_chat_id_on(app, &["/menu-input", "placeholder", input], request_chat_id).ok()
}

/// 在指定上下文上解析数字 chat_id。
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
) -> i64 {
    resolve_target_chat_id_on(app, &["/menu-input", "placeholder"], request_chat_id)
        .expect("default target resolution without an explicit argument cannot fail")
}

/// 目标选择卡片正文。
fn build_target_choice_text(kind: MenuInputKind, source_link: &str) -> String {
    build_target_choice_text_lines(kind, source_link).join("\n")
}

/// 构造目标选择卡片的正文行。
fn build_target_choice_text_lines(kind: MenuInputKind, source_link: &str) -> Vec<String> {
    let mut lines = vec![
        kind.target_choice_title().to_owned(),
        build_menu_step_state_line("waiting-target", "2/3"),
        crate::tgbot::transfer::card::DIVIDER.to_owned(),
    ];
    lines.extend(build_menu_context_lines(Some(source_link), None));
    lines.extend([
        crate::tgbot::transfer::card::section("目标方式"),
        "优先点“选择聊天”使用 Telegram 原生选择器；也可使用当前私聊、已有别名/上次目标或手动输入。"
            .to_owned(),
        "取消：点击“取消”按钮，或回复“取消”结束当前流程。".to_owned(),
    ]);
    lines
}

/// 确认卡片正文。
fn build_confirm_text(
    kind: MenuInputKind,
    source_link: &str,
    target_chat_id: i64,
    target_chat_title: Option<&str>,
) -> String {
    let mut lines = vec![
        kind.confirm_title().to_owned(),
        build_menu_target_step_state_line("waiting-confirm", target_chat_id, "3/3"),
        crate::tgbot::transfer::card::DIVIDER.to_owned(),
    ];
    lines.extend(build_menu_context_lines(
        Some(source_link),
        Some(target_chat_id),
    ));
    if let Some(title) = target_chat_title
        .map(str::trim)
        .filter(|title| !title.is_empty())
    {
        lines.push(crate::tgbot::transfer::card::field("目标名称", title));
    }
    lines.extend([
        crate::tgbot::transfer::card::section("下一步"),
        "确认无误后点击“执行”；来源或目标不对时，可使用下方按钮返回修改。".to_owned(),
        "取消：点击“取消”按钮，或回复“取消”结束当前流程。".to_owned(),
    ]);
    lines.join("\n")
}

/// 默认目标按钮文案。
///
/// 默认目标等于请求私聊时直接说明位置，否则使用统一目标名称。
fn default_target_button_label(default_target_chat_id: i64, request_chat_id: i64) -> &'static str {
    if default_target_chat_id == request_chat_id {
        "当前私聊"
    } else {
        "默认目标"
    }
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

    fn install_target_runtime(targets: crate::config::TargetsConfig) {
        super::super::state::clear_last_targets();
        let app = test_app_context();
        app.targets_runtime.update_runtime_config(targets);
    }

    fn resolve_default_target_for_test(config: &BotConfig, request_chat_id: i64) -> i64 {
        let app = test_app_context();
        resolve_default_target_on(app.as_ref(), config, request_chat_id)
    }

    fn test_build_target_choice_buttons(
        config: &BotConfig,
        request_chat_id: i64,
        sender_user_id: i64,
    ) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
        let app = test_app_context();
        build_target_choice_buttons_on(app.as_ref(), config, request_chat_id, sender_user_id)
    }

    // 快速转存应优先使用显式默认目标，未配置时回落到当前私聊。
    #[test]
    fn test_resolve_default_target() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(crate::config::TargetsConfig::default());
        assert_eq!(resolve_default_target_for_test(&config, 1), 1);

        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: Default::default(),
        });
        assert_eq!(resolve_default_target_for_test(&config, 1), -100);
    }

    // 目标选择页应优先提供当前私聊/默认目标、常用目标和手动输入。
    #[test]
    fn test_build_target_choice_buttons_layout() {
        use base64::{Engine as _, engine::general_purpose};

        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: std::collections::HashMap::from([("archive".to_owned(), -200)]),
        });

        let rows = test_build_target_choice_buttons(&config, 61001, 62001);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "默认目标");
        assert!(labels.contains(&"当前私聊"));
        assert!(labels.contains(&"archive"));
        assert!(labels.contains(&"选择聊天"));
        assert!(labels.contains(&"手动输入"));
        assert_eq!(rows.last().expect("should have cancel row")[0].text, "取消");

        let private_chat = rows
            .iter()
            .flatten()
            .find(|button| button.text == "当前私聊")
            .expect("private chat target should exist");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &private_chat.r#type
        else {
            panic!("private chat target must be callback");
        };
        let decoded = String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap())
            .expect("callback should be utf8");
        assert_eq!(decoded, "m:ta:61001");

        let chat_picker = rows
            .iter()
            .flatten()
            .find(|button| button.text == "选择聊天")
            .expect("native chat picker should exist");
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &chat_picker.r#type
        else {
            panic!("chat picker entry must be callback");
        };
        let decoded = String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap())
            .expect("callback should be utf8");
        assert_eq!(decoded, "m:tp");
    }

    // 快速入口仍应使用和实际命令一致的目标标题和确认标题。
    #[test]
    fn test_menu_input_kind_labels_do_not_panic_for_quick_entries() {
        assert_eq!(
            MenuInputKind::TransferDefault.target_choice_title(),
            "选择转存目标"
        );
        assert_eq!(MenuInputKind::LookupDefault.confirm_title(), "确认查询");
        assert_eq!(default_target_button_label(1, 2), "默认目标");
    }

    // 当默认目标就是当前请求私聊时，按钮文案应明确显示为“当前私聊”。
    #[test]
    fn test_default_target_button_label_uses_private_chat_name() {
        assert_eq!(default_target_button_label(10001, 10001), "当前私聊");
    }

    // 确认页应同时展示 Telegram 聊天名称和 chat_id，避免只看数字无法复核目标。
    #[test]
    fn test_build_confirm_text_shows_target_chat_title() {
        let text = build_confirm_text(
            MenuInputKind::Transfer,
            "https://t.me/c/1/2",
            -100123,
            Some("归档群"),
        );

        assert!(text.contains("目标名称：‹归档群›"));
        assert!(text.contains("目标：‹-100123›"));
    }

    // 已确认过的目标应作为上次目标优先展示，并避免和默认目标重复出现。
    #[test]
    fn test_build_target_choice_buttons_prefers_last_target() {
        let _guard = lock_target_runtime_tests();
        install_target_runtime(crate::config::TargetsConfig::default());
        let config = BotConfig::default();
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: Default::default(),
        });
        super::super::state::remember_last_target(101, 202, -100);

        let rows = test_build_target_choice_buttons(&config, 101, 202);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"上次目标"));
        assert!(!labels.contains(&"默认目标"));
    }

    // 默认目标和当前私聊相同时只保留一个入口。
    #[test]
    fn test_build_target_choice_buttons_deduplicates_private_default() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: 61001,
            aliases: std::collections::HashMap::from([
                ("same-private".to_owned(), 61001),
                ("archive".to_owned(), -200),
            ]),
        });

        let rows = test_build_target_choice_buttons(&config, 61001, 62001);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            labels.iter().filter(|label| **label == "当前私聊").count(),
            1
        );
        assert!(!labels.contains(&"same-private"));
    }

    // 上次目标就是当前私聊时不再追加同一目标的独立按钮。
    #[test]
    fn test_build_target_choice_buttons_deduplicates_private_last_target() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: Default::default(),
        });
        super::super::state::remember_last_target(61001, 62001, 61001);

        let rows = test_build_target_choice_buttons(&config, 61001, 62001);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            labels.iter().filter(|label| **label == "上次目标").count(),
            1
        );
        assert!(!labels.contains(&"当前私聊"));
    }

    // 多个别名指向同一目标时，应稳定保留字典序靠前的别名。
    #[test]
    fn test_build_target_choice_buttons_deduplicates_aliases_after_sorting() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: std::collections::HashMap::from([
                ("z-backup".to_owned(), -200),
                ("a-archive".to_owned(), -200),
            ]),
        });

        let rows = test_build_target_choice_buttons(&config, 61001, 62001);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"a-archive"));
        assert!(!labels.contains(&"z-backup"));
    }

    // 确认页第一行只放“执行”，降低误触取消或重选的概率。
    #[test]
    fn test_confirm_button_rows_layout() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = confirm_button_rows();

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[0][0].text, "执行");
        assert_eq!(rows[1][0].text, "修改来源");
        assert_eq!(rows[1][1].text, "重选目标");
        assert_eq!(rows[1][2].text, "取消");

        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[1][0].r#type
        else {
            panic!("source back must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "m:ts");
    }
}
