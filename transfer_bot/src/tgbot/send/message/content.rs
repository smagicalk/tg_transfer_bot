// `FormattedText` 构造工具。
// Telegram 的实体 offset/length 使用 UTF-16 单位，这里统一处理，避免调用方重复踩坑。

use crate::tgbot::TdError;

/// 卡片字段值标记起始符。
/// 发送前会被转换成 TDLib `textEntityTypeCode`，不会把标记本身发给用户。
pub const CARD_CODE_START: char = '‹';
/// 卡片字段值标记结束符。
pub const CARD_CODE_END: char = '›';
/// 卡片链接标记起始符，语法：`【文本】(url)`。
pub const CARD_LINK_TEXT_START: char = '【';
/// 卡片链接标记结束符，语法：`【文本】(url)`。
pub const CARD_LINK_TEXT_END: char = '】';

/// 构造 TDLib 文本消息内容。
pub(in crate::tgbot::send::message) fn build_text_input_message_content(
    text: tdlib_rs::types::FormattedText,
) -> tdlib_rs::enums::InputMessageContent {
    tdlib_rs::enums::InputMessageContent::InputMessageText(tdlib_rs::types::InputMessageText {
        text,
        link_preview_options: None,
        clear_draft: true,
    })
}

/// 构造不带实体的普通文本。
pub(in crate::tgbot::send::message) fn build_plain_formatted_text(
    text: String,
) -> tdlib_rs::types::FormattedText {
    tdlib_rs::types::FormattedText {
        text,
        entities: vec![],
    }
}

/// 解析 Markdown，转成 Telegram 原生 `FormattedText`。
pub(in crate::tgbot::send::message) async fn parse_markdown_text(
    text: String,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::FormattedText> {
    let parsed = tdlib_rs::functions::parse_text_entities(
        text,
        tdlib_rs::enums::TextParseMode::Markdown(tdlib_rs::types::TextParseModeMarkdown {
            // 现有文案使用 Bot API Markdown v1 风格：`*bold*`、`code`、`[text](url)`。
            version: 1,
        }),
        client_id,
    )
    .await
    .map_err(|e| anyhow::Error::new(TdError(e)))?;
    let tdlib_rs::enums::FormattedText::FormattedText(formatted_text) = parsed;
    Ok(formatted_text)
}

/// 构造卡片风格 `FormattedText`。
///
/// 这比 Markdown 更适合机器人固定回复：
/// - 第一行标题和以 `■` 开头的分区标题会加粗；
/// - `‹...›` 会变成行内代码，适合状态、ID、命令；
/// - `【文本】(url)` 会变成原生可点击链接；
/// - 用户输入只作为普通文本拼进去，不会破坏实体边界。
pub(in crate::tgbot::send::message) fn build_card_formatted_text(
    source: String,
) -> anyhow::Result<tdlib_rs::types::FormattedText> {
    let mut builder = FormattedTextBuilder::default();
    let mut line_start = true;
    let mut first_line = true;
    let mut line_started = false;
    let mut line_bold = false;
    let mut line_start_offset = 0;
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        if line_start {
            // 标题行仍然按正常规则解析 code/link，最后再叠加 Bold 实体。
            line_start_offset = builder.current_offset()?;
            line_bold = first_line || ch == '■';
            line_started = true;
            line_start = false;
        }

        if ch == CARD_CODE_START {
            let mut probe = chars.clone();
            if let Some(value) = take_until_required(&mut probe, CARD_CODE_END) {
                chars = probe;
                builder.push_entity_text(value, tdlib_rs::enums::TextEntityType::Code)?;
                continue;
            }
        }

        if ch == CARD_LINK_TEXT_START {
            let mut probe = chars.clone();
            if let Some((label, url)) = take_card_link(&mut probe) {
                chars = probe;
                builder.push_entity_text(
                    label,
                    tdlib_rs::enums::TextEntityType::TextUrl(
                        tdlib_rs::types::TextEntityTypeTextUrl { url },
                    ),
                )?;
                continue;
            }
        }

        if ch == '\n' {
            if line_bold {
                builder.push_entity_range(
                    line_start_offset,
                    builder.current_offset()?,
                    tdlib_rs::enums::TextEntityType::Bold,
                );
            }
            builder.push_char(ch);
            line_start = true;
            line_started = false;
            line_bold = false;
            first_line = false;
            continue;
        }

        builder.push_char(ch);
    }

    if line_started && line_bold {
        builder.push_entity_range(
            line_start_offset,
            builder.current_offset()?,
            tdlib_rs::enums::TextEntityType::Bold,
        );
    }

    builder.into_formatted_text()
}

/// 构造整段可复制的等宽文本。
/// TDLib 的 `offset` 和 `length` 均按 UTF-16 code unit 计算。
pub(in crate::tgbot::send::message) fn build_copyable_formatted_text(
    text: String,
) -> anyhow::Result<tdlib_rs::types::FormattedText> {
    let length = i32::try_from(text.encode_utf16().count())
        .map_err(|_| anyhow::anyhow!("message too long"))?;

    Ok(tdlib_rs::types::FormattedText {
        text,
        entities: vec![tdlib_rs::types::TextEntity {
            offset: 0,
            length,
            r#type: tdlib_rs::enums::TextEntityType::PreCode(
                tdlib_rs::types::TextEntityTypePreCode {
                    language: "".to_owned(),
                },
            ),
        }],
    })
}

/// `FormattedText` 构建器，统一维护 UTF-16 offset/length。
#[derive(Default)]
struct FormattedTextBuilder {
    text: String,
    entities: Vec<tdlib_rs::types::TextEntity>,
}

