// 转存回复卡片文本构造工具。
// 这里输出的是“卡片标记文本”，发送层会转换成 TDLib 原生 FormattedText 实体。

/// 卡片分隔线，使用短线保持移动端可读，不占太多宽度。
pub(in crate::tgbot) const DIVIDER: &str = "━━━━━━━━━━━━";
/// 文本进度条固定长度。
///
/// 固定 20 格可以在移动端保持紧凑，同时足够表达 5% 粒度。
const PROGRESS_BAR_WIDTH: i64 = 20;

/// 构造行内代码字段。
/// 发送层会把 `‹...›` 解析成 TDLib `textEntityTypeCode`。
pub(in crate::tgbot) fn code(value: impl std::fmt::Display) -> String {
    format!("‹{}›", escape_marker_text(value.to_string()))
}

/// 构造多行等宽代码块。
/// 发送层会把 `«... »` 解析成 TDLib `textEntityTypePreCode`，适合错误详情。
pub(in crate::tgbot) fn pre_code(value: impl std::fmt::Display) -> String {
    format!("«{}»", escape_pre_code_text(value.to_string()))
}

/// 构造原生文本链接。
/// 发送层会把 `【文本】(url)` 解析成 TDLib `textEntityTypeTextUrl`。
pub(in crate::tgbot) fn link(label: &str, url: &str) -> String {
    format!(
        "【{}】({})",
        escape_marker_text(label),
        escape_link_url(url)
    )
}

/// 构造一个分区标题。
pub(in crate::tgbot) fn section(title: &str) -> String {
    format!("■ {title}")
}

/// 构造普通说明行。
///
/// 说明文字不使用 code 实体，保留自然换行和可读性。
pub(in crate::tgbot) fn note(value: &str) -> String {
    format!("说明：{value}")
}

/// 构造单个 `label：value` 字段。
///
/// 卡片正文大量使用短字段；统一入口可以避免不同模块出现半角冒号、空格等展示差异。
pub(in crate::tgbot) fn field(label: &str, value: impl std::fmt::Display) -> String {
    format!("{}：{}", label, code(value))
}

/// 构造同一行上的两个字段，适合状态/目标、进度/更新时间这类高频摘要。
pub(in crate::tgbot) fn field_pair(
    left_label: &str,
    left_value: impl std::fmt::Display,
    right_label: &str,
    right_value: impl std::fmt::Display,
) -> String {
    format!(
        "{}：{}  {}：{}",
        left_label,
        code(left_value),
        right_label,
        code(right_value)
    )
}

/// 构造三段式状态摘要。
///
/// 用于等待、进度、结果卡片的第二行，保证用户第一眼能看到状态、job 和目标。
pub(in crate::tgbot) fn summary_line(
    status: &str,
    job_id: Option<i64>,
    target_chat_id: i64,
) -> String {
    match job_id {
        Some(job_id) => status_job_target(status, job_id, target_chat_id),
        None => status_target(status, target_chat_id),
    }
}

/// 构造状态 + 目标行。
pub(in crate::tgbot) fn status_target(status: &str, target_chat_id: i64) -> String {
    format!("状态：{}  目标：{}", code(status), code(target_chat_id))
}

/// 构造状态 + job + 目标行。
pub(in crate::tgbot) fn status_job_target(
    status: &str,
    job_id: i64,
    target_chat_id: i64,
) -> String {
    format!(
        "状态：{}  job：{}  目标：{}",
        code(status),
        job_ref(job_id),
        code(target_chat_id)
    )
}

/// 构造 job 引用字段，统一展示为 `#id`。
pub(in crate::tgbot) fn job_ref(job_id: i64) -> String {
    code(format!("#{job_id}"))
}

/// 构造固定宽度文本进度条。
///
/// 输出示例：`||||||||||---------- 50%`。使用 ASCII 是为了避免不同 Telegram 客户端对 Unicode 宽度渲染不一致。
pub(in crate::tgbot) fn progress_bar(done: i64, total: i64) -> String {
    if total <= 0 {
        return "||||................ --".to_owned();
    }

    let percent = done
        .max(0)
        .saturating_mul(100)
        .saturating_div(total.max(1))
        .clamp(0, 100);
    progress_bar_percent(percent)
}

/// 按百分比构造固定宽度文本进度条。
pub(in crate::tgbot) fn progress_bar_percent(percent: impl TryInto<i64>) -> String {
    let percent = percent.try_into().unwrap_or(0).clamp(0, 100);
    let filled = (percent * PROGRESS_BAR_WIDTH + 50) / 100;
    let empty = PROGRESS_BAR_WIDTH - filled;
    format!(
        "{}{} {}%",
        "|".repeat(filled as usize),
        "-".repeat(empty as usize),
        percent
    )
}

