//! `/menu` 菜单多步骤向导中的目标会话选择与最终确认卡片模块。
//!
//! 负责目标会话候选网格（包括上次目标、默认目标、当前私聊、已配置别名、原生选聊、手动输入等）的动态渲染与去重排布，
//! 以及最终转存/查询操作执行前二次确认卡片的展示与标题解析。

use std::collections::HashSet;

use crate::config::BotConfig;
use crate::tgbot::send;

use super::super::super::common::resolve_target_chat_id_on;
use super::super::callback;
use super::super::text::{
    build_menu_context_lines, build_menu_step_state_line, build_menu_target_step_state_line,
};
use super::state::{MenuInputKind, last_target};

/// 目标会话选择卡片发送与渲染上下文结构体。
///
/// 聚合会话聊天 ID、操作者用户 ID、TDLib 客户端标识与应用上下文句柄。
#[derive(Clone, Copy)]
pub(super) struct TargetPromptContext<'a> {
    /// 全局应用上下文引用
    pub(super) app: &'a crate::app_context::AppContext,
    /// 交互所在的聊天会话 ID
    pub(super) request_chat_id: i64,
    /// 操作发起者的 Telegram 用户 ID
    pub(super) sender_user_id: i64,
    /// TDLib 客户端实例标识符
    pub(super) client_id: i32,
}

/// 发送目标选择引导卡片（新消息方式）。
///
/// # 参数说明
/// - `config`: 当前生效的配置快照
/// - `ctx`: 目标选择卡片发送上下文
/// - `kind`: 流程分类（转存、查重等）
/// - `source_link`: 当前已确认的来源链接
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

/// 原位编辑当前交互卡片为目标选择卡片。
///
/// # 参数说明
/// - `config`: 当前配置快照
/// - `ctx`: 发送上下文
/// - `message_id`: 待编辑的原卡片消息 ID
/// - `kind`: 流程大类
/// - `source_link`: 来源链接
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

