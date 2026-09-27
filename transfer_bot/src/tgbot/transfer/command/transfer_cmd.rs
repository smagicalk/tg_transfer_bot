// `/transfer` 命令实现。

use std::sync::Arc;

use crate::config::ClientRole;
use crate::config::{BotConfig, RequestActor};
use crate::tgbot::send;
use crate::tgbot::transfer::card;

use super::build_menu_home_button_data;
use super::common::{
    CommandStyle, resolve_target_chat_id_on, transfer_command as build_transfer_command,
};
use super::menu;
use super::{build_downloads_status_button_data, build_view_commands_button};
use crate::tgbot::transfer::types::{SourceKind, TransferPlan};

/// 转存命令的请求与交互消息上下文。
pub(super) struct TransferCommandContext {
    /// 触发转存命令的原始消息 ID
    pub(super) request_message_id: i64,
    /// 交互卡片的消息 ID（如果是在现有交互卡片上编辑触发）
    pub(super) interaction_message_id: Option<i64>,
    /// 请求发起者（包含用户 ID 与会话 ID）
    pub(super) actor: RequestActor,
    /// TDLib 客户端 ID
    pub(super) client_id: i32,
}

/// 在指定上下文上执行 `/transfer` 命令。
///
/// 解析用户输入的参数或回复的消息，若无参数且非回复消息则进入交互式向导，
/// 否则生成转存计划并派发后台任务。
///
/// # 参数
/// - `app_context`: 全局应用上下文
/// - `text`: 解析拆分后的命令文本切片集合
/// - `config`: Bot 配置引用
/// - `request_message`: 触发命令的 Telegram 消息对象引用
/// - `actor`: 发起请求的操作者
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功执行返回 Ok(())
pub async fn transfer_command_on(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    text: Vec<&str>,
    config: Arc<BotConfig>,
    request_message: &tdlib_rs::types::Message,
    actor: RequestActor,
    client_id: i32,
) -> anyhow::Result<()> {
    let request_chat_id = request_message.chat_id;
    let request_message_id = request_message.id;
    // 尝试解析转存源（从文本中的链接或被回复/转发的消息中提取）
    let source = match resolve_transfer_source(&text, request_message) {
        Ok(source) => source,
        // 如果未提供参数且未回复消息（命令仅有 "/transfer" 一个参数），进入交互式转存向导
        Err(_) if text.len() == 1 => {
            menu::start_transfer_input_from_command(request_chat_id, actor.user_id, client_id)
                .await?;
            tracing::debug!(
                request_chat_id,
                request_message_id,
                sender_user_id = actor.user_id,
                "transfer command without args entered interactive wizard"
            );
            return Ok(());
        }
        Err(err) => return Err(err),
    };
    // 运行转存计划
    run_transfer_plan_on(
        app_context,
        text,
        source,
        config,
        TransferCommandContext {
            request_message_id,
            interaction_message_id: None,
            actor,
            client_id,
        },
    )
    .await
}

/// 在指定上下文上执行菜单/向导收集好的链接转存。
///
/// # 参数
/// - `app_context`: 全局应用上下文
/// - `text`: 命令参数列表（例如 `["/transfer", "<link>", ...]`）
/// - `config`: Bot 配置引用
/// - `_request_chat_id`: 会话 ID
/// - `ctx`: 转存命令上下文
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功派发转存计划返回 Ok(())
pub(in crate::tgbot::transfer::command) async fn transfer_link_command_on(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    text: Vec<&str>,
    config: Arc<BotConfig>,
    _request_chat_id: i64,
    ctx: TransferCommandContext,
) -> anyhow::Result<()> {
    if text.len() < 2 {
        anyhow::bail!("usage: /transfer <link> [target]");
    }
    // 构造由向导提取出的转存源对象
    let source = ResolvedTransferSource {
        source_link: text[1].to_owned(),
        source_kind: SourceKind::Link,
        preferred_source_client_role: effective_link_source_role(&config),
        source_message_chat_id: None,
        source_message_id: None,
    };
    run_transfer_plan_on(app_context, text, source, config, ctx).await
}