/// 构造来源链接分区。
pub(in crate::tgbot) fn source_block(source_link: &str) -> Vec<String> {
    vec![section("来源"), code(source_link)]
}

/// 构造源链接分区。
///
/// 进度面板里标题已经是“转存进度”，这里用更短的“源链接”避免重复出现“来源/源链接”两层标题。
pub(in crate::tgbot) fn source_link_block(source_link: &str) -> Vec<String> {
    vec![section("源链接"), code(source_link)]
}

/// 构造结果链接分区。
///
/// HTTP(S) 结果会同时给出原生文本链接和可复制的明文链接；不可打开的定位只展示为代码字段。
pub(in crate::tgbot) fn result_block(result_link: &str) -> String {
    if crate::tgbot::send::is_openable_url(result_link) {
        return format!(
            "{}\n{}\n链接：{}",
            section("结果"),
            link("打开转存消息", result_link),
            code(result_link)
        );
    }

    format!(
        "{}\n说明：当前 chat 不提供独立 URL，请通过结果通知的消息引用跳转\n定位：{}",
        section("结果"),
        code(result_link)
    )
}

/// 清理卡片标记字段里的保留字符。
///
/// 用户输入可能包含 `‹›【】` 这些标记符；直接拼进去会破坏发送层实体解析。
fn escape_marker_text(value: impl AsRef<str>) -> String {
    value
        .as_ref()
        .replace('‹', "<")
        .replace('›', ">")
        .replace('【', "[")
        .replace('】', "]")
}

/// 清理代码块字段里的保留结束符，避免错误文本截断卡片解析。
fn escape_pre_code_text(value: impl AsRef<str>) -> String {
    value.as_ref().replace('»', ">")
}

/// 清理文本链接 URL 里的右括号，避免提前结束 `【label】(url)` 标记。
fn escape_link_url(value: &str) -> String {
    value.replace(')', "%29")
}

#[cfg(test)]
mod tests {
    use super::{
        code, field, field_pair, job_ref, link, note, pre_code, progress_bar, progress_bar_percent,
        result_block, section, source_block, status_job_target, summary_line,
    };

    // 卡片标记应保持简单稳定，供发送层解析为 TDLib 实体。
    #[test]
    fn test_card_markers() {
        assert_eq!(code("success"), "‹success›");
        assert_eq!(section("结果"), "■ 结果");
        assert_eq!(note("等待刷新。"), "说明：等待刷新。");
        assert_eq!(job_ref(42), "‹#42›");
        assert_eq!(pre_code("line1\nline2"), "«line1\nline2»");
        assert_eq!(progress_bar(5, 10), "||||||||||---------- 50%");
        assert_eq!(progress_bar_percent(84), "|||||||||||||||||--- 84%");
        assert_eq!(
            link("打开", "https://t.me/c/1/2"),
            "【打开】(https://t.me/c/1/2)"
        );
    }

    // 用户输入里的卡片保留字符不应破坏发送层解析。
    #[test]
    fn test_card_markers_escape_user_value() {
        assert_eq!(code("a‹b›c【d】"), "‹a<b>c[d]›");
        assert_eq!(pre_code("a»b"), "«a>b»");
        assert_eq!(
            link("【打开】", "https://example.com/a)"),
            "【[打开]】(https://example.com/a%29)"
        );
    }

    // 常用卡片行保持统一，避免各命令模块展示不一致。
    #[test]
    fn test_card_common_blocks() {
        assert_eq!(
            status_job_target("running", 42, -100),
            "状态：‹running›  job：‹#42›  目标：‹-100›"
        );
        assert_eq!(
            source_block("https://t.me/c/1/2"),
            vec!["■ 来源", "‹https://t.me/c/1/2›"]
        );
        assert_eq!(field("目标", -100), "目标：‹-100›");
        assert_eq!(
            field_pair("进度", "1/2", "更新", "10:00"),
            "进度：‹1/2›  更新：‹10:00›"
        );
        assert_eq!(
            summary_line("running", Some(42), -100),
            "状态：‹running›  job：‹#42›  目标：‹-100›"
        );
    }

    // 结果块只给 HTTP(S) 链接生成可点击实体标记。
    #[test]
    fn test_result_block_openable_and_locator() {
        assert!(result_block("https://t.me/c/1/2").contains("【打开转存消息】("));
        assert!(!result_block("chat_id=-1 message_id=2").contains("【打开转存消息】("));
    }
}
