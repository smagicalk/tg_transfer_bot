// `FormattedText` 构造工具。
// Telegram 的实体 offset/length 使用 UTF-16 单位，这里统一处理，避免调用方重复踩坑。

use crate::tgbot::TdError;

/// 卡片字段值标记起始符（‹）。
/// 发送前会被转换成 TDLib `textEntityTypeCode` 行内等宽代码实体，不会把标记符号本身发给用户。
pub const CARD_CODE_START: char = '‹';

/// 卡片字段值标记结束符（›）。
pub const CARD_CODE_END: char = '›';

/// 卡片超链接标记起始符（【），卡片内嵌超链接语法：`【文本】(url)`。
pub const CARD_LINK_TEXT_START: char = '【';

/// 卡片超链接标记结束符（】）。
pub const CARD_LINK_TEXT_END: char = '】';

/// 卡片多行预格式化代码块起始符（«）。
const CARD_PRE_CODE_START: char = '«';

/// 卡片多行预格式化代码块结束符（»）。
const CARD_PRE_CODE_END: char = '»';

/// 构造不带任何富文本实体的纯文本对象。
///
/// 直接包裹 `text`，`entities` 列表置空。
///
/// # 参数
/// - `text`: 要包装的纯字符串。
///
/// # 返回值
/// - `tdlib_rs::types::FormattedText`: 格式化文本结构体。
pub(in crate::tgbot::send::message) fn build_plain_formatted_text(
    text: String,
) -> tdlib_rs::types::FormattedText {
    tdlib_rs::types::FormattedText {
        text,
        entities: vec![],
    }
}

/// 解析 Markdown 文本，转成 Telegram 原生 `FormattedText`。
///
/// 内部调用 TDLib 的 `parse_text_entities` API，采用 Telegram Bot API Markdown v1 模式：
/// - `*bold*` -> 粗体
/// - `` `code` `` -> 行内代码
/// - `[text](url)` -> 超链接
///
/// # 参数
/// - `text`: 包含 Markdown 标记的源文本。
/// - `client_id`: TDLib 客户端实例 ID。
///
/// # 返回值
/// - `Ok(FormattedText)`: TDLib 生成的富文本及实体偏移数组。
/// - `Err(anyhow::Error)`: 解析失败或客户端调用异常。
pub(in crate::tgbot::send::message) async fn parse_markdown_text(
    text: String,
    client_id: i32,
) -> anyhow::Result<tdlib_rs::types::FormattedText> {
    let parsed = tdlib_rs::functions::parse_text_entities(
        text,
        tdlib_rs::enums::TextParseMode::Markdown(Box::new(
            tdlib_rs::types::TextParseModeMarkdown {
                // 现有文案使用 Bot API Markdown v1 风格：`*bold*`、`code`、`[text](url)`。
                version: 1,
            },
        )),
        client_id,
    )
    .await
    .map_err(|e| anyhow::Error::new(TdError(e)))?;
    let tdlib_rs::enums::FormattedText::FormattedText(formatted_text) = parsed;
    Ok(*formatted_text)
}

/// 构造卡片风格的 `FormattedText` 富文本。
///
/// 这比 Markdown 更适合机器人固定回复与交互式卡片：
/// - 首行标题自动加粗；
/// - 以 `■` 开头的段落标题行自动加粗；
/// - `‹...›` 被解析为行内代码实体（`Code`），适用于展示 ID、文件哈希、命令名；
/// - `«...»` 被解析为预格式化代码块（`PreCode`），适用于展示多行日志、错误堆栈；
/// - `【文本】(url)` 被解析为超链接实体（`TextUrl`）；
/// - 用户原始输入字符原样插入正文，不会因为特殊符号破坏 Markdown 解析树。
///
/// # 参数
/// - `source`: 包含卡片标记的源文本。
///
/// # 返回值
/// - `Ok(FormattedText)`: 构造好的 Telegram 富文本。
/// - `Err(anyhow::Error)`: 如果字符串长度超出 UTF-16 上限等异常情况。
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

        if ch == CARD_PRE_CODE_START {
            let mut probe = chars.clone();
            if let Some(value) = take_until_required(&mut probe, CARD_PRE_CODE_END) {
                chars = probe;
                builder.push_entity_text(
                    value,
                    tdlib_rs::enums::TextEntityType::PreCode(Box::new(
                        tdlib_rs::types::TextEntityTypePreCode {
                            language: "".to_owned(),
                        },
                    )),
                )?;
                continue;
            }
        }

        if ch == CARD_LINK_TEXT_START {
            let mut probe = chars.clone();
            if let Some((label, url)) = take_card_link(&mut probe) {
                chars = probe;
                builder.push_entity_text(
                    label,
                    tdlib_rs::enums::TextEntityType::TextUrl(Box::new(
                        tdlib_rs::types::TextEntityTypeTextUrl { url },
                    )),
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
///
/// 使用 TDLib 的 `TextEntityType::PreCode` 实体将全文包裹为一个代码块，
/// 允许用户点击后一键复制。注意：TDLib 的 `offset` 和 `length` 均按 UTF-16 code unit 计数。
///
/// # 参数
/// - `text`: 待包裹的原始字符串。
///
/// # 返回值
/// - `Ok(FormattedText)`: 包含单个 PreCode 实体的格式化文本。
/// - `Err(anyhow::Error)`: 文本长度超过 i32 范围时报错。
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
            r#type: tdlib_rs::enums::TextEntityType::PreCode(Box::new(
                tdlib_rs::types::TextEntityTypePreCode {
                    language: "".to_owned(),
                },
            )),
        }],
    })
}