/// 在指定上下文上创建计划、发送进度卡片并派发后台任务。
///
/// # 参数
/// - `app_context`: 全局应用上下文
/// - `text`: 原始命令参数数组
/// - `source`: 已解析出的转存源
/// - `config`: Bot 配置引用
/// - `ctx`: 转存命令上下文
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功派发任务返回 Ok(())
async fn run_transfer_plan_on(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    text: Vec<&str>,
    mut source: ResolvedTransferSource,
    config: Arc<BotConfig>,
    ctx: TransferCommandContext,
) -> anyhow::Result<()> {
    // 链接源的策略是“bot 优先，user 备用”；如果当前配置没有 bot client，
    // 则直接降级 user，避免单 user 部署下因为缺少 bot client 失败。
    if source.source_kind == SourceKind::Link
        && source.preferred_source_client_role == ClientRole::Bot
        && !config.runtime_clients.contains_key(&ClientRole::Bot)
    {
        source.preferred_source_client_role = ClientRole::User;
    }

    // 解析目标转存频道或会话的 chat_id
    let target_chat_id = resolve_transfer_target_chat_id_on(
        app_context.as_ref(),
        &text,
        &source,
        &config,
        ctx.actor.request_chat_id,
    )?;

    // 构建完整的转存计划对象
    let plan = TransferPlan {
        actor: ctx.actor,
        source_link: source.source_link,
        source_kind: source.source_kind,
        preferred_source_client_role: source.preferred_source_client_role,
        allow_user_fallback: true,
        source_message_chat_id: source.source_message_chat_id,
        source_message_id: source.source_message_id,
        target_chat_id,
        request_chat_id: ctx.actor.request_chat_id,
        request_message_id: ctx.request_message_id,
        force_retransfer: false,
    };
    // 派发转存计划
    dispatch_transfer_plan(
        app_context,
        plan,
        config,
        ctx.actor.request_chat_id,
        ctx.request_message_id,
        ctx.interaction_message_id,
        ctx.client_id,
    )
    .await
}

/// 发送初始回执并启动后台转存任务。
///
/// 若已有历史成功记录，则发送二次确认卡片；否则申请执行许可并启动异步后台任务。
///
/// # 参数
/// - `app_context`: 全局应用上下文
/// - `plan`: 转存任务执行计划
/// - `_config`: Bot 配置引用
/// - `request_chat_id`: 请求会话 ID
/// - `request_message_id`: 请求消息 ID
/// - `interaction_message_id`: 交互卡片消息 ID（如果有）
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功派发返回 Ok(())
async fn dispatch_transfer_plan(
    app_context: std::sync::Arc<crate::app_context::AppContext>,
    plan: TransferPlan,
    _config: Arc<BotConfig>,
    request_chat_id: i64,
    request_message_id: i64,
    interaction_message_id: Option<i64>,
    client_id: i32,
) -> anyhow::Result<()> {
    // 日志只记录请求定位和目标 chat；源链接会回显给用户，但不写入日志文件。
    tracing::info!(
        request_chat_id,
        request_message_id,
        target_chat_id = plan.target_chat_id,
        source_kind = plan.source_kind.as_str(),
        source_role = plan.preferred_source_client_role.as_str(),
        owner_user_id = plan.actor.user_id,
        "transfer command accepted"
    );

    // 先给用户一个即时反馈，避免长时间下载/上传期间命令看起来像“卡住了”。
    let progress_message_id = if let Some(message_id) = interaction_message_id {
        edit_transfer_interaction_card(
            format_transfer_accepted_text(&plan),
            build_transfer_accepted_button_rows(),
            request_chat_id,
            message_id,
            client_id,
        )
        .await?;
        message_id
    } else {
        send::send_card_message_with_buttons_returning(
            format_transfer_accepted_text(&plan),
            request_chat_id,
            build_transfer_accepted_button_rows(),
            client_id,
        )
        .await?
        .id
    };

    // 已有成功结果时先在同一张卡片上确认，避免误触重复上传。
    if !plan.force_retransfer
        && let Some(old) = crate::tgbot::transfer::store::find_success_job_by_source_target(
            &plan.source_link,
            plan.target_chat_id,
        )
        .await?
    {
        let old_link = crate::tgbot::transfer::refresh_stored_result_link(
            old.id,
            old.target_chat_id,
            old.result_message_id,
            &old.result_message_link,
            super::super::transfer_client_ids()?.upload,
        )
        .await?;
        app_context.retransfer_confirm.put_plan(
            request_chat_id,
            plan.actor.user_id,
            progress_message_id,
            plan.clone(),
        );
        edit_transfer_interaction_card(
            format_retransfer_confirm_text(&plan, old.id, &old_link),
            build_retransfer_confirm_button_rows(&old_link),
            request_chat_id,
            progress_message_id,
            client_id,
        )
        .await?;
        return Ok(());
    }
    // 后台任务会持续编辑这条消息，把它变成转存进度面板。
    let admission = app_context
        .transfer_runtime
        .try_admit_transfer()
        .ok_or_else(|| anyhow::anyhow!("执行器正在退出，暂不接收新的转存任务"))?;
    super::super::spawn_transfer_job(
        app_context,
        plan,
        request_chat_id,
        Some(progress_message_id),
        super::super::transfer_client_ids()?,
        admission,
    );
    Ok(())
}

/// 再次转存确认按钮的内联回调前缀标识常量。
const RETRANSFER_CALLBACK_DATA: &str = "tr:again";