/// 发送最终操作确认卡片（新消息方式）。
///
/// 内部会异步解析目标聊天的直观标题（优先使用已有标题，否则查询 TDLib）。
///
/// # 参数说明
/// - `kind`: 流程大类
/// - `source_link`: 来源链接
/// - `target_chat_id`: 目标聊天会话 ID
/// - `target_chat_title`: 已知的目标聊天标题（可选）
/// - `request_chat_id`: 请求会话 ID
/// - `client_id`: 客户端实例 ID
pub(super) async fn send_confirm_prompt(
    kind: MenuInputKind,
    source_link: &str,
    target_chat_id: i64,
    target_chat_title: Option<&str>,
    request_chat_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    // 异步补全目标群组/频道的名称标题
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

/// 原位编辑当前卡片为最终操作确认卡片。
///
/// # 参数说明
/// - `kind`: 流程大类
/// - `source_link`: 来源链接
/// - `target_chat_id`: 目标聊天 ID
/// - `request_chat_id`: 请求会话 ID
/// - `message_id`: 原消息 ID
/// - `client_id`: 客户端实例 ID
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

/// 解析用于在确认页中展示的目标聊天名称（title）。
///
/// 解析策略：
/// 1. 若外部已传入非空的 preferred_title（例如来自原生选聊回调的 title/username），直接优先使用；
/// 2. 否则通过 TDLib `get_chat` 异步接口查询目标 chat 对象的 title 属性；
/// 3. 若查询失败或无可用标题，返回 `None`（卡片降级只回显数字 target_chat_id）。
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

/// 在指定应用与配置上下文下，动态构建目标选择界面的内联按钮网格。
///
/// 按钮排布策略与去重机制：
/// 1. 首行优先级（上次使用过的目标）：若存在记忆的 `last_target` 且合法，置于首行（Primary 样式）；
/// 2. 默认目标（`default_target`）：若未与上次目标重复，追加默认目标按钮（Default 样式）；
/// 3. 当前私聊（`request_chat_id`）：若为私聊会话且未重复，追加“当前私聊”入口；
/// 4. 自定义别名列表：按别名 ASCII 字典序排序，过滤掉重复 ID 与非法目标后，每行排布 2 个按钮；
/// 5. 控制区倒数第二行：原生“选择聊天”按钮（Primary 样式）与“手动输入”按钮（Default 样式）；
/// 6. 末行：“取消”红色警示按钮（Danger 样式）。
///
/// # 参数说明
/// - `app`: 全局应用上下文引用
/// - `config`: 机器人静态配置引用
/// - `request_chat_id`: 发起交互所在的聊天会话 ID
/// - `sender_user_id`: 发起交互的用户 ID
///
/// # 返回值
/// 返回二维内联按钮向量矩阵。
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

    // 1. 上次目标（优先展示）
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

    // 2. 默认目标
    let default_target_chat_id = resolve_default_target_on(app, config, request_chat_id);
    if seen_targets.insert(default_target_chat_id) {
        rows.push(vec![send::build_callback_button(
            default_target_button_label(default_target_chat_id, request_chat_id),
            &callback::target_default_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        )]);
    }

    // 3. 当前私聊
    if seen_targets.insert(request_chat_id) {
        rows.push(vec![send::build_callback_button(
            "当前私聊",
            &callback::target_alias_callback_data(request_chat_id),
            tdlib_rs::enums::ButtonStyle::Default,
        )]);
    }

    // 4. 别名列表（排序并按两列分行展示）
    let mut aliases = targets_runtime.aliases.iter().collect::<Vec<_>>();
    aliases.sort_by_key(|(alias, _)| *alias);
    let alias_buttons = aliases
        .into_iter()
        .filter_map(|(alias, chat_id)| {
            // 过滤重复的 target_chat_id 或无法解析的目标
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

    // 5. 原生选聊与手动输入入口
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
    // 6. 取消操作
    rows.push(vec![send::build_callback_button(
        "取消",
        &callback::cancel_input_callback_data(),
        tdlib_rs::enums::ButtonStyle::Danger,
    )]);
    rows
}

/// 构建确认页面的标准操作按钮行。
///
/// 结构设计：
/// - 第一行单列：高亮绿色的“执行”（Success 样式），作为主要推进动作；
/// - 第二行辅助动作：“修改来源”、“重选目标”、“取消”三个平级按钮。
pub(super) fn confirm_button_rows() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![
        // 第一行：确认执行
        vec![send::build_callback_button(
            "执行",
            &callback::target_confirm_callback_data(),
            tdlib_rs::enums::ButtonStyle::Success,
        )],
        // 第二行：返回前置步骤或取消
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

/// 解析用户输入的目标文本（支持别名、特殊标记 `default` 以及纯数字 chat_id）。
///
/// # 参数说明
/// - `app`: 全局应用上下文引用
/// - `input`: 用户输入的字符串
/// - `config`: 配置引用
/// - `request_chat_id`: 当前请求会话 ID
///
/// # 返回值
/// 若成功解析为目标 chat_id 则返回 `Some(i64)`，否则返回 `None`。
pub(super) fn resolve_target_input_on(
    app: &crate::app_context::AppContext,
    input: &str,
    config: &BotConfig,
    request_chat_id: i64,
) -> Option<i64> {
    // 特殊标记 "default" 对应系统配置默认目标
    if input.eq_ignore_ascii_case("default") {
        return Some(resolve_default_target_on(app, config, request_chat_id));
    }
    // 委托给 targets 模块的通用目标解析器
    resolve_target_chat_id_on(app, &["/menu-input", "placeholder", input], request_chat_id).ok()
}

/// 验证并解析纯数字目标会话 ID 的合法性。
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

/// 解析当前交互所采用的默认目标会话 ID。
///
/// 若配置了全局默认目标则使用全局目标；若未配置且为私聊，则回退到当前私聊。
pub(super) fn resolve_default_target_on(
    app: &crate::app_context::AppContext,
    _config: &BotConfig,
    request_chat_id: i64,
) -> i64 {
    resolve_target_chat_id_on(app, &["/menu-input", "placeholder"], request_chat_id)
        .expect("default target resolution without an explicit argument cannot fail")
}

/// 组装目标选择引导卡片的完整文本内容。
fn build_target_choice_text(kind: MenuInputKind, source_link: &str) -> String {
    build_target_choice_text_lines(kind, source_link).join("\n")
}

/// 构建目标选择卡片的多行文本切片列表。
fn build_target_choice_text_lines(kind: MenuInputKind, source_link: &str) -> Vec<String> {
    let mut lines = vec![
        kind.target_choice_title().to_owned(),
        build_menu_step_state_line("waiting-target", "2/3"),
        crate::tgbot::transfer::card::DIVIDER.to_owned(),
    ];
    // 回显第一步已输入的来源链接
    lines.extend(build_menu_context_lines(Some(source_link), None));
    lines.extend([
        crate::tgbot::transfer::card::section("目标方式"),
        "优先点“选择聊天”使用 Telegram 原生选择器；也可使用当前私聊、已有别名/上次目标或手动输入。"
            .to_owned(),
        "取消：点击“取消”按钮，或回复“取消”结束当前流程。".to_owned(),
    ]);
    lines
}

/// 组装最终操作确认卡片的文本内容。
///
/// 包含步骤指示（3/3）、来源链接回显、目标 chat_id 回显、目标名称展示（若已解析出）以及下一步操作提示。
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
    // 回显来源与目标 ID 上下文
    lines.extend(build_menu_context_lines(
        Some(source_link),
        Some(target_chat_id),
    ));
    // 回显目标群组/频道标题（若可用）
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

/// 动态计算默认目标按钮的展示标签文案。
///
/// 若当前会话本身就是默认目标（例如私聊会话作为默认目标），直接展示为“当前私聊”，否则展示为“默认目标”。
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

    /// 测试全局锁，防止目标配置动态更新和上次目标记录在多测试并发运行时发生数据竞争
    static TARGET_RUNTIME_TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    /// 获取测试同步锁守卫
    fn lock_target_runtime_tests() -> MutexGuard<'static, ()> {
        match TARGET_RUNTIME_TEST_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    /// 获取当前测试用的全局应用上下文
    fn test_app_context() -> std::sync::Arc<crate::app_context::AppContext> {
        app_context()
    }

    /// 安装测试用的目标运行时配置并清空上次目标记忆
    fn install_target_runtime(targets: crate::config::TargetsConfig) {
        // 清理上一次记录的目标缓存
        super::super::state::clear_last_targets();
        // 获取应用上下文
        let app = test_app_context();
        // 更新目标运行时配置
        app.targets_runtime.update_runtime_config(targets);
    }

    /// 辅助函数：针对测试上下文解析默认目标
    fn resolve_default_target_for_test(config: &BotConfig, request_chat_id: i64) -> i64 {
        let app = test_app_context();
        resolve_default_target_on(app.as_ref(), config, request_chat_id)
    }

    /// 辅助函数：针对测试上下文构建目标选择内联按钮矩阵
    fn test_build_target_choice_buttons(
        config: &BotConfig,
        request_chat_id: i64,
        sender_user_id: i64,
    ) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
        let app = test_app_context();
        build_target_choice_buttons_on(app.as_ref(), config, request_chat_id, sender_user_id)
    }

    /// 测试用例：快速转存应优先使用显式默认目标，未配置时回落到当前私聊
    #[test]
    fn test_resolve_default_target() {
        // 获取同步测试锁
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        // 初始安装默认空配置，此时默认目标未配置（default_chat_id 为 0），应回退到请求会话 ID 1
        install_target_runtime(crate::config::TargetsConfig::default());
        assert_eq!(resolve_default_target_for_test(&config, 1), 1);

        // 配置默认目标为 -100，应成功解析为 -100
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: Default::default(),
        });
        assert_eq!(resolve_default_target_for_test(&config, 1), -100);
    }

    /// 测试用例：目标选择页应优先提供当前私聊/默认目标、常用目标和手动输入
    #[test]
    fn test_build_target_choice_buttons_layout() {
        use base64::{Engine as _, engine::general_purpose};

        // 获取同步测试锁
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        // 安装包含默认目标与别名的目标配置
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: std::collections::HashMap::from([("archive".to_owned(), -200)]),
        });

        // 为会话 61001 构建目标按钮矩阵
        let rows = test_build_target_choice_buttons(&config, 61001, 62001);
        // 提取所有按钮的文本标签
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        // 首个按钮应为“默认目标”
        assert_eq!(rows[0][0].text, "默认目标");
        // 校验包含当前私聊、别名目标、原生聊天选择器和手动输入
        assert!(labels.contains(&"当前私聊"));
        assert!(labels.contains(&"archive"));
        assert!(labels.contains(&"选择聊天"));
        assert!(labels.contains(&"手动输入"));
        // 最后一行的唯一按钮应为“取消”
        assert_eq!(rows.last().expect("should have cancel row")[0].text, "取消");

        // 校验“当前私聊”按钮的回调数据格式
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

        // 校验“选择聊天”按钮的回调数据格式
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

    /// 测试用例：快速入口仍应使用和实际命令一致的目标标题和确认标题
    #[test]
    fn test_menu_input_kind_labels_do_not_panic_for_quick_entries() {
        // 验证 TransferDefault 的目标选择标题
        assert_eq!(
            MenuInputKind::TransferDefault.target_choice_title(),
            "选择转存目标"
        );
        // 验证 LookupDefault 的确认标题
        assert_eq!(MenuInputKind::LookupDefault.confirm_title(), "确认查询");
        // 验证默认目标按钮文案
        assert_eq!(default_target_button_label(1, 2), "默认目标");
    }

    /// 测试用例：当默认目标就是当前请求私聊时，按钮文案应明确显示为“当前私聊”
    #[test]
    fn test_default_target_button_label_uses_private_chat_name() {
        assert_eq!(default_target_button_label(10001, 10001), "当前私聊");
    }

    /// 测试用例：确认页应同时展示 Telegram 聊天名称和 chat_id，避免只看数字无法复核目标
    #[test]
    fn test_build_confirm_text_shows_target_chat_title() {
        // 构建包含自定义群名和 chat_id 的确认文本
        let text = build_confirm_text(
            MenuInputKind::Transfer,
            "https://t.me/c/1/2",
            -100123,
            Some("归档群"),
        );

        // 验证文本包含格式化后的目标名称和聊天 ID
        assert!(text.contains("目标名称：‹归档群›"));
        assert!(text.contains("目标：‹-100123›"));
    }

    /// 测试用例：已确认过的目标应作为上次目标优先展示，并避免和默认目标重复出现
    #[test]
    fn test_build_target_choice_buttons_prefers_last_target() {
        let _guard = lock_target_runtime_tests();
        install_target_runtime(crate::config::TargetsConfig::default());
        let config = BotConfig::default();
        // 设置默认目标为 -100
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: Default::default(),
        });
        // 记录上次目标同样为 -100
        super::super::state::remember_last_target(101, 202, -100);

        let rows = test_build_target_choice_buttons(&config, 101, 202);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        // 此时应显示“上次目标”，并且默认目标因 ID 重复被自动去重
        assert!(labels.contains(&"上次目标"));
        assert!(!labels.contains(&"默认目标"));
    }

    /// 测试用例：默认目标和当前私聊相同时只保留一个入口
    #[test]
    fn test_build_target_choice_buttons_deduplicates_private_default() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        // 设置默认目标与别名同为当前私聊 61001
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

        // “当前私聊”仅出现一次
        assert_eq!(
            labels.iter().filter(|label| **label == "当前私聊").count(),
            1
        );
        // 指向同一 chat_id 的别名已被去重过滤
        assert!(!labels.contains(&"same-private"));
    }

    /// 测试用例：上次目标就是当前私聊时不再追加同一目标的独立按钮
    #[test]
    fn test_build_target_choice_buttons_deduplicates_private_last_target() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: -100,
            aliases: Default::default(),
        });
        // 将上次目标记录为当前私聊 61001
        super::super::state::remember_last_target(61001, 62001, 61001);

        let rows = test_build_target_choice_buttons(&config, 61001, 62001);
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        // 仅保留“上次目标”，当前私聊被去重
        assert_eq!(
            labels.iter().filter(|label| **label == "上次目标").count(),
            1
        );
        assert!(!labels.contains(&"当前私聊"));
    }

    /// 测试用例：多个别名指向同一目标时，应稳定保留字典序靠前的别名
    #[test]
    fn test_build_target_choice_buttons_deduplicates_aliases_after_sorting() {
        let _guard = lock_target_runtime_tests();
        let config = BotConfig::default();
        // 两个别名指向相同 chat_id -200
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

        // 应保留字典序靠前的 "a-archive"，排除 "z-backup"
        assert!(labels.contains(&"a-archive"));
        assert!(!labels.contains(&"z-backup"));
    }

    /// 测试用例：确认页第一行只放“执行”，降低误触取消或重选的概率
    #[test]
    fn test_confirm_button_rows_layout() {
        use base64::{Engine as _, engine::general_purpose};

        let rows = confirm_button_rows();

        // 验证两行布局
        assert_eq!(rows.len(), 2);
        // 第一行仅包含 1 个按钮："执行"
        assert_eq!(rows[0].len(), 1);
        assert_eq!(rows[0][0].text, "执行");
        // 第二行包含 3 个操作按钮
        assert_eq!(rows[1][0].text, "修改来源");
        assert_eq!(rows[1][1].text, "重选目标");
        assert_eq!(rows[1][2].text, "取消");

        // 验证“修改来源”按钮的回调数据
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &rows[1][0].r#type
        else {
            panic!("source back must be callback");
        };
        let decoded =
            String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap();
        assert_eq!(decoded, "m:ts");
    }
}