/// `FormattedText` 构建器，负责在字符流扫描中累积文本并统一维护 UTF-16 的 offset 与 length。
#[derive(Default)]
struct FormattedTextBuilder {
    /// 累积生成的纯文本字符串。
    text: String,
    /// 识别并累积的富文本格式化实体列表。
    entities: Vec<tdlib_rs::types::TextEntity>,
}

impl FormattedTextBuilder {
    /// 向当前构建缓冲区追加一个普通字符。
    ///
    /// # 参数
    /// - `ch`: 待追加的 Unicode 字符。
    fn push_char(&mut self, ch: char) {
        self.text.push(ch);
    }

    /// 返回当前构建缓冲区尾部对应的 UTF-16 代码单元偏移量（offset）。
    ///
    /// # 返回值
    /// - `Ok(i32)`: 当前 UTF-16 代码单元计数。
    /// - `Err(anyhow::Error)`: 溢出时返回错误。
    fn current_offset(&self) -> anyhow::Result<i32> {
        i32::try_from(self.text.encode_utf16().count())
            .map_err(|_| anyhow::anyhow!("message too long"))
    }

    /// 向缓冲区追加一段带指定实体的文本片段。
    ///
    /// 记录追加前后的 UTF-16 偏移并自动生成对应的 `TextEntity`。
    ///
    /// # 参数
    /// - `value`: 文本内容。
    /// - `r#type`: 实体类型（如 Code, Bold, TextUrl 等）。
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

    /// 为已经追加到缓冲区的文本范围补充实体。
    ///
    /// 适用于跨越整行或包含子实体的父级样式（例如整个标题行的加粗）。
    ///
    /// # 参数
    /// - `start`: 起始 UTF-16 偏移。
    /// - `end`: 结束 UTF-16 偏移。
    /// - `r#type`: 实体类型。
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

    /// 消费当前构建器，输出最终排好序的 TDLib `FormattedText`。
    ///
    /// TDLib 允许实体嵌套，按照 (offset 升序, length 降序) 排序保证外层实体排在前面。
    ///
    /// # 返回值
    /// - `Ok(FormattedText)`: 组装好的格式化文本。
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
///
/// 从字符迭代器中提取 `【` 与 `】` 之间的文本作为超链接标题，
/// 以及随后的 `(` 与 `)` 之间的字符串作为目标 URL。
///
/// # 参数
/// - `chars`: 字符可预览迭代器引用。
///
/// # 返回值
/// - `Some((label, url))`: 成功解析出标题与 URL。
/// - `None`: 格式不匹配或提前遇到 EOF。
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

/// 从字符迭代器中持续读取字符直到指定结束字符出现为止。
///
/// # 参数
/// - `chars`: 字符迭代器引用。
/// - `end`: 目标结束字符。
///
/// # 返回值
/// - `Some(String)`: 读取到的内容（不含 `end` 字符本身）。
/// - `None`: 未遇到结束符且迭代器耗尽时返回。
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
            "转存完成\n■ 结果\n状态：‹success›\n错误：«line1\nline2»\n【打开转存消息】(https://t.me/c/1/2)".to_owned();
        let formatted = build_card_formatted_text(text).expect("card text should parse");

        assert_eq!(
            formatted.text,
            "转存完成\n■ 结果\n状态：success\n错误：line1\nline2\n打开转存消息"
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
        assert!(formatted.entities.iter().any(|entity| {
            matches!(entity.r#type, tdlib_rs::enums::TextEntityType::PreCode(_))
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