/// 判断回调数据字符串是否为“再次转存”确认。
///
/// # 参数
/// - `data`: 回调数据字符串
///
/// # 返回值
/// - `bool`: 若匹配返回 true
pub(super) fn is_retransfer_callback_data(data: &str) -> bool {
    data == RETRANSFER_CALLBACK_DATA
}

/// 处理“再次转存”确认回调查询；计划按当前卡片定位，避免把长链接写入 callback_data。
///
/// 当用户点击“再次转存”按钮时触发。从内存确认暂存区中提取计划，将 `force_retransfer` 设为 true，
/// 并重新调度转存任务。
///
/// # 参数
/// - `app`: 全局应用上下文引用
/// - `update`: 回调查询事件
/// - `config`: Bot 配置引用
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 成功执行返回 Ok(())
pub(super) async fn retransfer_callback_query_on(
    app: &crate::app_context::AppContext,
    update: tdlib_rs::types::UpdateNewCallbackQuery,
    config: Arc<BotConfig>,
    client_id: i32,
) -> anyhow::Result<()> {
    // 校验回调数据载荷
    let tdlib_rs::enums::CallbackQueryPayload::Data(data) = update.payload else {
        send::answer_callback_query(update.id, Some("暂不支持这种按钮类型"), client_id).await?;
        return Ok(());
    };
    if !is_retransfer_callback_data(&data.data) {
        send::answer_callback_query(update.id, Some("重复转存按钮参数无效"), client_id).await?;
        return Ok(());
    }
    // 从再次转存确认管理器中取出暂存的转存计划
    let Some(mut plan) =
        app.retransfer_confirm
            .take_plan(update.chat_id, update.sender_user_id, update.message_id)
    else {
        send::answer_callback_query(update.id, Some("确认已失效，请重新发起"), client_id).await?;
        return Ok(());
    };
    // 标记为强制重新转存，跳过历史结果查重
    plan.force_retransfer = true;
    // callback query ID 区分这次明确确认，避免与原命令请求幂等键混用
    plan.request_message_id = update.id;
    send::answer_callback_query(update.id, Some("开始再次转存"), client_id).await?;
    // 重新派发转存计划
    dispatch_transfer_plan(
        Arc::new(app.clone()),
        plan,
        config,
        update.chat_id,
        update.id,
        Some(update.message_id),
        client_id,
    )
    .await
}

/// 编辑现有的转存交互卡片消息。
///
/// # 参数
/// - `text`: 更新后的卡片文本
/// - `rows`: 内联键盘按钮矩阵
/// - `chat_id`: 目标会话 ID
/// - `message_id`: 目标消息 ID
/// - `client_id`: TDLib 客户端 ID
///
/// # 返回值
/// - `anyhow::Result<()>`: 编辑成功返回 Ok(())
async fn edit_transfer_interaction_card(
    text: String,
    rows: Vec<Vec<tdlib_rs::types::InlineKeyboardButton>>,
    chat_id: i64,
    message_id: i64,
    client_id: i32,
) -> anyhow::Result<()> {
    let (text, keyboard) = send::ReplyPanel::card(text).rows(rows).into_card_parts()?;
    send::edit_interaction_card_or_error(
        text,
        chat_id,
        message_id,
        keyboard,
        client_id,
        "转存卡片更新失败",
        "当前转存状态已生成，但原消息编辑失败；请返回菜单重新发起。",
    )
    .await
}

/// 格式化“已存在转存结果，确认是否再次转存”的卡片正文。
///
/// # 参数
/// - `plan`: 转存计划引用
/// - `old_job_id`: 历史转存任务 ID
/// - `old_link`: 历史转存结果的消息链接或文本定位
///
/// # 返回值
/// - `String`: 格式化后的卡片文本
fn format_retransfer_confirm_text(plan: &TransferPlan, old_job_id: i64, old_link: &str) -> String {
    [
        "已存在转存结果".to_owned(),
        card::summary_line("confirm-again", Some(old_job_id), plan.target_chat_id),
        card::section("原结果"),
        card::field("地址", old_link),
        card::note(
            "原消息可能已被删除。再次转存会创建新任务，并将最新结果作为后续查询的默认地址。",
        ),
    ]
    .join("\n")
}

/// 构造再次转存确认卡片的按钮行矩阵。
///
/// 若原历史结果为可用 URL 则提供“打开原结果”，同时附带“再次转存”和“取消”按钮。
///
/// # 参数
/// - `old_link`: 原历史转存消息链接
///
/// # 返回值
/// - `Vec<Vec<InlineKeyboardButton>>`: 内联键盘矩阵
fn build_retransfer_confirm_button_rows(
    old_link: &str,
) -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    let mut rows = Vec::new();
    if send::is_openable_url(old_link) {
        rows.push(vec![send::build_url_button(
            "打开原结果",
            old_link,
            tdlib_rs::enums::ButtonStyle::Default,
        )]);
    }
    rows.push(vec![send::build_callback_button(
        "再次转存",
        RETRANSFER_CALLBACK_DATA,
        tdlib_rs::enums::ButtonStyle::Danger,
    )]);
    rows.push(vec![send::build_callback_button(
        "取消",
        &build_menu_home_button_data(),
        tdlib_rs::enums::ButtonStyle::Default,
    )]);
    rows
}