impl FormattedTextBuilder {
    /// 追加一个普通字符。
    fn push_char(&mut self, ch: char) {
        self.text.push(ch);
    }

    /// 返回当前文本尾部的 UTF-16 offset。
    fn current_offset(&self) -> anyhow::Result<i32> {
        i32::try_from(self.text.encode_utf16().count())
            .map_err(|_| anyhow::anyhow!("message too long"))
    }

    /// 追加一段带实体的文本。
    fn push_entity_text(
        &mut self,
        value: String,
        r#type: tdlib_rs::enums::TextEntityType,
    ) -> anyhow::Result<()> {
        if value.is_empty() {
            return Ok(());
        }
        let offset = self.current_offset()?;
        let length = i32::try_from(value.encode_utf16().count())
            .map_err(|_| anyhow::anyhow!("message too long"))?;
        self.text.push_str(&value);
        self.push_entity_range(offset, offset + length, r#type);
        Ok(())
    }

    /// 为已经追加的文本范围补充实体。
    fn push_entity_range(&mut self, start: i32, end: i32, r#type: tdlib_rs::enums::TextEntityType) {
        let length = end.saturating_sub(start);
        if length <= 0 {
            return;
        }
        self.entities.push(tdlib_rs::types::TextEntity {
            offset: start,
            length,
            r#type,
        });
    }

    /// 输出最终 TDLib `FormattedText`。
    fn into_formatted_text(mut self) -> anyhow::Result<tdlib_rs::types::FormattedText> {
        let _ = i32::try_from(self.text.encode_utf16().count())
            .map_err(|_| anyhow::anyhow!("message too long"))?;
        // TDLib 可以处理嵌套实体；排序后更便于测试和排查。
        self.entities
            .sort_by_key(|entity| (entity.offset, -entity.length));
        Ok(tdlib_rs::types::FormattedText {
            text: self.text,
            entities: self.entities,
        })
    }
}

/// 尝试读取卡片链接标记：`【文本】(url)`。
fn take_card_link(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Option<(String, String)> {
    let label = take_until_required(chars, CARD_LINK_TEXT_END)?;
    if chars.next()? != '(' {
        return None;
    }
    let url = take_until_required(chars, ')')?;
    Some((label, url))
}

/// 读取到指定结束符为止；结束符不存在时返回 `None`。
fn take_until_required(
    chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    end: char,
) -> Option<String> {
    let mut value = String::new();
    for ch in chars.by_ref() {
        if ch == end {
            return Some(value);
        }
        value.push(ch);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::build_card_formatted_text;

    // 卡片文本应被转换成 TDLib 原生实体，而不是依赖 Markdown。
    #[test]
    fn test_build_card_formatted_text_entities() {
        let text =
            "转存完成\n■ 结果\n状态：‹success›\n【打开转存消息】(https://t.me/c/1/2)".to_owned();
        let formatted = build_card_formatted_text(text).expect("card text should parse");

        assert_eq!(
            formatted.text,
            "转存完成\n■ 结果\n状态：success\n打开转存消息"
        );
        let bold_count = formatted
            .entities
            .iter()
            .filter(|entity| matches!(entity.r#type, tdlib_rs::enums::TextEntityType::Bold))
            .count();
        assert_eq!(bold_count, 2);
        assert!(
            formatted
                .entities
                .iter()
                .any(|entity| { matches!(entity.r#type, tdlib_rs::enums::TextEntityType::Bold) })
        );
        assert!(
            formatted
                .entities
                .iter()
                .any(|entity| { matches!(entity.r#type, tdlib_rs::enums::TextEntityType::Code) })
        );
        assert!(formatted.entities.iter().any(|entity| {
            matches!(entity.r#type, tdlib_rs::enums::TextEntityType::TextUrl(_))
        }));
    }

    // 不完整链接标记要按普通文本保留，不能吞掉用户输入。
    #[test]
    fn test_build_card_formatted_text_keeps_broken_link_marker() {
        let text = "提示\n【打开】(https://example.com".to_owned();
        let formatted = build_card_formatted_text(text).expect("card text should parse");

        assert_eq!(formatted.text, "提示\n【打开】(https://example.com");
        assert!(!formatted.entities.iter().any(|entity| {
            matches!(entity.r#type, tdlib_rs::enums::TextEntityType::TextUrl(_))
        }));
    }

    // 标题行可以同时加粗并包含 code/link 实体，用户不应看到卡片标记符。
    #[test]
    fn test_build_card_formatted_text_supports_nested_title_entities() {
        let text = "转存进度 ‹#42›\n■ 结果：‹ok›".to_owned();
        let formatted = build_card_formatted_text(text).expect("card text should parse");

        assert_eq!(formatted.text, "转存进度 #42\n■ 结果：ok");
        let bold_count = formatted
            .entities
            .iter()
            .filter(|entity| matches!(entity.r#type, tdlib_rs::enums::TextEntityType::Bold))
            .count();
        let code_count = formatted
            .entities
            .iter()
            .filter(|entity| matches!(entity.r#type, tdlib_rs::enums::TextEntityType::Code))
            .count();
        assert_eq!(bold_count, 2);
        assert_eq!(code_count, 2);
    }

    // 末尾换行不应让最后一行的 Bold 实体重复追加。
    #[test]
    fn test_build_card_formatted_text_trailing_newline_does_not_duplicate_bold() {
        let text = "标题\n".to_owned();
        let formatted = build_card_formatted_text(text).expect("card text should parse");
        let bold_count = formatted
            .entities
            .iter()
            .filter(|entity| matches!(entity.r#type, tdlib_rs::enums::TextEntityType::Bold))
            .count();

        assert_eq!(formatted.text, "标题\n");
        assert_eq!(bold_count, 1);
    }
}
