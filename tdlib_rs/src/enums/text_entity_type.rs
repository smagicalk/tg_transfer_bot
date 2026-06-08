#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextEntityType {
    /// A mention of a user, a supergroup, or a channel by their username
    #[serde(rename(
        serialize = "textEntityTypeMention",
        deserialize = "textEntityTypeMention"
    ))]
    Mention,
    /// A hashtag text, beginning with "#" and optionally containing a chat username at the end
    #[serde(rename(
        serialize = "textEntityTypeHashtag",
        deserialize = "textEntityTypeHashtag"
    ))]
    Hashtag,
    /// A cashtag text, beginning with "$", consisting of capital English letters (e.g., "$USD"), and optionally containing a chat username at the end
    #[serde(rename(
        serialize = "textEntityTypeCashtag",
        deserialize = "textEntityTypeCashtag"
    ))]
    Cashtag,
    /// A bot command, beginning with "/"
    #[serde(rename(
        serialize = "textEntityTypeBotCommand",
        deserialize = "textEntityTypeBotCommand"
    ))]
    BotCommand,
    /// An HTTP URL
    #[serde(rename(serialize = "textEntityTypeUrl", deserialize = "textEntityTypeUrl"))]
    Url,
    /// An email address
    #[serde(rename(
        serialize = "textEntityTypeEmailAddress",
        deserialize = "textEntityTypeEmailAddress"
    ))]
    EmailAddress,
    /// A phone number
    #[serde(rename(
        serialize = "textEntityTypePhoneNumber",
        deserialize = "textEntityTypePhoneNumber"
    ))]
    PhoneNumber,
    /// A bank card number. The getBankCardInfo method can be used to get information about the bank card
    #[serde(rename(
        serialize = "textEntityTypeBankCardNumber",
        deserialize = "textEntityTypeBankCardNumber"
    ))]
    BankCardNumber,
    /// A bold text
    #[serde(rename(serialize = "textEntityTypeBold", deserialize = "textEntityTypeBold"))]
    Bold,
    /// An italic text
    #[serde(rename(
        serialize = "textEntityTypeItalic",
        deserialize = "textEntityTypeItalic"
    ))]
    Italic,
    /// An underlined text
    #[serde(rename(
        serialize = "textEntityTypeUnderline",
        deserialize = "textEntityTypeUnderline"
    ))]
    Underline,
    /// A strikethrough text
    #[serde(rename(
        serialize = "textEntityTypeStrikethrough",
        deserialize = "textEntityTypeStrikethrough"
    ))]
    Strikethrough,
    /// A spoiler text
    #[serde(rename(
        serialize = "textEntityTypeSpoiler",
        deserialize = "textEntityTypeSpoiler"
    ))]
    Spoiler,
    /// Text that must be formatted as if inside a code HTML tag
    #[serde(rename(serialize = "textEntityTypeCode", deserialize = "textEntityTypeCode"))]
    Code,
    /// Text that must be formatted as if inside a pre HTML tag
    #[serde(rename(serialize = "textEntityTypePre", deserialize = "textEntityTypePre"))]
    Pre,
    /// Text that must be formatted as if inside pre, and code HTML tags
    #[serde(rename(
        serialize = "textEntityTypePreCode",
        deserialize = "textEntityTypePreCode"
    ))]
    PreCode(crate::types::TextEntityTypePreCode),
    /// Text that must be formatted as if inside a blockquote HTML tag; not supported in secret chats
    #[serde(rename(
        serialize = "textEntityTypeBlockQuote",
        deserialize = "textEntityTypeBlockQuote"
    ))]
    BlockQuote,
    /// Text that must be formatted as if inside a blockquote HTML tag and collapsed by default to 3 lines with the ability to show full text; not supported in secret chats
    #[serde(rename(
        serialize = "textEntityTypeExpandableBlockQuote",
        deserialize = "textEntityTypeExpandableBlockQuote"
    ))]
    ExpandableBlockQuote,
    /// A text description shown instead of a raw URL
    #[serde(rename(
        serialize = "textEntityTypeTextUrl",
        deserialize = "textEntityTypeTextUrl"
    ))]
    TextUrl(crate::types::TextEntityTypeTextUrl),
    /// A text shows instead of a raw mention of the user (e.g., when the user has no username)
    #[serde(rename(
        serialize = "textEntityTypeMentionName",
        deserialize = "textEntityTypeMentionName"
    ))]
    MentionName(crate::types::TextEntityTypeMentionName),
    /// A custom emoji. The text behind a custom emoji must be an emoji. Only premium users can use premium custom emoji
    #[serde(rename(
        serialize = "textEntityTypeCustomEmoji",
        deserialize = "textEntityTypeCustomEmoji"
    ))]
    CustomEmoji(crate::types::TextEntityTypeCustomEmoji),
    /// A media timestamp
    #[serde(rename(
        serialize = "textEntityTypeMediaTimestamp",
        deserialize = "textEntityTypeMediaTimestamp"
    ))]
    MediaTimestamp(crate::types::TextEntityTypeMediaTimestamp),
    /// A date and time
    #[serde(rename(
        serialize = "textEntityTypeDateTime",
        deserialize = "textEntityTypeDateTime"
    ))]
    DateTime(crate::types::TextEntityTypeDateTime),
}