/// 构造 `/transfer` 首次回执按钮。
///
/// 按钮区优先放可直接点击的运行列表、命令说明和菜单。
///
/// # 返回值
/// - `Vec<Vec<InlineKeyboardButton>>`: 内联键盘矩阵
fn build_transfer_accepted_button_rows() -> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "查看运行列表",
            &build_downloads_status_button_data("running", 8),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        build_view_commands_button(Some("transfer")),
        send::build_callback_button(
            "菜单",
            &build_menu_home_button_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

/// `/help transfer` 共用的详细说明正文。
///
/// 转存命令支持“链接源”和“bot 可见消息源”两种输入方式，说明留在转存模块维护，
/// 避免 help 模块重复理解目标解析、bot/user 下载策略和交互入口。
///
/// # 返回值
/// - `String`: 帮助说明卡片正文
pub(in crate::tgbot::transfer::command) fn build_transfer_help_detail_text() -> String {
    [
        "transfer".to_owned(),
        "用途：转存单条消息或相册链接。".to_owned(),
        "说明：target 可填数字 chat_id 或配置里的别名；不传时使用预先配置的目标。".to_owned(),
        "说明：bot 无法读取链接源时会尝试使用备用 user；两个账号至少有一个必须能访问源。"
            .to_owned(),
        card::DIVIDER.to_owned(),
        card::section("命令"),
        build_transfer_command("<link>", 0, CommandStyle::Long).replace(" 0", " [target]"),
        String::new(),
        card::section("示例"),
        "/transfer https://t.me/c/123/456".to_owned(),
        "/transfer https://t.me/c/123/456 -1001234567890".to_owned(),
        "/transfer https://t.me/c/123/456 archive".to_owned(),
    ]
    .join("\n")
}

/// `/help transfer` 共用的按钮入口。
///
/// help 详情页正文已经给出命令示例，这里只保留真实交互入口；
/// 返回目录和菜单由 help 模块统一追加。
///
/// # 返回值
/// - `Vec<Vec<InlineKeyboardButton>>`: 内联键盘矩阵
pub(in crate::tgbot::transfer::command) fn build_transfer_help_entry_rows()
-> Vec<Vec<tdlib_rs::types::InlineKeyboardButton>> {
    vec![vec![
        send::build_callback_button(
            "开始转存",
            &menu::build_menu_new_transfer_callback_data(),
            tdlib_rs::enums::ButtonStyle::Primary,
        ),
        send::build_callback_button(
            "快速转存",
            &menu::build_menu_quick_transfer_default_callback_data(),
            tdlib_rs::enums::ButtonStyle::Default,
        ),
    ]]
}

/// 解析 `/transfer` 的源输入。
///
/// 支持两种输入：
/// - `/transfer <link> [target]`：链接源，优先 bot 读取，失败再 user；
/// - 回复 bot 可见媒体后发送 `/transfer [target]`：bot 消息源，直接读取被回复消息。
///
/// # 参数
/// - `text`: 命令行参数切片数组
/// - `request_message`: Telegram 原始消息对象
///
/// # 返回值
/// - `anyhow::Result<ResolvedTransferSource>`: 解析出的转存源数据
fn resolve_transfer_source(
    text: &[&str],
    request_message: &tdlib_rs::types::Message,
) -> anyhow::Result<ResolvedTransferSource> {
    // 检查第 1 个参数是否形如 Telegram 链接
    if text.get(1).is_some_and(|arg| looks_like_telegram_link(arg)) {
        return Ok(ResolvedTransferSource {
            source_link: text[1].to_owned(),
            source_kind: SourceKind::Link,
            preferred_source_client_role: ClientRole::Bot,
            source_message_chat_id: None,
            source_message_id: None,
        });
    }

    // 检查当前消息是否回复了某条具体消息
    if let Some((chat_id, message_id)) = replied_message_location(request_message) {
        return Ok(ResolvedTransferSource {
            source_link: bot_message_source_link(chat_id, message_id),
            source_kind: SourceKind::BotMessage,
            preferred_source_client_role: ClientRole::Bot,
            source_message_chat_id: Some(chat_id),
            source_message_id: Some(message_id),
        });
    }

    // 检查当前消息是否包含转发来源定位
    if let Some((chat_id, message_id)) = forwarded_message_location(request_message) {
        return Ok(ResolvedTransferSource {
            source_link: bot_message_source_link(chat_id, message_id),
            source_kind: SourceKind::BotMessage,
            preferred_source_client_role: ClientRole::Bot,
            source_message_chat_id: Some(chat_id),
            source_message_id: Some(message_id),
        });
    }

    anyhow::bail!(
        "usage: /transfer <link> [target], or reply a bot-visible media message with /transfer [target]"
    )
}

/// bot 可见消息的稳定源标识。
///
/// # 参数
/// - `chat_id`: 会话 ID
/// - `message_id`: 消息 ID
///
/// # 返回值
/// - `String`: 稳定源标识字符串，如 `bot-message:123:456`
fn bot_message_source_link(chat_id: i64, message_id: i64) -> String {
    format!("bot-message:{chat_id}:{message_id}")
}

/// 链接源优先使用 bot；如果当前配置没有启用 bot client，则自动退回 user。
///
/// # 参数
/// - `config`: Bot 配置引用
///
/// # 返回值
/// - `ClientRole`: 首选的角色类型
fn effective_link_source_role(config: &BotConfig) -> ClientRole {
    if config.runtime_clients.contains_key(&ClientRole::Bot) {
        ClientRole::Bot
    } else {
        ClientRole::User
    }
}

/// 解析目标 chat（仅供测试调用）。
///
/// 回复消息模式下 `/transfer archive` 的第 2 个参数是 target；
/// 链接模式下 `/transfer <link> archive` 的第 3 个参数才是 target，
/// 因此这里需要按 source_kind 重新组装给公共解析器。
#[cfg(test)]
fn resolve_transfer_target_chat_id(
    text: &[&str],
    source: &ResolvedTransferSource,
    _config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    let app_context = crate::app_context::app_context();
    resolve_transfer_target_chat_id_on(app_context.as_ref(), text, source, _config, request_chat_id)
}

/// 在指定上下文上解析 `/transfer` 目标 chat。
///
/// # 参数
/// - `app`: 全局应用上下文
/// - `text`: 命令参数列表
/// - `source`: 已解析出的转存源
/// - `_config`: Bot 配置引用
/// - `request_chat_id`: 当前请求会话 ID
///
/// # 返回值
/// - `anyhow::Result<i64>`: 解析出的目标 chat_id
fn resolve_transfer_target_chat_id_on(
    app: &crate::app_context::AppContext,
    text: &[&str],
    source: &ResolvedTransferSource,
    _config: &BotConfig,
    request_chat_id: i64,
) -> anyhow::Result<i64> {
    match source.source_kind {
        // 链接模式下直接利用公共目标解析器
        SourceKind::Link => resolve_target_chat_id_on(app, text, request_chat_id),
        // 回复消息模式下伪造第 2 个占位参数以符合标准目标参数位置
        SourceKind::BotMessage => {
            let target_args = if text.len() >= 2 {
                vec![text[0], "bot-message-source", text[1]]
            } else {
                vec![text[0], "bot-message-source"]
            };
            resolve_target_chat_id_on(app, &target_args, request_chat_id)
        }
    }
}

/// 命令解析出的源信息。
struct ResolvedTransferSource {
    /// 源链接文本或虚拟源定位字符串
    source_link: String,
    /// 转存源种类（链接源或 Bot 可见消息源）
    source_kind: SourceKind,
    /// 首选下载客户端角色（Bot 或 User）
    preferred_source_client_role: ClientRole,
    /// 来源消息所在的会话 ID（针对 Bot 可见消息）
    source_message_chat_id: Option<i64>,
    /// 来源消息的消息 ID（针对 Bot 可见消息）
    source_message_id: Option<i64>,
}

/// 粗略判断参数是否是 Telegram 链接；真正合法性仍由 spider 层负责。
///
/// # 参数
/// - `input`: 输入字符串
///
/// # 返回值
/// - `bool`: 若符合链接开头返回 true
fn looks_like_telegram_link(input: &str) -> bool {
    input.starts_with("https://t.me/")
        || input.starts_with("http://t.me/")
        || input.starts_with("t.me/")
}

/// 从命令消息中提取被回复消息定位。
///
/// # 参数
/// - `message`: 消息对象引用
///
/// # 返回值
/// - `Option<(i64, i64)>`: 被回复消息的 (chat_id, message_id)
fn replied_message_location(message: &tdlib_rs::types::Message) -> Option<(i64, i64)> {
    let tdlib_rs::enums::MessageReplyTo::Message(reply) = message.reply_to.as_ref()? else {
        return None;
    };
    let chat_id = if reply.chat_id != 0 {
        reply.chat_id
    } else {
        message.chat_id
    };
    if reply.message_id == 0 {
        return None;
    }
    Some((chat_id, reply.message_id))
}

/// 从转发消息中提取原始消息定位。
///
/// 优先使用 TDLib 给出的 `forward_info.source`，因为它包含“上一次转发来源”的真实 chat/message；
/// 如果没有 source，再退回 channel origin 的 `chat_id/message_id`。匿名来源或个人来源没有稳定
/// message_id 时不返回，避免生成伪源标识。
///
/// # 参数
/// - `message`: 消息对象引用
///
/// # 返回值
/// - `Option<(i64, i64)>`: 转发来源原始 (chat_id, message_id)
fn forwarded_message_location(message: &tdlib_rs::types::Message) -> Option<(i64, i64)> {
    let forward = message.forward_info.as_ref()?;
    if let Some(source) = &forward.source
        && source.chat_id != 0
        && source.message_id != 0
    {
        return Some((source.chat_id, source.message_id));
    }

    match &forward.origin {
        tdlib_rs::enums::MessageOrigin::Channel(channel)
            if channel.chat_id != 0 && channel.message_id != 0 =>
        {
            Some((channel.chat_id, channel.message_id))
        }
        _ => None,
    }
}

/// 对外提供统一的“媒体消息源定位”提取。
///
/// 优先级：
/// 1. reply_to 指向的原消息
/// 2. forwarded 来源里可还原的原始 chat/message
/// 3. 当前 bot 可见消息本身
///
/// 特殊规则：
/// - 如果消息本身是 forwarded，但转发来源无法还原稳定 message_id，则返回 None，交给上层提示用户
///   改用消息链接或回复 bot 可见媒体，避免把“转发壳消息”的新 message_id 当成伪源。
///
/// # 参数
/// - `message`: 待提取的消息对象引用
///
/// # 返回值
/// - `Option<(i64, i64)>`: 解析得到的稳定媒体源 (chat_id, message_id)
pub(in crate::tgbot) fn transferable_message_source_location(
    message: &tdlib_rs::types::Message,
) -> Option<(i64, i64)> {
    if let Some(reply) = replied_message_location(message) {
        return Some(reply);
    }

    if message.forward_info.is_some() {
        return forwarded_message_location(message);
    }

    Some((message.chat_id, message.id))
}

/// 构造 `/transfer` 首次回执卡片。
///
/// 后台任务启动后会持续编辑同一条消息，因此初始卡片也使用 card 格式，避免样式闪变。
///
/// # 参数
/// - `plan`: 转存计划引用
///
/// # 返回值
/// - `String`: 格式化后的回执卡片正文
fn format_transfer_accepted_text(plan: &TransferPlan) -> String {
    [
        "已接收转存请求".to_owned(),
        card::status_target("queued", plan.target_chat_id),
        card::DIVIDER.to_owned(),
        card::section("进度"),
        "后台会自动下载并上传，本消息会持续刷新。".to_owned(),
        card::note("可直接点击下方按钮查看进度；需要命令时点击“查看命令”。"),
        String::new(),
    ]
    .into_iter()
    .chain(card::source_block(&plan.source_link))
    .collect::<Vec<_>>()
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::{
        RETRANSFER_CALLBACK_DATA, ResolvedTransferSource, bot_message_source_link,
        build_retransfer_confirm_button_rows, build_transfer_accepted_button_rows,
        format_transfer_accepted_text, forwarded_message_location, resolve_transfer_target_chat_id,
    };
    use crate::ClientRole;
    use crate::app_context::app_context;
    use crate::config::{BotConfig, RequestActor};
    use crate::tgbot::transfer::types::{SourceKind, TransferPlan};
    use base64::{Engine as _, engine::general_purpose};

    fn install_target_runtime(targets: crate::config::TargetsConfig) {
        let app = app_context();
        app.targets_runtime.update_runtime_config(targets);
    }

    /// 测试首次转存回执卡片文本格式包含排队状态、目标频道及源链接代码块。
    #[test]
    fn test_format_transfer_accepted_text() {
        let text = format_transfer_accepted_text(&TransferPlan {
            actor: RequestActor {
                request_chat_id: 1,
                user_id: 1,
            },
            source_link: "https://t.me/c/1/2".to_owned(),
            source_kind: SourceKind::Link,
            preferred_source_client_role: ClientRole::Bot,
            allow_user_fallback: true,
            source_message_chat_id: None,
            source_message_id: None,
            target_chat_id: -100,
            request_chat_id: 1,
            request_message_id: 2,
            force_retransfer: false,
        });

        assert!(text.contains("状态：‹queued›"));
        assert!(text.contains("目标：‹-100›"));
        assert!(text.contains("‹https://t.me/c/1/2›"));
    }

    /// 测试首次回执按钮矩阵包含运行列表、查看命令和菜单跳转按钮。
    #[test]
    fn test_build_transfer_accepted_button_rows() {
        let rows = build_transfer_accepted_button_rows();
        let labels = rows
            .iter()
            .flatten()
            .map(|button| button.text.as_str())
            .collect::<Vec<_>>();

        assert_eq!(rows[0][0].text, "查看运行列表");
        assert_eq!(rows[0][1].text, "查看命令");
        assert_eq!(rows[0][2].text, "菜单");
        assert_eq!(rows.len(), 1);
        assert!(!labels.contains(&"复制查询命令"));
        assert!(!labels.contains(&"复制源标识"));
        assert!(matches!(
            rows[0][0].r#type,
            tdlib_rs::enums::InlineKeyboardButtonType::Callback(_)
        ));
    }

    /// 测试再次转存确认卡片按钮包含打开原结果链接与再次转存确认按钮。
    #[test]
    fn test_retransfer_confirm_buttons_include_old_result_and_confirmation() {
        let rows = build_retransfer_confirm_button_rows("https://t.me/c/123/456");

        assert_eq!(rows[0][0].text, "打开原结果");
        assert_eq!(rows[1][0].text, "再次转存");
        assert_eq!(rows[2][0].text, "取消");
        assert_eq!(decoded_callback_data(&rows[1][0]), RETRANSFER_CALLBACK_DATA);
    }

    /// 测试当历史结果为非可打开 URL 的定位文本时，不展示打开链接按钮。
    #[test]
    fn test_retransfer_confirm_buttons_keep_locator_as_text_only() {
        let rows = build_retransfer_confirm_button_rows("chat_id=-100 message_id=456");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0].text, "再次转存");
    }

    /// 解析内联按钮中的 base64 回调数据字符串。
    fn decoded_callback_data(button: &tdlib_rs::types::InlineKeyboardButton) -> String {
        let tdlib_rs::enums::InlineKeyboardButtonType::Callback(callback) = &button.r#type else {
            panic!("button must be callback");
        };
        String::from_utf8(general_purpose::STANDARD.decode(&callback.data).unwrap()).unwrap()
    }

    /// 测试生成稳定的 Bot 可见消息虚拟源定位标识。
    #[test]
    fn test_bot_message_source_link() {
        assert_eq!(bot_message_source_link(100, 200), "bot-message:100:200");
    }

    /// 测试转发消息定位提取优先选用 forward_source。
    #[test]
    fn test_forwarded_message_location_prefers_forward_source() {
        let message = tdlib_rs::types::Message {
            id: 1,
            sender_id: tdlib_rs::enums::MessageSender::User(Box::new(
                tdlib_rs::types::MessageSenderUser { user_id: 1 },
            )),
            chat_id: 1000,
            can_be_saved: true,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::Channel(Box::new(
                    tdlib_rs::types::MessageOriginChannel {
                        chat_id: -2000,
                        message_id: 88,
                        author_signature: String::new(),
                    },
                )),
                date: 0,
                source: Some(tdlib_rs::types::ForwardSource {
                    chat_id: -3000,
                    message_id: 99,
                    sender_id: None,
                    sender_name: String::new(),
                    date: 0,
                    is_outgoing: false,
                }),
                public_service_announcement_type: String::new(),
            }),
            content: tdlib_rs::enums::MessageContent::MessageText(Box::new(
                tdlib_rs::types::MessageText {
                    text: tdlib_rs::types::FormattedText {
                        text: "test".to_owned(),
                        entities: vec![],
                    },
                    link_preview: None,
                    link_preview_options: None,
                },
            )),
            ..crate::tgbot::mock_message()
        };

        assert_eq!(forwarded_message_location(&message), Some((-3000, 99)));
    }

    /// 测试在无 forward_source 时转发消息定位回退至频道原始信息 channel origin。
    #[test]
    fn test_forwarded_message_location_falls_back_to_channel_origin() {
        let message = tdlib_rs::types::Message {
            id: 1,
            sender_id: tdlib_rs::enums::MessageSender::User(Box::new(
                tdlib_rs::types::MessageSenderUser { user_id: 1 },
            )),
            chat_id: 1000,
            can_be_saved: true,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::Channel(Box::new(
                    tdlib_rs::types::MessageOriginChannel {
                        chat_id: -2000,
                        message_id: 88,
                        author_signature: String::new(),
                    },
                )),
                date: 0,
                source: None,
                public_service_announcement_type: String::new(),
            }),
            content: tdlib_rs::enums::MessageContent::MessageText(Box::new(
                tdlib_rs::types::MessageText {
                    text: tdlib_rs::types::FormattedText {
                        text: "test".to_owned(),
                        entities: vec![],
                    },
                    link_preview: None,
                    link_preview_options: None,
                },
            )),
            ..crate::tgbot::mock_message()
        };

        assert_eq!(forwarded_message_location(&message), Some((-2000, 88)));
    }

    /// 测试转发消息来自不稳定来源（如普通用户转发）时拒绝解析并返回 None。
    #[test]
    fn test_forwarded_message_location_rejects_unstable_origin() {
        let message = tdlib_rs::types::Message {
            id: 1,
            sender_id: tdlib_rs::enums::MessageSender::User(Box::new(
                tdlib_rs::types::MessageSenderUser { user_id: 1 },
            )),
            chat_id: 1000,
            can_be_saved: true,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::User(Box::new(
                    tdlib_rs::types::MessageOriginUser { sender_user_id: 42 },
                )),
                date: 0,
                source: None,
                public_service_announcement_type: String::new(),
            }),
            content: tdlib_rs::enums::MessageContent::MessageText(Box::new(
                tdlib_rs::types::MessageText {
                    text: tdlib_rs::types::FormattedText {
                        text: "test".to_owned(),
                        entities: vec![],
                    },
                    link_preview: None,
                    link_preview_options: None,
                },
            )),
            ..crate::tgbot::mock_message()
        };

        assert_eq!(forwarded_message_location(&message), None);
    }

    /// 测试媒体消息源定位提取优先级：优先 reply，其次 forward，最后自身消息。
    #[test]
    fn test_transferable_message_source_location_prefers_reply_then_forward_then_self() {
        let base_message = tdlib_rs::types::Message {
            id: 7,
            sender_id: tdlib_rs::enums::MessageSender::User(Box::new(
                tdlib_rs::types::MessageSenderUser { user_id: 1 },
            )),
            chat_id: 1000,
            can_be_saved: true,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::Channel(Box::new(
                    tdlib_rs::types::MessageOriginChannel {
                        chat_id: -2000,
                        message_id: 88,
                        author_signature: String::new(),
                    },
                )),
                date: 0,
                source: Some(tdlib_rs::types::ForwardSource {
                    chat_id: -3000,
                    message_id: 99,
                    sender_id: None,
                    sender_name: String::new(),
                    date: 0,
                    is_outgoing: false,
                }),
                public_service_announcement_type: String::new(),
            }),
            content: tdlib_rs::enums::MessageContent::MessageText(Box::new(
                tdlib_rs::types::MessageText {
                    text: tdlib_rs::types::FormattedText {
                        text: "test".to_owned(),
                        entities: vec![],
                    },
                    link_preview: None,
                    link_preview_options: None,
                },
            )),
            ..crate::tgbot::mock_message()
        };

        let mut replied = base_message.clone();
        replied.reply_to = Some(tdlib_rs::enums::MessageReplyTo::Message(Box::new(
            tdlib_rs::types::MessageReplyToMessage {
                chat_id: -4000,
                message_id: 66,
                quote: None,
                checklist_task_id: 0,
                poll_option_id: String::new(),
                origin: None,
                origin_send_date: 0,
                content: None,
            },
        )));
        assert_eq!(
            super::transferable_message_source_location(&replied),
            Some((-4000, 66))
        );

        let forwarded = base_message.clone();
        assert_eq!(
            super::transferable_message_source_location(&forwarded),
            Some((-3000, 99))
        );

        let mut self_only = base_message;
        self_only.forward_info = None;
        assert_eq!(
            super::transferable_message_source_location(&self_only),
            Some((1000, 7))
        );
    }

    /// 测试媒体消息源定位提取拒绝不稳定转发外壳消息。
    #[test]
    fn test_transferable_message_source_location_rejects_unstable_forward_shell() {
        let message = tdlib_rs::types::Message {
            id: 7,
            sender_id: tdlib_rs::enums::MessageSender::User(Box::new(
                tdlib_rs::types::MessageSenderUser { user_id: 1 },
            )),
            chat_id: 1000,
            can_be_saved: true,
            forward_info: Some(tdlib_rs::types::MessageForwardInfo {
                origin: tdlib_rs::enums::MessageOrigin::User(Box::new(
                    tdlib_rs::types::MessageOriginUser { sender_user_id: 42 },
                )),
                date: 0,
                source: None,
                public_service_announcement_type: String::new(),
            }),
            content: tdlib_rs::enums::MessageContent::MessageText(Box::new(
                tdlib_rs::types::MessageText {
                    text: tdlib_rs::types::FormattedText {
                        text: "test".to_owned(),
                        entities: vec![],
                    },
                    link_preview: None,
                    link_preview_options: None,
                },
            )),
            ..crate::tgbot::mock_message()
        };

        assert_eq!(super::transferable_message_source_location(&message), None);
    }

    /// 测试回复媒体模式下第二个参数正确解析为目标别名 target。
    #[test]
    fn test_resolve_target_for_bot_message_source() {
        let config = BotConfig::default();
        install_target_runtime(crate::config::TargetsConfig {
            default_chat_id: 0,
            aliases: std::collections::HashMap::from([("archive".to_owned(), -100)]),
        });
        let source = ResolvedTransferSource {
            source_link: bot_message_source_link(10, 20),
            source_kind: SourceKind::BotMessage,
            preferred_source_client_role: ClientRole::Bot,
            source_message_chat_id: Some(10),
            source_message_id: Some(20),
        };

        let target = resolve_transfer_target_chat_id(&["/t", "archive"], &source, &config, 1)
            .expect("alias target should resolve");

        assert_eq!(target, -100);
    }
}
