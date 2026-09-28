//!
//! TDLib `message` domain enums.
//!
//! Types, enums, and functions for message content, rich formatting, reactions, and drafts.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// TDLib `TextEntity` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextEntity {
    /// Represents a part of the text that needs to be formatted in some unusual way
    #[serde(rename(serialize = "textEntity", deserialize = "textEntity"))]
    TextEntity(Box<crate::types::TextEntity>),
}

impl TextEntity {
    /// Convenience constructor to create a [`TextEntity::TextEntity`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_entity(val: crate::types::TextEntity) -> Self {
        Self::TextEntity(Box::new(val))
    }

}

/// Converts a [`crate::types::TextEntity`] into [`TextEntity`].
impl From<crate::types::TextEntity> for TextEntity {
    fn from(val: crate::types::TextEntity) -> Self {
        Self::TextEntity(Box::new(val))
    }
}

/// TDLib `TextEntities` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextEntities {
    /// Contains a list of text entities
    #[serde(rename(serialize = "textEntities", deserialize = "textEntities"))]
    TextEntities(Box<crate::types::TextEntities>),
}

impl TextEntities {
    /// Convenience constructor to create a [`TextEntities::TextEntities`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_entities(val: crate::types::TextEntities) -> Self {
        Self::TextEntities(Box::new(val))
    }

}

/// Converts a [`crate::types::TextEntities`] into [`TextEntities`].
impl From<crate::types::TextEntities> for TextEntities {
    fn from(val: crate::types::TextEntities) -> Self {
        Self::TextEntities(Box::new(val))
    }
}

/// TDLib `FormattedText` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FormattedText {
    /// A text with some entities
    #[serde(rename(serialize = "formattedText", deserialize = "formattedText"))]
    FormattedText(Box<crate::types::FormattedText>),
}

impl FormattedText {
    /// Convenience constructor to create a [`FormattedText::FormattedText`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn formatted_text(val: crate::types::FormattedText) -> Self {
        Self::FormattedText(Box::new(val))
    }

}

/// Converts a [`crate::types::FormattedText`] into [`FormattedText`].
impl From<crate::types::FormattedText> for FormattedText {
    fn from(val: crate::types::FormattedText) -> Self {
        Self::FormattedText(Box::new(val))
    }
}

/// TDLib `RichMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RichMessage {
    /// Describes a message with rich formatting
    #[serde(rename(serialize = "richMessage", deserialize = "richMessage"))]
    RichMessage(Box<crate::types::RichMessage>),
}

impl RichMessage {
    /// Convenience constructor to create a [`RichMessage::RichMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn rich_message(val: crate::types::RichMessage) -> Self {
        Self::RichMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::RichMessage`] into [`RichMessage`].
impl From<crate::types::RichMessage> for RichMessage {
    fn from(val: crate::types::RichMessage) -> Self {
        Self::RichMessage(Box::new(val))
    }
}

/// TDLib `InputRichMessageMedia` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputRichMessageMedia {
    /// Describes a media to be used in a sent rich message
    #[serde(rename(serialize = "inputRichMessageMedia", deserialize = "inputRichMessageMedia"))]
    InputRichMessageMedia(Box<crate::types::InputRichMessageMedia>),
}

impl InputRichMessageMedia {
    /// Convenience constructor to create a [`InputRichMessageMedia::InputRichMessageMedia`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_rich_message_media(val: crate::types::InputRichMessageMedia) -> Self {
        Self::InputRichMessageMedia(Box::new(val))
    }

}

/// Converts a [`crate::types::InputRichMessageMedia`] into [`InputRichMessageMedia`].
impl From<crate::types::InputRichMessageMedia> for InputRichMessageMedia {
    fn from(val: crate::types::InputRichMessageMedia) -> Self {
        Self::InputRichMessageMedia(Box::new(val))
    }
}

/// Describes source of a rich message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RichMessageSource {
    /// A rich message defined by blocks
    #[serde(rename(serialize = "richMessageSourceBlocks", deserialize = "richMessageSourceBlocks"))]
    Blocks(Box<crate::types::RichMessageSourceBlocks>),
    /// A Markdown-formatted rich message; for bots only
    #[serde(rename(serialize = "richMessageSourceMarkdown", deserialize = "richMessageSourceMarkdown"))]
    Markdown(Box<crate::types::RichMessageSourceMarkdown>),
    /// An HTML-formatted rich message; for bots only
    #[serde(rename(serialize = "richMessageSourceHtml", deserialize = "richMessageSourceHtml"))]
    Html(Box<crate::types::RichMessageSourceHtml>),
}

impl RichMessageSource {
    /// Convenience constructor to create a [`RichMessageSource::Blocks`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn blocks(val: crate::types::RichMessageSourceBlocks) -> Self {
        Self::Blocks(Box::new(val))
    }

    /// Convenience constructor to create a [`RichMessageSource::Markdown`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn markdown(val: crate::types::RichMessageSourceMarkdown) -> Self {
        Self::Markdown(Box::new(val))
    }

    /// Convenience constructor to create a [`RichMessageSource::Html`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn html(val: crate::types::RichMessageSourceHtml) -> Self {
        Self::Html(Box::new(val))
    }

}

/// Converts a [`crate::types::RichMessageSourceBlocks`] into [`RichMessageSource`].
impl From<crate::types::RichMessageSourceBlocks> for RichMessageSource {
    fn from(val: crate::types::RichMessageSourceBlocks) -> Self {
        Self::Blocks(Box::new(val))
    }
}

/// Converts a [`crate::types::RichMessageSourceMarkdown`] into [`RichMessageSource`].
impl From<crate::types::RichMessageSourceMarkdown> for RichMessageSource {
    fn from(val: crate::types::RichMessageSourceMarkdown) -> Self {
        Self::Markdown(Box::new(val))
    }
}

/// Converts a [`crate::types::RichMessageSourceHtml`] into [`RichMessageSource`].
impl From<crate::types::RichMessageSourceHtml> for RichMessageSource {
    fn from(val: crate::types::RichMessageSourceHtml) -> Self {
        Self::Html(Box::new(val))
    }
}

/// TDLib `InputRichMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputRichMessage {
    /// A rich message to send. Total length of all texts, including custom emoji alternative text and formula source, must not exceed getOption("rich_message_text_length_max").
    /// The total number of all blocks, list items and table rows must not exceed getOption("rich_message_block_count_max").
    /// The maximum allowed depth of nested blocks and rich texts is getOption("rich_message_depth_max").
    /// The total number of media in all blocks must not exceed getOption("rich_message_media_count_max").
    /// The maximum allowed number of table columns is getOption("rich_message_table_column_count_max")
    #[serde(rename(serialize = "inputRichMessage", deserialize = "inputRichMessage"))]
    InputRichMessage(Box<crate::types::InputRichMessage>),
}

impl InputRichMessage {
    /// Convenience constructor to create a [`InputRichMessage::InputRichMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_rich_message(val: crate::types::InputRichMessage) -> Self {
        Self::InputRichMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::InputRichMessage`] into [`InputRichMessage`].
impl From<crate::types::InputRichMessage> for InputRichMessage {
    fn from(val: crate::types::InputRichMessage) -> Self {
        Self::InputRichMessage(Box::new(val))
    }
}

/// TDLib `DiffText` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DiffText {
    /// A text with some changes highlighted
    #[serde(rename(serialize = "diffText", deserialize = "diffText"))]
    DiffText(Box<crate::types::DiffText>),
}

impl DiffText {
    /// Convenience constructor to create a [`DiffText::DiffText`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn diff_text(val: crate::types::DiffText) -> Self {
        Self::DiffText(Box::new(val))
    }

}

/// Converts a [`crate::types::DiffText`] into [`DiffText`].
impl From<crate::types::DiffText> for DiffText {
    fn from(val: crate::types::DiffText) -> Self {
        Self::DiffText(Box::new(val))
    }
}

/// TDLib `FixedText` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FixedText {
    /// A text fixed using fixTextWithAi
    #[serde(rename(serialize = "fixedText", deserialize = "fixedText"))]
    FixedText(Box<crate::types::FixedText>),
}

impl FixedText {
    /// Convenience constructor to create a [`FixedText::FixedText`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fixed_text(val: crate::types::FixedText) -> Self {
        Self::FixedText(Box::new(val))
    }

}

/// Converts a [`crate::types::FixedText`] into [`FixedText`].
impl From<crate::types::FixedText> for FixedText {
    fn from(val: crate::types::FixedText) -> Self {
        Self::FixedText(Box::new(val))
    }
}

/// TDLib `TextCompositionStyleExample` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextCompositionStyleExample {
    /// Contains an example of text composition style usage
    #[serde(rename(serialize = "textCompositionStyleExample", deserialize = "textCompositionStyleExample"))]
    TextCompositionStyleExample(Box<crate::types::TextCompositionStyleExample>),
}

impl TextCompositionStyleExample {
    /// Convenience constructor to create a [`TextCompositionStyleExample::TextCompositionStyleExample`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_composition_style_example(val: crate::types::TextCompositionStyleExample) -> Self {
        Self::TextCompositionStyleExample(Box::new(val))
    }

}

/// Converts a [`crate::types::TextCompositionStyleExample`] into [`TextCompositionStyleExample`].
impl From<crate::types::TextCompositionStyleExample> for TextCompositionStyleExample {
    fn from(val: crate::types::TextCompositionStyleExample) -> Self {
        Self::TextCompositionStyleExample(Box::new(val))
    }
}

/// TDLib `TextCompositionStyle` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextCompositionStyle {
    /// Describes a style that can be used to compose a text
    #[serde(rename(serialize = "textCompositionStyle", deserialize = "textCompositionStyle"))]
    TextCompositionStyle(Box<crate::types::TextCompositionStyle>),
}

impl TextCompositionStyle {
    /// Convenience constructor to create a [`TextCompositionStyle::TextCompositionStyle`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_composition_style(val: crate::types::TextCompositionStyle) -> Self {
        Self::TextCompositionStyle(Box::new(val))
    }

}

/// Converts a [`crate::types::TextCompositionStyle`] into [`TextCompositionStyle`].
impl From<crate::types::TextCompositionStyle> for TextCompositionStyle {
    fn from(val: crate::types::TextCompositionStyle) -> Self {
        Self::TextCompositionStyle(Box::new(val))
    }
}

/// Contains information about the sender of a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSender {
    /// The message was sent by a known user
    #[serde(rename(serialize = "messageSenderUser", deserialize = "messageSenderUser"))]
    User(Box<crate::types::MessageSenderUser>),
    /// The message was sent on behalf of a chat
    #[serde(rename(serialize = "messageSenderChat", deserialize = "messageSenderChat"))]
    Chat(Box<crate::types::MessageSenderChat>),
}

impl MessageSender {
    /// Convenience constructor to create a [`MessageSender::User`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user(val: crate::types::MessageSenderUser) -> Self {
        Self::User(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageSender::Chat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat(val: crate::types::MessageSenderChat) -> Self {
        Self::Chat(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageSenderUser`] into [`MessageSender`].
impl From<crate::types::MessageSenderUser> for MessageSender {
    fn from(val: crate::types::MessageSenderUser) -> Self {
        Self::User(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSenderChat`] into [`MessageSender`].
impl From<crate::types::MessageSenderChat> for MessageSender {
    fn from(val: crate::types::MessageSenderChat) -> Self {
        Self::Chat(Box::new(val))
    }
}

/// TDLib `MessageSenders` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSenders {
    /// Represents a list of message senders
    #[serde(rename(serialize = "messageSenders", deserialize = "messageSenders"))]
    MessageSenders(Box<crate::types::MessageSenders>),
}

impl MessageSenders {
    /// Convenience constructor to create a [`MessageSenders::MessageSenders`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_senders(val: crate::types::MessageSenders) -> Self {
        Self::MessageSenders(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageSenders`] into [`MessageSenders`].
impl From<crate::types::MessageSenders> for MessageSenders {
    fn from(val: crate::types::MessageSenders) -> Self {
        Self::MessageSenders(Box::new(val))
    }
}

/// TDLib `ChatMessageSender` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMessageSender {
    /// Represents a message sender, which can be used to send messages in a chat
    #[serde(rename(serialize = "chatMessageSender", deserialize = "chatMessageSender"))]
    ChatMessageSender(Box<crate::types::ChatMessageSender>),
}

impl ChatMessageSender {
    /// Convenience constructor to create a [`ChatMessageSender::ChatMessageSender`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_message_sender(val: crate::types::ChatMessageSender) -> Self {
        Self::ChatMessageSender(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatMessageSender`] into [`ChatMessageSender`].
impl From<crate::types::ChatMessageSender> for ChatMessageSender {
    fn from(val: crate::types::ChatMessageSender) -> Self {
        Self::ChatMessageSender(Box::new(val))
    }
}

/// TDLib `ChatMessageSenders` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatMessageSenders {
    /// Represents a list of message senders, which can be used to send messages in a chat
    #[serde(rename(serialize = "chatMessageSenders", deserialize = "chatMessageSenders"))]
    ChatMessageSenders(Box<crate::types::ChatMessageSenders>),
}

impl ChatMessageSenders {
    /// Convenience constructor to create a [`ChatMessageSenders::ChatMessageSenders`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_message_senders(val: crate::types::ChatMessageSenders) -> Self {
        Self::ChatMessageSenders(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatMessageSenders`] into [`ChatMessageSenders`].
impl From<crate::types::ChatMessageSenders> for ChatMessageSenders {
    fn from(val: crate::types::ChatMessageSenders) -> Self {
        Self::ChatMessageSenders(Box::new(val))
    }
}

/// Describes read date of a recent outgoing message in a private chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReadDate {
    /// Contains read date of the message
    #[serde(rename(serialize = "messageReadDateRead", deserialize = "messageReadDateRead"))]
    Read(Box<crate::types::MessageReadDateRead>),
    /// The message is unread yet
    #[serde(rename(serialize = "messageReadDateUnread", deserialize = "messageReadDateUnread"))]
    Unread,
    /// The message is too old to get read date
    #[serde(rename(serialize = "messageReadDateTooOld", deserialize = "messageReadDateTooOld"))]
    TooOld,
    /// The read date is unknown due to privacy settings of the other user
    #[serde(rename(serialize = "messageReadDateUserPrivacyRestricted", deserialize = "messageReadDateUserPrivacyRestricted"))]
    UserPrivacyRestricted,
    /// The read date is unknown due to privacy settings of the current user, but will be known if the user subscribes to Telegram Premium
    #[serde(rename(serialize = "messageReadDateMyPrivacyRestricted", deserialize = "messageReadDateMyPrivacyRestricted"))]
    MyPrivacyRestricted,
}

impl MessageReadDate {
    /// Convenience constructor to create a [`MessageReadDate::Read`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn read(val: crate::types::MessageReadDateRead) -> Self {
        Self::Read(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageReadDateRead`] into [`MessageReadDate`].
impl From<crate::types::MessageReadDateRead> for MessageReadDate {
    fn from(val: crate::types::MessageReadDateRead) -> Self {
        Self::Read(Box::new(val))
    }
}

/// TDLib `MessageViewer` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageViewer {
    /// Represents a viewer of a message
    #[serde(rename(serialize = "messageViewer", deserialize = "messageViewer"))]
    MessageViewer(Box<crate::types::MessageViewer>),
}

impl MessageViewer {
    /// Convenience constructor to create a [`MessageViewer::MessageViewer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_viewer(val: crate::types::MessageViewer) -> Self {
        Self::MessageViewer(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageViewer`] into [`MessageViewer`].
impl From<crate::types::MessageViewer> for MessageViewer {
    fn from(val: crate::types::MessageViewer) -> Self {
        Self::MessageViewer(Box::new(val))
    }
}

/// TDLib `MessageViewers` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageViewers {
    /// Represents a list of message viewers
    #[serde(rename(serialize = "messageViewers", deserialize = "messageViewers"))]
    MessageViewers(Box<crate::types::MessageViewers>),
}

impl MessageViewers {
    /// Convenience constructor to create a [`MessageViewers::MessageViewers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_viewers(val: crate::types::MessageViewers) -> Self {
        Self::MessageViewers(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageViewers`] into [`MessageViewers`].
impl From<crate::types::MessageViewers> for MessageViewers {
    fn from(val: crate::types::MessageViewers) -> Self {
        Self::MessageViewers(Box::new(val))
    }
}

/// Contains information about the origin of a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageOrigin {
    /// The message was originally sent by a known user
    #[serde(rename(serialize = "messageOriginUser", deserialize = "messageOriginUser"))]
    User(Box<crate::types::MessageOriginUser>),
    /// The message was originally sent by a user who is hidden by their privacy settings
    #[serde(rename(serialize = "messageOriginHiddenUser", deserialize = "messageOriginHiddenUser"))]
    HiddenUser(Box<crate::types::MessageOriginHiddenUser>),
    /// The message was originally sent on behalf of a chat
    #[serde(rename(serialize = "messageOriginChat", deserialize = "messageOriginChat"))]
    Chat(Box<crate::types::MessageOriginChat>),
    /// The message was originally a post in a channel
    #[serde(rename(serialize = "messageOriginChannel", deserialize = "messageOriginChannel"))]
    Channel(Box<crate::types::MessageOriginChannel>),
}

impl MessageOrigin {
    /// Convenience constructor to create a [`MessageOrigin::User`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user(val: crate::types::MessageOriginUser) -> Self {
        Self::User(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageOrigin::HiddenUser`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn hidden_user(val: crate::types::MessageOriginHiddenUser) -> Self {
        Self::HiddenUser(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageOrigin::Chat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat(val: crate::types::MessageOriginChat) -> Self {
        Self::Chat(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageOrigin::Channel`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn channel(val: crate::types::MessageOriginChannel) -> Self {
        Self::Channel(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageOriginUser`] into [`MessageOrigin`].
impl From<crate::types::MessageOriginUser> for MessageOrigin {
    fn from(val: crate::types::MessageOriginUser) -> Self {
        Self::User(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageOriginHiddenUser`] into [`MessageOrigin`].
impl From<crate::types::MessageOriginHiddenUser> for MessageOrigin {
    fn from(val: crate::types::MessageOriginHiddenUser) -> Self {
        Self::HiddenUser(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageOriginChat`] into [`MessageOrigin`].
impl From<crate::types::MessageOriginChat> for MessageOrigin {
    fn from(val: crate::types::MessageOriginChat) -> Self {
        Self::Chat(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageOriginChannel`] into [`MessageOrigin`].
impl From<crate::types::MessageOriginChannel> for MessageOrigin {
    fn from(val: crate::types::MessageOriginChannel) -> Self {
        Self::Channel(Box::new(val))
    }
}

/// Describes type of message reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReactionType {
    /// A reaction with an emoji
    #[serde(rename(serialize = "reactionTypeEmoji", deserialize = "reactionTypeEmoji"))]
    Emoji(Box<crate::types::ReactionTypeEmoji>),
    /// A reaction with a custom emoji
    #[serde(rename(serialize = "reactionTypeCustomEmoji", deserialize = "reactionTypeCustomEmoji"))]
    CustomEmoji(Box<crate::types::ReactionTypeCustomEmoji>),
    /// The paid reaction in a channel chat
    #[serde(rename(serialize = "reactionTypePaid", deserialize = "reactionTypePaid"))]
    Paid,
}

impl ReactionType {
    /// Convenience constructor to create a [`ReactionType::Emoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji(val: crate::types::ReactionTypeEmoji) -> Self {
        Self::Emoji(Box::new(val))
    }

    /// Convenience constructor to create a [`ReactionType::CustomEmoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom_emoji(val: crate::types::ReactionTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }

}

/// Converts a [`crate::types::ReactionTypeEmoji`] into [`ReactionType`].
impl From<crate::types::ReactionTypeEmoji> for ReactionType {
    fn from(val: crate::types::ReactionTypeEmoji) -> Self {
        Self::Emoji(Box::new(val))
    }
}

/// Converts a [`crate::types::ReactionTypeCustomEmoji`] into [`ReactionType`].
impl From<crate::types::ReactionTypeCustomEmoji> for ReactionType {
    fn from(val: crate::types::ReactionTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }
}

/// Describes type of paid message reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaidReactionType {
    /// A paid reaction on behalf of the current user
    #[serde(rename(serialize = "paidReactionTypeRegular", deserialize = "paidReactionTypeRegular"))]
    Regular,
    /// An anonymous paid reaction
    #[serde(rename(serialize = "paidReactionTypeAnonymous", deserialize = "paidReactionTypeAnonymous"))]
    Anonymous,
    /// A paid reaction on behalf of an owned chat
    #[serde(rename(serialize = "paidReactionTypeChat", deserialize = "paidReactionTypeChat"))]
    Chat(Box<crate::types::PaidReactionTypeChat>),
}

impl PaidReactionType {
    /// Convenience constructor to create a [`PaidReactionType::Chat`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat(val: crate::types::PaidReactionTypeChat) -> Self {
        Self::Chat(Box::new(val))
    }

}

/// Converts a [`crate::types::PaidReactionTypeChat`] into [`PaidReactionType`].
impl From<crate::types::PaidReactionTypeChat> for PaidReactionType {
    fn from(val: crate::types::PaidReactionTypeChat) -> Self {
        Self::Chat(Box::new(val))
    }
}

/// TDLib `MessageForwardInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageForwardInfo {
    /// Contains information about a forwarded message
    #[serde(rename(serialize = "messageForwardInfo", deserialize = "messageForwardInfo"))]
    MessageForwardInfo(Box<crate::types::MessageForwardInfo>),
}

impl MessageForwardInfo {
    /// Convenience constructor to create a [`MessageForwardInfo::MessageForwardInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_forward_info(val: crate::types::MessageForwardInfo) -> Self {
        Self::MessageForwardInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageForwardInfo`] into [`MessageForwardInfo`].
impl From<crate::types::MessageForwardInfo> for MessageForwardInfo {
    fn from(val: crate::types::MessageForwardInfo) -> Self {
        Self::MessageForwardInfo(Box::new(val))
    }
}

/// TDLib `MessageImportInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageImportInfo {
    /// Contains information about a message created with importMessages
    #[serde(rename(serialize = "messageImportInfo", deserialize = "messageImportInfo"))]
    MessageImportInfo(Box<crate::types::MessageImportInfo>),
}

impl MessageImportInfo {
    /// Convenience constructor to create a [`MessageImportInfo::MessageImportInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_import_info(val: crate::types::MessageImportInfo) -> Self {
        Self::MessageImportInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageImportInfo`] into [`MessageImportInfo`].
impl From<crate::types::MessageImportInfo> for MessageImportInfo {
    fn from(val: crate::types::MessageImportInfo) -> Self {
        Self::MessageImportInfo(Box::new(val))
    }
}

/// TDLib `MessageReplyInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReplyInfo {
    /// Contains information about replies to a message
    #[serde(rename(serialize = "messageReplyInfo", deserialize = "messageReplyInfo"))]
    MessageReplyInfo(Box<crate::types::MessageReplyInfo>),
}

impl MessageReplyInfo {
    /// Convenience constructor to create a [`MessageReplyInfo::MessageReplyInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_reply_info(val: crate::types::MessageReplyInfo) -> Self {
        Self::MessageReplyInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageReplyInfo`] into [`MessageReplyInfo`].
impl From<crate::types::MessageReplyInfo> for MessageReplyInfo {
    fn from(val: crate::types::MessageReplyInfo) -> Self {
        Self::MessageReplyInfo(Box::new(val))
    }
}

/// TDLib `MessageReaction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReaction {
    /// Contains information about a reaction to a message
    #[serde(rename(serialize = "messageReaction", deserialize = "messageReaction"))]
    MessageReaction(Box<crate::types::MessageReaction>),
}

impl MessageReaction {
    /// Convenience constructor to create a [`MessageReaction::MessageReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_reaction(val: crate::types::MessageReaction) -> Self {
        Self::MessageReaction(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageReaction`] into [`MessageReaction`].
impl From<crate::types::MessageReaction> for MessageReaction {
    fn from(val: crate::types::MessageReaction) -> Self {
        Self::MessageReaction(Box::new(val))
    }
}

/// TDLib `MessageReactions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReactions {
    /// Contains a list of reactions added to a message
    #[serde(rename(serialize = "messageReactions", deserialize = "messageReactions"))]
    MessageReactions(Box<crate::types::MessageReactions>),
}

impl MessageReactions {
    /// Convenience constructor to create a [`MessageReactions::MessageReactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_reactions(val: crate::types::MessageReactions) -> Self {
        Self::MessageReactions(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageReactions`] into [`MessageReactions`].
impl From<crate::types::MessageReactions> for MessageReactions {
    fn from(val: crate::types::MessageReactions) -> Self {
        Self::MessageReactions(Box::new(val))
    }
}

/// TDLib `MessageInteractionInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageInteractionInfo {
    /// Contains information about interactions with a message
    #[serde(rename(serialize = "messageInteractionInfo", deserialize = "messageInteractionInfo"))]
    MessageInteractionInfo(Box<crate::types::MessageInteractionInfo>),
}

impl MessageInteractionInfo {
    /// Convenience constructor to create a [`MessageInteractionInfo::MessageInteractionInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_interaction_info(val: crate::types::MessageInteractionInfo) -> Self {
        Self::MessageInteractionInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageInteractionInfo`] into [`MessageInteractionInfo`].
impl From<crate::types::MessageInteractionInfo> for MessageInteractionInfo {
    fn from(val: crate::types::MessageInteractionInfo) -> Self {
        Self::MessageInteractionInfo(Box::new(val))
    }
}

/// TDLib `UnreadReaction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UnreadReaction {
    /// Contains information about an unread reaction to a message
    #[serde(rename(serialize = "unreadReaction", deserialize = "unreadReaction"))]
    UnreadReaction(Box<crate::types::UnreadReaction>),
}

impl UnreadReaction {
    /// Convenience constructor to create a [`UnreadReaction::UnreadReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn unread_reaction(val: crate::types::UnreadReaction) -> Self {
        Self::UnreadReaction(Box::new(val))
    }

}

/// Converts a [`crate::types::UnreadReaction`] into [`UnreadReaction`].
impl From<crate::types::UnreadReaction> for UnreadReaction {
    fn from(val: crate::types::UnreadReaction) -> Self {
        Self::UnreadReaction(Box::new(val))
    }
}

/// Describes type of emoji effect
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageEffectType {
    /// An effect from an emoji reaction
    #[serde(rename(serialize = "messageEffectTypeEmojiReaction", deserialize = "messageEffectTypeEmojiReaction"))]
    EmojiReaction(Box<crate::types::MessageEffectTypeEmojiReaction>),
    /// An effect from a premium sticker
    #[serde(rename(serialize = "messageEffectTypePremiumSticker", deserialize = "messageEffectTypePremiumSticker"))]
    PremiumSticker(Box<crate::types::MessageEffectTypePremiumSticker>),
}

impl MessageEffectType {
    /// Convenience constructor to create a [`MessageEffectType::EmojiReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn emoji_reaction(val: crate::types::MessageEffectTypeEmojiReaction) -> Self {
        Self::EmojiReaction(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageEffectType::PremiumSticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_sticker(val: crate::types::MessageEffectTypePremiumSticker) -> Self {
        Self::PremiumSticker(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageEffectTypeEmojiReaction`] into [`MessageEffectType`].
impl From<crate::types::MessageEffectTypeEmojiReaction> for MessageEffectType {
    fn from(val: crate::types::MessageEffectTypeEmojiReaction) -> Self {
        Self::EmojiReaction(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageEffectTypePremiumSticker`] into [`MessageEffectType`].
impl From<crate::types::MessageEffectTypePremiumSticker> for MessageEffectType {
    fn from(val: crate::types::MessageEffectTypePremiumSticker) -> Self {
        Self::PremiumSticker(Box::new(val))
    }
}

/// TDLib `MessageEffect` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageEffect {
    /// Contains information about an effect added to a message
    #[serde(rename(serialize = "messageEffect", deserialize = "messageEffect"))]
    MessageEffect(Box<crate::types::MessageEffect>),
}

impl MessageEffect {
    /// Convenience constructor to create a [`MessageEffect::MessageEffect`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_effect(val: crate::types::MessageEffect) -> Self {
        Self::MessageEffect(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageEffect`] into [`MessageEffect`].
impl From<crate::types::MessageEffect> for MessageEffect {
    fn from(val: crate::types::MessageEffect) -> Self {
        Self::MessageEffect(Box::new(val))
    }
}

/// Contains information about the sending state of the message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSendingState {
    /// The message is being sent now, but has not yet been delivered to the server
    #[serde(rename(serialize = "messageSendingStatePending", deserialize = "messageSendingStatePending"))]
    Pending(Box<crate::types::MessageSendingStatePending>),
    /// The message failed to be sent
    #[serde(rename(serialize = "messageSendingStateFailed", deserialize = "messageSendingStateFailed"))]
    Failed(Box<crate::types::MessageSendingStateFailed>),
}

impl MessageSendingState {
    /// Convenience constructor to create a [`MessageSendingState::Pending`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pending(val: crate::types::MessageSendingStatePending) -> Self {
        Self::Pending(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageSendingState::Failed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn failed(val: crate::types::MessageSendingStateFailed) -> Self {
        Self::Failed(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageSendingStatePending`] into [`MessageSendingState`].
impl From<crate::types::MessageSendingStatePending> for MessageSendingState {
    fn from(val: crate::types::MessageSendingStatePending) -> Self {
        Self::Pending(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSendingStateFailed`] into [`MessageSendingState`].
impl From<crate::types::MessageSendingStateFailed> for MessageSendingState {
    fn from(val: crate::types::MessageSendingStateFailed) -> Self {
        Self::Failed(Box::new(val))
    }
}

/// TDLib `TextQuote` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextQuote {
    /// Describes manually or automatically chosen quote from another message
    #[serde(rename(serialize = "textQuote", deserialize = "textQuote"))]
    TextQuote(Box<crate::types::TextQuote>),
}

impl TextQuote {
    /// Convenience constructor to create a [`TextQuote::TextQuote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_quote(val: crate::types::TextQuote) -> Self {
        Self::TextQuote(Box::new(val))
    }

}

/// Converts a [`crate::types::TextQuote`] into [`TextQuote`].
impl From<crate::types::TextQuote> for TextQuote {
    fn from(val: crate::types::TextQuote) -> Self {
        Self::TextQuote(Box::new(val))
    }
}

/// TDLib `InputTextQuote` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputTextQuote {
    /// Describes manually chosen quote from another message
    #[serde(rename(serialize = "inputTextQuote", deserialize = "inputTextQuote"))]
    InputTextQuote(Box<crate::types::InputTextQuote>),
}

impl InputTextQuote {
    /// Convenience constructor to create a [`InputTextQuote::InputTextQuote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_text_quote(val: crate::types::InputTextQuote) -> Self {
        Self::InputTextQuote(Box::new(val))
    }

}

/// Converts a [`crate::types::InputTextQuote`] into [`InputTextQuote`].
impl From<crate::types::InputTextQuote> for InputTextQuote {
    fn from(val: crate::types::InputTextQuote) -> Self {
        Self::InputTextQuote(Box::new(val))
    }
}

/// Contains information about the message or the story a message is replying to
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageReplyTo {
    /// Describes a message replied by a given message
    #[serde(rename(serialize = "messageReplyToMessage", deserialize = "messageReplyToMessage"))]
    Message(Box<crate::types::MessageReplyToMessage>),
    /// Describes a story replied by a given message
    #[serde(rename(serialize = "messageReplyToStory", deserialize = "messageReplyToStory"))]
    Story(Box<crate::types::MessageReplyToStory>),
}

impl MessageReplyTo {
    /// Convenience constructor to create a [`MessageReplyTo::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::MessageReplyToMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageReplyTo::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::MessageReplyToStory) -> Self {
        Self::Story(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageReplyToMessage`] into [`MessageReplyTo`].
impl From<crate::types::MessageReplyToMessage> for MessageReplyTo {
    fn from(val: crate::types::MessageReplyToMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageReplyToStory`] into [`MessageReplyTo`].
impl From<crate::types::MessageReplyToStory> for MessageReplyTo {
    fn from(val: crate::types::MessageReplyToStory) -> Self {
        Self::Story(Box::new(val))
    }
}

/// Contains information about the message or the story to be replied
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputMessageReplyTo {
    /// Describes a message to be replied in the same chat and forum topic
    #[serde(rename(serialize = "inputMessageReplyToMessage", deserialize = "inputMessageReplyToMessage"))]
    Message(Box<crate::types::InputMessageReplyToMessage>),
    /// Describes a message to be replied that is from a different chat or a forum topic; not supported in secret chats
    #[serde(rename(serialize = "inputMessageReplyToExternalMessage", deserialize = "inputMessageReplyToExternalMessage"))]
    ExternalMessage(Box<crate::types::InputMessageReplyToExternalMessage>),
    /// Describes a story to be replied
    #[serde(rename(serialize = "inputMessageReplyToStory", deserialize = "inputMessageReplyToStory"))]
    Story(Box<crate::types::InputMessageReplyToStory>),
    /// Describes an ephemeral message to be replied; for bots only
    #[serde(rename(serialize = "inputMessageReplyToEphemeralMessage", deserialize = "inputMessageReplyToEphemeralMessage"))]
    EphemeralMessage(Box<crate::types::InputMessageReplyToEphemeralMessage>),
}

impl InputMessageReplyTo {
    /// Convenience constructor to create a [`InputMessageReplyTo::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::InputMessageReplyToMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageReplyTo::ExternalMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn external_message(val: crate::types::InputMessageReplyToExternalMessage) -> Self {
        Self::ExternalMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageReplyTo::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::InputMessageReplyToStory) -> Self {
        Self::Story(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageReplyTo::EphemeralMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ephemeral_message(val: crate::types::InputMessageReplyToEphemeralMessage) -> Self {
        Self::EphemeralMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::InputMessageReplyToMessage`] into [`InputMessageReplyTo`].
impl From<crate::types::InputMessageReplyToMessage> for InputMessageReplyTo {
    fn from(val: crate::types::InputMessageReplyToMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageReplyToExternalMessage`] into [`InputMessageReplyTo`].
impl From<crate::types::InputMessageReplyToExternalMessage> for InputMessageReplyTo {
    fn from(val: crate::types::InputMessageReplyToExternalMessage) -> Self {
        Self::ExternalMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageReplyToStory`] into [`InputMessageReplyTo`].
impl From<crate::types::InputMessageReplyToStory> for InputMessageReplyTo {
    fn from(val: crate::types::InputMessageReplyToStory) -> Self {
        Self::Story(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageReplyToEphemeralMessage`] into [`InputMessageReplyTo`].
impl From<crate::types::InputMessageReplyToEphemeralMessage> for InputMessageReplyTo {
    fn from(val: crate::types::InputMessageReplyToEphemeralMessage) -> Self {
        Self::EphemeralMessage(Box::new(val))
    }
}

/// TDLib `EphemeralMessageContent` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EphemeralMessageContent {
    /// Describes an ephemeral content of a regular message, which must be shown instead of the regular content
    #[serde(rename(serialize = "ephemeralMessageContent", deserialize = "ephemeralMessageContent"))]
    EphemeralMessageContent(Box<crate::types::EphemeralMessageContent>),
}

impl EphemeralMessageContent {
    /// Convenience constructor to create a [`EphemeralMessageContent::EphemeralMessageContent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ephemeral_message_content(val: crate::types::EphemeralMessageContent) -> Self {
        Self::EphemeralMessageContent(Box::new(val))
    }

}

/// Converts a [`crate::types::EphemeralMessageContent`] into [`EphemeralMessageContent`].
impl From<crate::types::EphemeralMessageContent> for EphemeralMessageContent {
    fn from(val: crate::types::EphemeralMessageContent) -> Self {
        Self::EphemeralMessageContent(Box::new(val))
    }
}

/// TDLib `Message` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Message {
    /// Describes a message
    #[serde(rename(serialize = "message", deserialize = "message"))]
    Message(Box<crate::types::Message>),
}

impl Message {
    /// Convenience constructor to create a [`Message::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::Message) -> Self {
        Self::Message(Box::new(val))
    }

}

/// Converts a [`crate::types::Message`] into [`Message`].
impl From<crate::types::Message> for Message {
    fn from(val: crate::types::Message) -> Self {
        Self::Message(Box::new(val))
    }
}

/// TDLib `Messages` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Messages {
    /// Contains a list of messages
    #[serde(rename(serialize = "messages", deserialize = "messages"))]
    Messages(Box<crate::types::Messages>),
}

impl Messages {
    /// Convenience constructor to create a [`Messages::Messages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn messages(val: crate::types::Messages) -> Self {
        Self::Messages(Box::new(val))
    }

}

/// Converts a [`crate::types::Messages`] into [`Messages`].
impl From<crate::types::Messages> for Messages {
    fn from(val: crate::types::Messages) -> Self {
        Self::Messages(Box::new(val))
    }
}

/// TDLib `FoundMessages` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundMessages {
    /// Contains a list of messages found by a search
    #[serde(rename(serialize = "foundMessages", deserialize = "foundMessages"))]
    FoundMessages(Box<crate::types::FoundMessages>),
}

impl FoundMessages {
    /// Convenience constructor to create a [`FoundMessages::FoundMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_messages(val: crate::types::FoundMessages) -> Self {
        Self::FoundMessages(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundMessages`] into [`FoundMessages`].
impl From<crate::types::FoundMessages> for FoundMessages {
    fn from(val: crate::types::FoundMessages) -> Self {
        Self::FoundMessages(Box::new(val))
    }
}

/// TDLib `FoundChatMessages` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundChatMessages {
    /// Contains a list of messages found by a search in a given chat
    #[serde(rename(serialize = "foundChatMessages", deserialize = "foundChatMessages"))]
    FoundChatMessages(Box<crate::types::FoundChatMessages>),
}

impl FoundChatMessages {
    /// Convenience constructor to create a [`FoundChatMessages::FoundChatMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_chat_messages(val: crate::types::FoundChatMessages) -> Self {
        Self::FoundChatMessages(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundChatMessages`] into [`FoundChatMessages`].
impl From<crate::types::FoundChatMessages> for FoundChatMessages {
    fn from(val: crate::types::FoundChatMessages) -> Self {
        Self::FoundChatMessages(Box::new(val))
    }
}

/// TDLib `MessagePosition` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessagePosition {
    /// Contains information about a message in a specific position
    #[serde(rename(serialize = "messagePosition", deserialize = "messagePosition"))]
    MessagePosition(Box<crate::types::MessagePosition>),
}

impl MessagePosition {
    /// Convenience constructor to create a [`MessagePosition::MessagePosition`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_position(val: crate::types::MessagePosition) -> Self {
        Self::MessagePosition(Box::new(val))
    }

}

/// Converts a [`crate::types::MessagePosition`] into [`MessagePosition`].
impl From<crate::types::MessagePosition> for MessagePosition {
    fn from(val: crate::types::MessagePosition) -> Self {
        Self::MessagePosition(Box::new(val))
    }
}

/// TDLib `MessagePositions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessagePositions {
    /// Contains a list of message positions
    #[serde(rename(serialize = "messagePositions", deserialize = "messagePositions"))]
    MessagePositions(Box<crate::types::MessagePositions>),
}

impl MessagePositions {
    /// Convenience constructor to create a [`MessagePositions::MessagePositions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_positions(val: crate::types::MessagePositions) -> Self {
        Self::MessagePositions(Box::new(val))
    }

}

/// Converts a [`crate::types::MessagePositions`] into [`MessagePositions`].
impl From<crate::types::MessagePositions> for MessagePositions {
    fn from(val: crate::types::MessagePositions) -> Self {
        Self::MessagePositions(Box::new(val))
    }
}

/// TDLib `MessageCalendarDay` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageCalendarDay {
    /// Contains information about found messages sent on a specific day
    #[serde(rename(serialize = "messageCalendarDay", deserialize = "messageCalendarDay"))]
    MessageCalendarDay(Box<crate::types::MessageCalendarDay>),
}

impl MessageCalendarDay {
    /// Convenience constructor to create a [`MessageCalendarDay::MessageCalendarDay`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_calendar_day(val: crate::types::MessageCalendarDay) -> Self {
        Self::MessageCalendarDay(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageCalendarDay`] into [`MessageCalendarDay`].
impl From<crate::types::MessageCalendarDay> for MessageCalendarDay {
    fn from(val: crate::types::MessageCalendarDay) -> Self {
        Self::MessageCalendarDay(Box::new(val))
    }
}

/// TDLib `MessageCalendar` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageCalendar {
    /// Contains information about found messages, split by days according to the option "utc_time_offset"
    #[serde(rename(serialize = "messageCalendar", deserialize = "messageCalendar"))]
    MessageCalendar(Box<crate::types::MessageCalendar>),
}

impl MessageCalendar {
    /// Convenience constructor to create a [`MessageCalendar::MessageCalendar`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_calendar(val: crate::types::MessageCalendar) -> Self {
        Self::MessageCalendar(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageCalendar`] into [`MessageCalendar`].
impl From<crate::types::MessageCalendar> for MessageCalendar {
    fn from(val: crate::types::MessageCalendar) -> Self {
        Self::MessageCalendar(Box::new(val))
    }
}

/// Describes source of a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSource {
    /// The message is from a chat history
    #[serde(rename(serialize = "messageSourceChatHistory", deserialize = "messageSourceChatHistory"))]
    ChatHistory,
    /// The message is from history of a message thread
    #[serde(rename(serialize = "messageSourceMessageThreadHistory", deserialize = "messageSourceMessageThreadHistory"))]
    MessageThreadHistory,
    /// The message is from history of a forum topic
    #[serde(rename(serialize = "messageSourceForumTopicHistory", deserialize = "messageSourceForumTopicHistory"))]
    ForumTopicHistory,
    /// The message is from history of a topic in a channel direct messages chat administered by the current user
    #[serde(rename(serialize = "messageSourceDirectMessagesChatTopicHistory", deserialize = "messageSourceDirectMessagesChatTopicHistory"))]
    DirectMessagesChatTopicHistory,
    /// The message is from chat, message thread or forum topic history preview
    #[serde(rename(serialize = "messageSourceHistoryPreview", deserialize = "messageSourceHistoryPreview"))]
    HistoryPreview,
    /// The message is from a chat list or a forum topic list
    #[serde(rename(serialize = "messageSourceChatList", deserialize = "messageSourceChatList"))]
    ChatList,
    /// The message is from search results, including file downloads, local file list, outgoing document messages, calendar
    #[serde(rename(serialize = "messageSourceSearch", deserialize = "messageSourceSearch"))]
    Search,
    /// The message is from a chat event log
    #[serde(rename(serialize = "messageSourceChatEventLog", deserialize = "messageSourceChatEventLog"))]
    ChatEventLog,
    /// The message is from a notification
    #[serde(rename(serialize = "messageSourceNotification", deserialize = "messageSourceNotification"))]
    Notification,
    /// The message was screenshotted; the source must be used only if the message content was visible during the screenshot
    #[serde(rename(serialize = "messageSourceScreenshot", deserialize = "messageSourceScreenshot"))]
    Screenshot,
    /// The message is from some other source
    #[serde(rename(serialize = "messageSourceOther", deserialize = "messageSourceOther"))]
    Other,
}

/// TDLib `SponsoredMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SponsoredMessage {
    /// Describes a sponsored message
    #[serde(rename(serialize = "sponsoredMessage", deserialize = "sponsoredMessage"))]
    SponsoredMessage(Box<crate::types::SponsoredMessage>),
}

impl SponsoredMessage {
    /// Convenience constructor to create a [`SponsoredMessage::SponsoredMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sponsored_message(val: crate::types::SponsoredMessage) -> Self {
        Self::SponsoredMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::SponsoredMessage`] into [`SponsoredMessage`].
impl From<crate::types::SponsoredMessage> for SponsoredMessage {
    fn from(val: crate::types::SponsoredMessage) -> Self {
        Self::SponsoredMessage(Box::new(val))
    }
}

/// TDLib `SponsoredMessages` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SponsoredMessages {
    /// Contains a list of sponsored messages
    #[serde(rename(serialize = "sponsoredMessages", deserialize = "sponsoredMessages"))]
    SponsoredMessages(Box<crate::types::SponsoredMessages>),
}

impl SponsoredMessages {
    /// Convenience constructor to create a [`SponsoredMessages::SponsoredMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sponsored_messages(val: crate::types::SponsoredMessages) -> Self {
        Self::SponsoredMessages(Box::new(val))
    }

}

/// Converts a [`crate::types::SponsoredMessages`] into [`SponsoredMessages`].
impl From<crate::types::SponsoredMessages> for SponsoredMessages {
    fn from(val: crate::types::SponsoredMessages) -> Self {
        Self::SponsoredMessages(Box::new(val))
    }
}

/// Content of the message draft
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DraftMessageContent {
    /// A text message draft
    #[serde(rename(serialize = "draftMessageContentText", deserialize = "draftMessageContentText"))]
    Text(Box<crate::types::DraftMessageContentText>),
    /// A rich message draft; not supported in setChatDraftMessage
    #[serde(rename(serialize = "draftMessageContentRichMessage", deserialize = "draftMessageContentRichMessage"))]
    RichMessage(Box<crate::types::DraftMessageContentRichMessage>),
    /// A rich message draft; only for setChatDraftMessage
    #[serde(rename(serialize = "draftMessageContentInputRichMessage", deserialize = "draftMessageContentInputRichMessage"))]
    InputRichMessage(Box<crate::types::DraftMessageContentInputRichMessage>),
    /// A video note message draft
    #[serde(rename(serialize = "draftMessageContentVideoNote", deserialize = "draftMessageContentVideoNote"))]
    VideoNote(Box<crate::types::DraftMessageContentVideoNote>),
    /// A voice note message draft
    #[serde(rename(serialize = "draftMessageContentVoiceNote", deserialize = "draftMessageContentVoiceNote"))]
    VoiceNote(Box<crate::types::DraftMessageContentVoiceNote>),
}

impl DraftMessageContent {
    /// Convenience constructor to create a [`DraftMessageContent::Text`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text(val: crate::types::DraftMessageContentText) -> Self {
        Self::Text(Box::new(val))
    }

    /// Convenience constructor to create a [`DraftMessageContent::RichMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn rich_message(val: crate::types::DraftMessageContentRichMessage) -> Self {
        Self::RichMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`DraftMessageContent::InputRichMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_rich_message(val: crate::types::DraftMessageContentInputRichMessage) -> Self {
        Self::InputRichMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`DraftMessageContent::VideoNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video_note(val: crate::types::DraftMessageContentVideoNote) -> Self {
        Self::VideoNote(Box::new(val))
    }

    /// Convenience constructor to create a [`DraftMessageContent::VoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn voice_note(val: crate::types::DraftMessageContentVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }

}

/// Converts a [`crate::types::DraftMessageContentText`] into [`DraftMessageContent`].
impl From<crate::types::DraftMessageContentText> for DraftMessageContent {
    fn from(val: crate::types::DraftMessageContentText) -> Self {
        Self::Text(Box::new(val))
    }
}

/// Converts a [`crate::types::DraftMessageContentRichMessage`] into [`DraftMessageContent`].
impl From<crate::types::DraftMessageContentRichMessage> for DraftMessageContent {
    fn from(val: crate::types::DraftMessageContentRichMessage) -> Self {
        Self::RichMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::DraftMessageContentInputRichMessage`] into [`DraftMessageContent`].
impl From<crate::types::DraftMessageContentInputRichMessage> for DraftMessageContent {
    fn from(val: crate::types::DraftMessageContentInputRichMessage) -> Self {
        Self::InputRichMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::DraftMessageContentVideoNote`] into [`DraftMessageContent`].
impl From<crate::types::DraftMessageContentVideoNote> for DraftMessageContent {
    fn from(val: crate::types::DraftMessageContentVideoNote) -> Self {
        Self::VideoNote(Box::new(val))
    }
}

/// Converts a [`crate::types::DraftMessageContentVoiceNote`] into [`DraftMessageContent`].
impl From<crate::types::DraftMessageContentVoiceNote> for DraftMessageContent {
    fn from(val: crate::types::DraftMessageContentVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }
}

/// TDLib `DraftMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DraftMessage {
    /// Contains information about a message draft
    #[serde(rename(serialize = "draftMessage", deserialize = "draftMessage"))]
    DraftMessage(Box<crate::types::DraftMessage>),
}

impl DraftMessage {
    /// Convenience constructor to create a [`DraftMessage::DraftMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn draft_message(val: crate::types::DraftMessage) -> Self {
        Self::DraftMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::DraftMessage`] into [`DraftMessage`].
impl From<crate::types::DraftMessage> for DraftMessage {
    fn from(val: crate::types::DraftMessage) -> Self {
        Self::DraftMessage(Box::new(val))
    }
}

/// Describes reactions available in the chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAvailableReactions {
    /// All reactions are available in the chat, excluding the paid reaction and custom reactions in channel chats
    #[serde(rename(serialize = "chatAvailableReactionsAll", deserialize = "chatAvailableReactionsAll"))]
    All(Box<crate::types::ChatAvailableReactionsAll>),
    /// Only specific reactions are available in the chat
    #[serde(rename(serialize = "chatAvailableReactionsSome", deserialize = "chatAvailableReactionsSome"))]
    Some(Box<crate::types::ChatAvailableReactionsSome>),
}

impl ChatAvailableReactions {
    /// Convenience constructor to create a [`ChatAvailableReactions::All`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn all(val: crate::types::ChatAvailableReactionsAll) -> Self {
        Self::All(Box::new(val))
    }

    /// Convenience constructor to create a [`ChatAvailableReactions::Some`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn some(val: crate::types::ChatAvailableReactionsSome) -> Self {
        Self::Some(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatAvailableReactionsAll`] into [`ChatAvailableReactions`].
impl From<crate::types::ChatAvailableReactionsAll> for ChatAvailableReactions {
    fn from(val: crate::types::ChatAvailableReactionsAll) -> Self {
        Self::All(Box::new(val))
    }
}

/// Converts a [`crate::types::ChatAvailableReactionsSome`] into [`ChatAvailableReactions`].
impl From<crate::types::ChatAvailableReactionsSome> for ChatAvailableReactions {
    fn from(val: crate::types::ChatAvailableReactionsSome) -> Self {
        Self::Some(Box::new(val))
    }
}

/// TDLib `SavedMessagesTag` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedMessagesTag {
    /// Represents a tag used in Saved Messages or a Saved Messages topic
    #[serde(rename(serialize = "savedMessagesTag", deserialize = "savedMessagesTag"))]
    SavedMessagesTag(Box<crate::types::SavedMessagesTag>),
}

impl SavedMessagesTag {
    /// Convenience constructor to create a [`SavedMessagesTag::SavedMessagesTag`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_messages_tag(val: crate::types::SavedMessagesTag) -> Self {
        Self::SavedMessagesTag(Box::new(val))
    }

}

/// Converts a [`crate::types::SavedMessagesTag`] into [`SavedMessagesTag`].
impl From<crate::types::SavedMessagesTag> for SavedMessagesTag {
    fn from(val: crate::types::SavedMessagesTag) -> Self {
        Self::SavedMessagesTag(Box::new(val))
    }
}

/// TDLib `SavedMessagesTags` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedMessagesTags {
    /// Contains a list of tags used in Saved Messages
    #[serde(rename(serialize = "savedMessagesTags", deserialize = "savedMessagesTags"))]
    SavedMessagesTags(Box<crate::types::SavedMessagesTags>),
}

impl SavedMessagesTags {
    /// Convenience constructor to create a [`SavedMessagesTags::SavedMessagesTags`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn saved_messages_tags(val: crate::types::SavedMessagesTags) -> Self {
        Self::SavedMessagesTags(Box::new(val))
    }

}

/// Converts a [`crate::types::SavedMessagesTags`] into [`SavedMessagesTags`].
impl From<crate::types::SavedMessagesTags> for SavedMessagesTags {
    fn from(val: crate::types::SavedMessagesTags) -> Self {
        Self::SavedMessagesTags(Box::new(val))
    }
}

/// TDLib `MessageThreadInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageThreadInfo {
    /// Contains information about a message thread
    #[serde(rename(serialize = "messageThreadInfo", deserialize = "messageThreadInfo"))]
    MessageThreadInfo(Box<crate::types::MessageThreadInfo>),
}

impl MessageThreadInfo {
    /// Convenience constructor to create a [`MessageThreadInfo::MessageThreadInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_thread_info(val: crate::types::MessageThreadInfo) -> Self {
        Self::MessageThreadInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageThreadInfo`] into [`MessageThreadInfo`].
impl From<crate::types::MessageThreadInfo> for MessageThreadInfo {
    fn from(val: crate::types::MessageThreadInfo) -> Self {
        Self::MessageThreadInfo(Box::new(val))
    }
}

/// Describes a formatted text object
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum RichText {
    /// A plain text
    #[serde(rename(serialize = "richTextPlain", deserialize = "richTextPlain"))]
    Plain(Box<crate::types::RichTextPlain>),
    /// A bold rich text
    #[serde(rename(serialize = "richTextBold", deserialize = "richTextBold"))]
    Bold(Box<crate::types::RichTextBold>),
    /// An italicized rich text
    #[serde(rename(serialize = "richTextItalic", deserialize = "richTextItalic"))]
    Italic(Box<crate::types::RichTextItalic>),
    /// An underlined rich text
    #[serde(rename(serialize = "richTextUnderline", deserialize = "richTextUnderline"))]
    Underline(Box<crate::types::RichTextUnderline>),
    /// A strikethrough rich text
    #[serde(rename(serialize = "richTextStrikethrough", deserialize = "richTextStrikethrough"))]
    Strikethrough(Box<crate::types::RichTextStrikethrough>),
    /// A spoilered rich text
    #[serde(rename(serialize = "richTextSpoiler", deserialize = "richTextSpoiler"))]
    Spoiler(Box<crate::types::RichTextSpoiler>),
    /// A subscript rich text
    #[serde(rename(serialize = "richTextSubscript", deserialize = "richTextSubscript"))]
    Subscript(Box<crate::types::RichTextSubscript>),
    /// A superscript rich text
    #[serde(rename(serialize = "richTextSuperscript", deserialize = "richTextSuperscript"))]
    Superscript(Box<crate::types::RichTextSuperscript>),
    /// A marked rich text
    #[serde(rename(serialize = "richTextMarked", deserialize = "richTextMarked"))]
    Marked(Box<crate::types::RichTextMarked>),
    /// A date and time
    #[serde(rename(serialize = "richTextDateTime", deserialize = "richTextDateTime"))]
    DateTime(Box<crate::types::RichTextDateTime>),
    /// A mention of a Telegram user or chat by a username
    #[serde(rename(serialize = "richTextMention", deserialize = "richTextMention"))]
    Mention(Box<crate::types::RichTextMention>),
    /// A hashtag
    #[serde(rename(serialize = "richTextHashtag", deserialize = "richTextHashtag"))]
    Hashtag(Box<crate::types::RichTextHashtag>),
    /// A cashtag
    #[serde(rename(serialize = "richTextCashtag", deserialize = "richTextCashtag"))]
    Cashtag(Box<crate::types::RichTextCashtag>),
    /// A bank card number
    #[serde(rename(serialize = "richTextBankCardNumber", deserialize = "richTextBankCardNumber"))]
    BankCardNumber(Box<crate::types::RichTextBankCardNumber>),
    /// A bot command
    #[serde(rename(serialize = "richTextBotCommand", deserialize = "richTextBotCommand"))]
    BotCommand(Box<crate::types::RichTextBotCommand>),
    /// A fixed-width rich text
    #[serde(rename(serialize = "richTextFixed", deserialize = "richTextFixed"))]
    Fixed(Box<crate::types::RichTextFixed>),
    /// A rich text that serves as a mention of a user
    #[serde(rename(serialize = "richTextMentionName", deserialize = "richTextMentionName"))]
    MentionName(Box<crate::types::RichTextMentionName>),
    /// A rich text URL link
    #[serde(rename(serialize = "richTextUrl", deserialize = "richTextUrl"))]
    Url(Box<crate::types::RichTextUrl>),
    /// A rich text email address
    #[serde(rename(serialize = "richTextEmailAddress", deserialize = "richTextEmailAddress"))]
    EmailAddress(Box<crate::types::RichTextEmailAddress>),
    /// A rich text phone number
    #[serde(rename(serialize = "richTextPhoneNumber", deserialize = "richTextPhoneNumber"))]
    PhoneNumber(Box<crate::types::RichTextPhoneNumber>),
    /// A custom emoji
    #[serde(rename(serialize = "richTextCustomEmoji", deserialize = "richTextCustomEmoji"))]
    CustomEmoji(Box<crate::types::RichTextCustomEmoji>),
    /// A small image inside the text; instant view only
    #[serde(rename(serialize = "richTextIcon", deserialize = "richTextIcon"))]
    Icon(Box<crate::types::RichTextIcon>),
    /// A mathematical expression
    #[serde(rename(serialize = "richTextMathematicalExpression", deserialize = "richTextMathematicalExpression"))]
    MathematicalExpression(Box<crate::types::RichTextMathematicalExpression>),
    /// A button
    #[serde(rename(serialize = "richTextButton", deserialize = "richTextButton"))]
    Button(Box<crate::types::RichTextButton>),
    /// A rich text replacing another rich text; not supported in inputRichMessage
    #[serde(rename(serialize = "richTextDiff", deserialize = "richTextDiff"))]
    Diff(Box<crate::types::RichTextDiff>),
    /// A reference
    #[serde(rename(serialize = "richTextReference", deserialize = "richTextReference"))]
    Reference(Box<crate::types::RichTextReference>),
    /// A link to a reference on the same page
    #[serde(rename(serialize = "richTextReferenceLink", deserialize = "richTextReferenceLink"))]
    ReferenceLink(Box<crate::types::RichTextReferenceLink>),
    /// An anchor
    #[serde(rename(serialize = "richTextAnchor", deserialize = "richTextAnchor"))]
    Anchor(Box<crate::types::RichTextAnchor>),
    /// A link to an anchor on the same page
    #[serde(rename(serialize = "richTextAnchorLink", deserialize = "richTextAnchorLink"))]
    AnchorLink(Box<crate::types::RichTextAnchorLink>),
    /// A concatenation of rich texts
    #[serde(rename(serialize = "richTexts", deserialize = "richTexts"))]
    RichTexts(Box<crate::types::RichTexts>),
}

impl RichText {
    /// Convenience constructor to create a [`RichText::Plain`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn plain(val: crate::types::RichTextPlain) -> Self {
        Self::Plain(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Bold`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bold(val: crate::types::RichTextBold) -> Self {
        Self::Bold(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Italic`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn italic(val: crate::types::RichTextItalic) -> Self {
        Self::Italic(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Underline`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn underline(val: crate::types::RichTextUnderline) -> Self {
        Self::Underline(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Strikethrough`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn strikethrough(val: crate::types::RichTextStrikethrough) -> Self {
        Self::Strikethrough(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Spoiler`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn spoiler(val: crate::types::RichTextSpoiler) -> Self {
        Self::Spoiler(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Subscript`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn subscript(val: crate::types::RichTextSubscript) -> Self {
        Self::Subscript(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Superscript`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn superscript(val: crate::types::RichTextSuperscript) -> Self {
        Self::Superscript(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Marked`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn marked(val: crate::types::RichTextMarked) -> Self {
        Self::Marked(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::DateTime`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn date_time(val: crate::types::RichTextDateTime) -> Self {
        Self::DateTime(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Mention`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mention(val: crate::types::RichTextMention) -> Self {
        Self::Mention(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Hashtag`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn hashtag(val: crate::types::RichTextHashtag) -> Self {
        Self::Hashtag(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Cashtag`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn cashtag(val: crate::types::RichTextCashtag) -> Self {
        Self::Cashtag(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::BankCardNumber`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bank_card_number(val: crate::types::RichTextBankCardNumber) -> Self {
        Self::BankCardNumber(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::BotCommand`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot_command(val: crate::types::RichTextBotCommand) -> Self {
        Self::BotCommand(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Fixed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fixed(val: crate::types::RichTextFixed) -> Self {
        Self::Fixed(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::MentionName`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mention_name(val: crate::types::RichTextMentionName) -> Self {
        Self::MentionName(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Url`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn url(val: crate::types::RichTextUrl) -> Self {
        Self::Url(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::EmailAddress`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn email_address(val: crate::types::RichTextEmailAddress) -> Self {
        Self::EmailAddress(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::PhoneNumber`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn phone_number(val: crate::types::RichTextPhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::CustomEmoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom_emoji(val: crate::types::RichTextCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Icon`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn icon(val: crate::types::RichTextIcon) -> Self {
        Self::Icon(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::MathematicalExpression`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mathematical_expression(val: crate::types::RichTextMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Button`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn button(val: crate::types::RichTextButton) -> Self {
        Self::Button(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Diff`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn diff(val: crate::types::RichTextDiff) -> Self {
        Self::Diff(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Reference`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn reference(val: crate::types::RichTextReference) -> Self {
        Self::Reference(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::ReferenceLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn reference_link(val: crate::types::RichTextReferenceLink) -> Self {
        Self::ReferenceLink(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::Anchor`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn anchor(val: crate::types::RichTextAnchor) -> Self {
        Self::Anchor(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::AnchorLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn anchor_link(val: crate::types::RichTextAnchorLink) -> Self {
        Self::AnchorLink(Box::new(val))
    }

    /// Convenience constructor to create a [`RichText::RichTexts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn rich_texts(val: crate::types::RichTexts) -> Self {
        Self::RichTexts(Box::new(val))
    }

}

/// Converts a [`crate::types::RichTextPlain`] into [`RichText`].
impl From<crate::types::RichTextPlain> for RichText {
    fn from(val: crate::types::RichTextPlain) -> Self {
        Self::Plain(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextBold`] into [`RichText`].
impl From<crate::types::RichTextBold> for RichText {
    fn from(val: crate::types::RichTextBold) -> Self {
        Self::Bold(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextItalic`] into [`RichText`].
impl From<crate::types::RichTextItalic> for RichText {
    fn from(val: crate::types::RichTextItalic) -> Self {
        Self::Italic(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextUnderline`] into [`RichText`].
impl From<crate::types::RichTextUnderline> for RichText {
    fn from(val: crate::types::RichTextUnderline) -> Self {
        Self::Underline(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextStrikethrough`] into [`RichText`].
impl From<crate::types::RichTextStrikethrough> for RichText {
    fn from(val: crate::types::RichTextStrikethrough) -> Self {
        Self::Strikethrough(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextSpoiler`] into [`RichText`].
impl From<crate::types::RichTextSpoiler> for RichText {
    fn from(val: crate::types::RichTextSpoiler) -> Self {
        Self::Spoiler(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextSubscript`] into [`RichText`].
impl From<crate::types::RichTextSubscript> for RichText {
    fn from(val: crate::types::RichTextSubscript) -> Self {
        Self::Subscript(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextSuperscript`] into [`RichText`].
impl From<crate::types::RichTextSuperscript> for RichText {
    fn from(val: crate::types::RichTextSuperscript) -> Self {
        Self::Superscript(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextMarked`] into [`RichText`].
impl From<crate::types::RichTextMarked> for RichText {
    fn from(val: crate::types::RichTextMarked) -> Self {
        Self::Marked(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextDateTime`] into [`RichText`].
impl From<crate::types::RichTextDateTime> for RichText {
    fn from(val: crate::types::RichTextDateTime) -> Self {
        Self::DateTime(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextMention`] into [`RichText`].
impl From<crate::types::RichTextMention> for RichText {
    fn from(val: crate::types::RichTextMention) -> Self {
        Self::Mention(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextHashtag`] into [`RichText`].
impl From<crate::types::RichTextHashtag> for RichText {
    fn from(val: crate::types::RichTextHashtag) -> Self {
        Self::Hashtag(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextCashtag`] into [`RichText`].
impl From<crate::types::RichTextCashtag> for RichText {
    fn from(val: crate::types::RichTextCashtag) -> Self {
        Self::Cashtag(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextBankCardNumber`] into [`RichText`].
impl From<crate::types::RichTextBankCardNumber> for RichText {
    fn from(val: crate::types::RichTextBankCardNumber) -> Self {
        Self::BankCardNumber(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextBotCommand`] into [`RichText`].
impl From<crate::types::RichTextBotCommand> for RichText {
    fn from(val: crate::types::RichTextBotCommand) -> Self {
        Self::BotCommand(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextFixed`] into [`RichText`].
impl From<crate::types::RichTextFixed> for RichText {
    fn from(val: crate::types::RichTextFixed) -> Self {
        Self::Fixed(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextMentionName`] into [`RichText`].
impl From<crate::types::RichTextMentionName> for RichText {
    fn from(val: crate::types::RichTextMentionName) -> Self {
        Self::MentionName(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextUrl`] into [`RichText`].
impl From<crate::types::RichTextUrl> for RichText {
    fn from(val: crate::types::RichTextUrl) -> Self {
        Self::Url(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextEmailAddress`] into [`RichText`].
impl From<crate::types::RichTextEmailAddress> for RichText {
    fn from(val: crate::types::RichTextEmailAddress) -> Self {
        Self::EmailAddress(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextPhoneNumber`] into [`RichText`].
impl From<crate::types::RichTextPhoneNumber> for RichText {
    fn from(val: crate::types::RichTextPhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextCustomEmoji`] into [`RichText`].
impl From<crate::types::RichTextCustomEmoji> for RichText {
    fn from(val: crate::types::RichTextCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextIcon`] into [`RichText`].
impl From<crate::types::RichTextIcon> for RichText {
    fn from(val: crate::types::RichTextIcon) -> Self {
        Self::Icon(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextMathematicalExpression`] into [`RichText`].
impl From<crate::types::RichTextMathematicalExpression> for RichText {
    fn from(val: crate::types::RichTextMathematicalExpression) -> Self {
        Self::MathematicalExpression(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextButton`] into [`RichText`].
impl From<crate::types::RichTextButton> for RichText {
    fn from(val: crate::types::RichTextButton) -> Self {
        Self::Button(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextDiff`] into [`RichText`].
impl From<crate::types::RichTextDiff> for RichText {
    fn from(val: crate::types::RichTextDiff) -> Self {
        Self::Diff(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextReference`] into [`RichText`].
impl From<crate::types::RichTextReference> for RichText {
    fn from(val: crate::types::RichTextReference) -> Self {
        Self::Reference(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextReferenceLink`] into [`RichText`].
impl From<crate::types::RichTextReferenceLink> for RichText {
    fn from(val: crate::types::RichTextReferenceLink) -> Self {
        Self::ReferenceLink(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextAnchor`] into [`RichText`].
impl From<crate::types::RichTextAnchor> for RichText {
    fn from(val: crate::types::RichTextAnchor) -> Self {
        Self::Anchor(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTextAnchorLink`] into [`RichText`].
impl From<crate::types::RichTextAnchorLink> for RichText {
    fn from(val: crate::types::RichTextAnchorLink) -> Self {
        Self::AnchorLink(Box::new(val))
    }
}

/// Converts a [`crate::types::RichTexts`] into [`RichText`].
impl From<crate::types::RichTexts> for RichText {
    fn from(val: crate::types::RichTexts) -> Self {
        Self::RichTexts(Box::new(val))
    }
}

/// TDLib `PageBlockCaption` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockCaption {
    /// Contains a caption of another block
    #[serde(rename(serialize = "pageBlockCaption", deserialize = "pageBlockCaption"))]
    PageBlockCaption(Box<crate::types::PageBlockCaption>),
}

impl PageBlockCaption {
    /// Convenience constructor to create a [`PageBlockCaption::PageBlockCaption`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn page_block_caption(val: crate::types::PageBlockCaption) -> Self {
        Self::PageBlockCaption(Box::new(val))
    }

}

/// Converts a [`crate::types::PageBlockCaption`] into [`PageBlockCaption`].
impl From<crate::types::PageBlockCaption> for PageBlockCaption {
    fn from(val: crate::types::PageBlockCaption) -> Self {
        Self::PageBlockCaption(Box::new(val))
    }
}

/// Contains the content of a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageContent {
    /// A text message
    #[serde(rename(serialize = "messageText", deserialize = "messageText"))]
    MessageText(Box<crate::types::MessageText>),
    /// A rich message; the message can have multiple media of the same type, all of which must be shown in the corresponding profile tab
    #[serde(rename(serialize = "messageRichMessage", deserialize = "messageRichMessage"))]
    MessageRichMessage(Box<crate::types::MessageRichMessage>),
    /// An animation message (GIF-style).
    #[serde(rename(serialize = "messageAnimation", deserialize = "messageAnimation"))]
    MessageAnimation(Box<crate::types::MessageAnimation>),
    /// An audio message
    #[serde(rename(serialize = "messageAudio", deserialize = "messageAudio"))]
    MessageAudio(Box<crate::types::MessageAudio>),
    /// A document message (general file)
    #[serde(rename(serialize = "messageDocument", deserialize = "messageDocument"))]
    MessageDocument(Box<crate::types::MessageDocument>),
    /// A message with paid media
    #[serde(rename(serialize = "messagePaidMedia", deserialize = "messagePaidMedia"))]
    MessagePaidMedia(Box<crate::types::MessagePaidMedia>),
    /// A photo message
    #[serde(rename(serialize = "messagePhoto", deserialize = "messagePhoto"))]
    MessagePhoto(Box<crate::types::MessagePhoto>),
    /// A sticker message
    #[serde(rename(serialize = "messageSticker", deserialize = "messageSticker"))]
    MessageSticker(Box<crate::types::MessageSticker>),
    /// A video message
    #[serde(rename(serialize = "messageVideo", deserialize = "messageVideo"))]
    MessageVideo(Box<crate::types::MessageVideo>),
    /// A video note message
    #[serde(rename(serialize = "messageVideoNote", deserialize = "messageVideoNote"))]
    MessageVideoNote(Box<crate::types::MessageVideoNote>),
    /// A voice note message
    #[serde(rename(serialize = "messageVoiceNote", deserialize = "messageVoiceNote"))]
    MessageVoiceNote(Box<crate::types::MessageVoiceNote>),
    /// A self-destructed photo message
    #[serde(rename(serialize = "messageExpiredPhoto", deserialize = "messageExpiredPhoto"))]
    MessageExpiredPhoto,
    /// A self-destructed video message
    #[serde(rename(serialize = "messageExpiredVideo", deserialize = "messageExpiredVideo"))]
    MessageExpiredVideo,
    /// A self-destructed video note message
    #[serde(rename(serialize = "messageExpiredVideoNote", deserialize = "messageExpiredVideoNote"))]
    MessageExpiredVideoNote,
    /// A self-destructed voice note message
    #[serde(rename(serialize = "messageExpiredVoiceNote", deserialize = "messageExpiredVoiceNote"))]
    MessageExpiredVoiceNote,
    /// A message with a live location
    #[serde(rename(serialize = "messageLiveLocation", deserialize = "messageLiveLocation"))]
    MessageLiveLocation(Box<crate::types::MessageLiveLocation>),
    /// A message with a location
    #[serde(rename(serialize = "messageLocation", deserialize = "messageLocation"))]
    MessageLocation(Box<crate::types::MessageLocation>),
    /// A message with information about a venue
    #[serde(rename(serialize = "messageVenue", deserialize = "messageVenue"))]
    MessageVenue(Box<crate::types::MessageVenue>),
    /// A message with a user contact
    #[serde(rename(serialize = "messageContact", deserialize = "messageContact"))]
    MessageContact(Box<crate::types::MessageContact>),
    /// A message with an animated emoji
    #[serde(rename(serialize = "messageAnimatedEmoji", deserialize = "messageAnimatedEmoji"))]
    MessageAnimatedEmoji(Box<crate::types::MessageAnimatedEmoji>),
    /// A dice message. The dice value is randomly generated by the server
    #[serde(rename(serialize = "messageDice", deserialize = "messageDice"))]
    MessageDice(Box<crate::types::MessageDice>),
    /// A message with a game
    #[serde(rename(serialize = "messageGame", deserialize = "messageGame"))]
    MessageGame(Box<crate::types::MessageGame>),
    /// A message with a poll
    #[serde(rename(serialize = "messagePoll", deserialize = "messagePoll"))]
    MessagePoll(Box<crate::types::MessagePoll>),
    /// A stake dice message. The dice value is randomly generated by the server
    #[serde(rename(serialize = "messageStakeDice", deserialize = "messageStakeDice"))]
    MessageStakeDice(Box<crate::types::MessageStakeDice>),
    /// A message with a forwarded story
    #[serde(rename(serialize = "messageStory", deserialize = "messageStory"))]
    MessageStory(Box<crate::types::MessageStory>),
    /// A message with a checklist
    #[serde(rename(serialize = "messageChecklist", deserialize = "messageChecklist"))]
    MessageChecklist(Box<crate::types::MessageChecklist>),
    /// A message with an invoice from a bot. Use getInternalLink with internalLinkTypeBotStart to share the invoice
    #[serde(rename(serialize = "messageInvoice", deserialize = "messageInvoice"))]
    MessageInvoice(Box<crate::types::MessageInvoice>),
    /// A message with information about an ended call
    #[serde(rename(serialize = "messageCall", deserialize = "messageCall"))]
    MessageCall(Box<crate::types::MessageCall>),
    /// A message with information about a group call not bound to a chat. If the message is incoming, the call isn't active, isn't missed, and has no duration,
    /// and getOption("can_accept_calls") is true, then incoming call screen must be shown to the user. Use getGroupCallParticipants to show current group call participants on the screen.
    /// Use joinGroupCall to accept the call or declineGroupCallInvitation to decline it. If the call become active or missed, then the call screen must be hidden
    #[serde(rename(serialize = "messageGroupCall", deserialize = "messageGroupCall"))]
    MessageGroupCall(Box<crate::types::MessageGroupCall>),
    /// A new video chat was scheduled
    #[serde(rename(serialize = "messageVideoChatScheduled", deserialize = "messageVideoChatScheduled"))]
    MessageVideoChatScheduled(Box<crate::types::MessageVideoChatScheduled>),
    /// A newly created video chat
    #[serde(rename(serialize = "messageVideoChatStarted", deserialize = "messageVideoChatStarted"))]
    MessageVideoChatStarted(Box<crate::types::MessageVideoChatStarted>),
    /// A message with information about an ended video chat
    #[serde(rename(serialize = "messageVideoChatEnded", deserialize = "messageVideoChatEnded"))]
    MessageVideoChatEnded(Box<crate::types::MessageVideoChatEnded>),
    /// A message with information about an invitation to a video chat
    #[serde(rename(serialize = "messageInviteVideoChatParticipants", deserialize = "messageInviteVideoChatParticipants"))]
    MessageInviteVideoChatParticipants(Box<crate::types::MessageInviteVideoChatParticipants>),
    /// A message with information about an added poll option
    #[serde(rename(serialize = "messagePollOptionAdded", deserialize = "messagePollOptionAdded"))]
    MessagePollOptionAdded(Box<crate::types::MessagePollOptionAdded>),
    /// A message with information about a deleted poll option
    #[serde(rename(serialize = "messagePollOptionDeleted", deserialize = "messagePollOptionDeleted"))]
    MessagePollOptionDeleted(Box<crate::types::MessagePollOptionDeleted>),
    /// A newly created basic group
    #[serde(rename(serialize = "messageBasicGroupChatCreate", deserialize = "messageBasicGroupChatCreate"))]
    MessageBasicGroupChatCreate(Box<crate::types::MessageBasicGroupChatCreate>),
    /// A newly created supergroup or channel
    #[serde(rename(serialize = "messageSupergroupChatCreate", deserialize = "messageSupergroupChatCreate"))]
    MessageSupergroupChatCreate(Box<crate::types::MessageSupergroupChatCreate>),
    /// An updated chat title
    #[serde(rename(serialize = "messageChatChangeTitle", deserialize = "messageChatChangeTitle"))]
    MessageChatChangeTitle(Box<crate::types::MessageChatChangeTitle>),
    /// An updated chat photo
    #[serde(rename(serialize = "messageChatChangePhoto", deserialize = "messageChatChangePhoto"))]
    MessageChatChangePhoto(Box<crate::types::MessageChatChangePhoto>),
    /// A deleted chat photo
    #[serde(rename(serialize = "messageChatDeletePhoto", deserialize = "messageChatDeletePhoto"))]
    MessageChatDeletePhoto,
    /// The owner of the chat has left
    #[serde(rename(serialize = "messageChatOwnerLeft", deserialize = "messageChatOwnerLeft"))]
    MessageChatOwnerLeft(Box<crate::types::MessageChatOwnerLeft>),
    /// The owner of the chat has changed
    #[serde(rename(serialize = "messageChatOwnerChanged", deserialize = "messageChatOwnerChanged"))]
    MessageChatOwnerChanged(Box<crate::types::MessageChatOwnerChanged>),
    /// Chat has_protected_content setting was changed or request to change it was rejected
    #[serde(rename(serialize = "messageChatHasProtectedContentToggled", deserialize = "messageChatHasProtectedContentToggled"))]
    MessageChatHasProtectedContentToggled(Box<crate::types::MessageChatHasProtectedContentToggled>),
    /// Chat has_protected_content setting was requested to be disabled
    #[serde(rename(serialize = "messageChatHasProtectedContentDisableRequested", deserialize = "messageChatHasProtectedContentDisableRequested"))]
    MessageChatHasProtectedContentDisableRequested(Box<crate::types::MessageChatHasProtectedContentDisableRequested>),
    /// New chat members were added
    #[serde(rename(serialize = "messageChatAddMembers", deserialize = "messageChatAddMembers"))]
    MessageChatAddMembers(Box<crate::types::MessageChatAddMembers>),
    /// A new member joined the chat via an invite link
    #[serde(rename(serialize = "messageChatJoinByLink", deserialize = "messageChatJoinByLink"))]
    MessageChatJoinByLink,
    /// A new member was accepted to the chat by an administrator
    #[serde(rename(serialize = "messageChatJoinByRequest", deserialize = "messageChatJoinByRequest"))]
    MessageChatJoinByRequest,
    /// A new member joined the chat from a community
    #[serde(rename(serialize = "messageChatJoinFromCommunity", deserialize = "messageChatJoinFromCommunity"))]
    MessageChatJoinFromCommunity(Box<crate::types::MessageChatJoinFromCommunity>),
    /// A chat member was deleted
    #[serde(rename(serialize = "messageChatDeleteMember", deserialize = "messageChatDeleteMember"))]
    MessageChatDeleteMember(Box<crate::types::MessageChatDeleteMember>),
    /// The chat was added to a community
    #[serde(rename(serialize = "messageChatAddedToCommunity", deserialize = "messageChatAddedToCommunity"))]
    MessageChatAddedToCommunity(Box<crate::types::MessageChatAddedToCommunity>),
    /// The chat was removed from a community
    #[serde(rename(serialize = "messageChatRemovedFromCommunity", deserialize = "messageChatRemovedFromCommunity"))]
    MessageChatRemovedFromCommunity,
    /// A basic group was upgraded to a supergroup and was deactivated as the result
    #[serde(rename(serialize = "messageChatUpgradeTo", deserialize = "messageChatUpgradeTo"))]
    MessageChatUpgradeTo(Box<crate::types::MessageChatUpgradeTo>),
    /// A supergroup has been created from a basic group
    #[serde(rename(serialize = "messageChatUpgradeFrom", deserialize = "messageChatUpgradeFrom"))]
    MessageChatUpgradeFrom(Box<crate::types::MessageChatUpgradeFrom>),
    /// A message has been pinned
    #[serde(rename(serialize = "messagePinMessage", deserialize = "messagePinMessage"))]
    MessagePinMessage(Box<crate::types::MessagePinMessage>),
    /// A screenshot of a message in the chat has been taken
    #[serde(rename(serialize = "messageScreenshotTaken", deserialize = "messageScreenshotTaken"))]
    MessageScreenshotTaken,
    /// A new background was set in the chat
    #[serde(rename(serialize = "messageChatSetBackground", deserialize = "messageChatSetBackground"))]
    MessageChatSetBackground(Box<crate::types::MessageChatSetBackground>),
    /// A theme in the chat has been changed
    #[serde(rename(serialize = "messageChatSetTheme", deserialize = "messageChatSetTheme"))]
    MessageChatSetTheme(Box<crate::types::MessageChatSetTheme>),
    /// The auto-delete or self-destruct timer for messages in the chat has been changed
    #[serde(rename(serialize = "messageChatSetMessageAutoDeleteTime", deserialize = "messageChatSetMessageAutoDeleteTime"))]
    MessageChatSetMessageAutoDeleteTime(Box<crate::types::MessageChatSetMessageAutoDeleteTime>),
    /// The chat was boosted by the sender of the message
    #[serde(rename(serialize = "messageChatBoost", deserialize = "messageChatBoost"))]
    MessageChatBoost(Box<crate::types::MessageChatBoost>),
    /// A forum topic has been created
    #[serde(rename(serialize = "messageForumTopicCreated", deserialize = "messageForumTopicCreated"))]
    MessageForumTopicCreated(Box<crate::types::MessageForumTopicCreated>),
    /// A forum topic has been edited
    #[serde(rename(serialize = "messageForumTopicEdited", deserialize = "messageForumTopicEdited"))]
    MessageForumTopicEdited(Box<crate::types::MessageForumTopicEdited>),
    /// A forum topic has been closed or opened
    #[serde(rename(serialize = "messageForumTopicIsClosedToggled", deserialize = "messageForumTopicIsClosedToggled"))]
    MessageForumTopicIsClosedToggled(Box<crate::types::MessageForumTopicIsClosedToggled>),
    /// A General forum topic has been hidden or unhidden
    #[serde(rename(serialize = "messageForumTopicIsHiddenToggled", deserialize = "messageForumTopicIsHiddenToggled"))]
    MessageForumTopicIsHiddenToggled(Box<crate::types::MessageForumTopicIsHiddenToggled>),
    /// A profile photo was suggested to a user in a private chat
    #[serde(rename(serialize = "messageSuggestProfilePhoto", deserialize = "messageSuggestProfilePhoto"))]
    MessageSuggestProfilePhoto(Box<crate::types::MessageSuggestProfilePhoto>),
    /// A birthdate was suggested to be set
    #[serde(rename(serialize = "messageSuggestBirthdate", deserialize = "messageSuggestBirthdate"))]
    MessageSuggestBirthdate(Box<crate::types::MessageSuggestBirthdate>),
    /// A non-standard action has happened in the chat
    #[serde(rename(serialize = "messageCustomServiceAction", deserialize = "messageCustomServiceAction"))]
    MessageCustomServiceAction(Box<crate::types::MessageCustomServiceAction>),
    /// A new high score was achieved in a game
    #[serde(rename(serialize = "messageGameScore", deserialize = "messageGameScore"))]
    MessageGameScore(Box<crate::types::MessageGameScore>),
    /// A bot managed by another bot was created by the user
    #[serde(rename(serialize = "messageManagedBotCreated", deserialize = "messageManagedBotCreated"))]
    MessageManagedBotCreated(Box<crate::types::MessageManagedBotCreated>),
    /// A payment has been sent to a bot or a business account
    #[serde(rename(serialize = "messagePaymentSuccessful", deserialize = "messagePaymentSuccessful"))]
    MessagePaymentSuccessful(Box<crate::types::MessagePaymentSuccessful>),
    /// A payment has been received by the bot or the business account
    #[serde(rename(serialize = "messagePaymentSuccessfulBot", deserialize = "messagePaymentSuccessfulBot"))]
    MessagePaymentSuccessfulBot(Box<crate::types::MessagePaymentSuccessfulBot>),
    /// A payment has been refunded
    #[serde(rename(serialize = "messagePaymentRefunded", deserialize = "messagePaymentRefunded"))]
    MessagePaymentRefunded(Box<crate::types::MessagePaymentRefunded>),
    /// Telegram Premium was gifted to a user
    #[serde(rename(serialize = "messageGiftedPremium", deserialize = "messageGiftedPremium"))]
    MessageGiftedPremium(Box<crate::types::MessageGiftedPremium>),
    /// A Telegram Premium gift code was created for the user
    #[serde(rename(serialize = "messagePremiumGiftCode", deserialize = "messagePremiumGiftCode"))]
    MessagePremiumGiftCode(Box<crate::types::MessagePremiumGiftCode>),
    /// A giveaway was created for the chat. Use telegramPaymentPurposePremiumGiveaway, storePaymentPurposePremiumGiveaway, telegramPaymentPurposeStarGiveaway, or storePaymentPurposeStarGiveaway to create a giveaway
    #[serde(rename(serialize = "messageGiveawayCreated", deserialize = "messageGiveawayCreated"))]
    MessageGiveawayCreated(Box<crate::types::MessageGiveawayCreated>),
    /// A giveaway
    #[serde(rename(serialize = "messageGiveaway", deserialize = "messageGiveaway"))]
    MessageGiveaway(Box<crate::types::MessageGiveaway>),
    /// A giveaway without public winners has been completed for the chat
    #[serde(rename(serialize = "messageGiveawayCompleted", deserialize = "messageGiveawayCompleted"))]
    MessageGiveawayCompleted(Box<crate::types::MessageGiveawayCompleted>),
    /// A giveaway with public winners has been completed for the chat
    #[serde(rename(serialize = "messageGiveawayWinners", deserialize = "messageGiveawayWinners"))]
    MessageGiveawayWinners(Box<crate::types::MessageGiveawayWinners>),
    /// Telegram Stars were gifted to a user
    #[serde(rename(serialize = "messageGiftedStars", deserialize = "messageGiftedStars"))]
    MessageGiftedStars(Box<crate::types::MessageGiftedStars>),
    /// TON Grams were gifted to a user
    #[serde(rename(serialize = "messageGiftedGrams", deserialize = "messageGiftedGrams"))]
    MessageGiftedGrams(Box<crate::types::MessageGiftedGrams>),
    /// Telegram Stars were received by the current user from a giveaway
    #[serde(rename(serialize = "messageGiveawayPrizeStars", deserialize = "messageGiveawayPrizeStars"))]
    MessageGiveawayPrizeStars(Box<crate::types::MessageGiveawayPrizeStars>),
    /// A regular gift was received or sent by the current user, or the current user was notified about a channel gift
    #[serde(rename(serialize = "messageGift", deserialize = "messageGift"))]
    MessageGift(Box<crate::types::MessageGift>),
    /// An upgraded gift was received or sent by the current user, or the current user was notified about a channel gift
    #[serde(rename(serialize = "messageUpgradedGift", deserialize = "messageUpgradedGift"))]
    MessageUpgradedGift(Box<crate::types::MessageUpgradedGift>),
    /// A gift which purchase, upgrade or transfer were refunded
    #[serde(rename(serialize = "messageRefundedUpgradedGift", deserialize = "messageRefundedUpgradedGift"))]
    MessageRefundedUpgradedGift(Box<crate::types::MessageRefundedUpgradedGift>),
    /// An offer to purchase an upgraded gift was sent or received
    #[serde(rename(serialize = "messageUpgradedGiftPurchaseOffer", deserialize = "messageUpgradedGiftPurchaseOffer"))]
    MessageUpgradedGiftPurchaseOffer(Box<crate::types::MessageUpgradedGiftPurchaseOffer>),
    /// An offer to purchase a gift was rejected or expired
    #[serde(rename(serialize = "messageUpgradedGiftPurchaseOfferRejected", deserialize = "messageUpgradedGiftPurchaseOfferRejected"))]
    MessageUpgradedGiftPurchaseOfferRejected(Box<crate::types::MessageUpgradedGiftPurchaseOfferRejected>),
    /// Paid messages were refunded
    #[serde(rename(serialize = "messagePaidMessagesRefunded", deserialize = "messagePaidMessagesRefunded"))]
    MessagePaidMessagesRefunded(Box<crate::types::MessagePaidMessagesRefunded>),
    /// A price for paid messages was changed in the supergroup chat
    #[serde(rename(serialize = "messagePaidMessagePriceChanged", deserialize = "messagePaidMessagePriceChanged"))]
    MessagePaidMessagePriceChanged(Box<crate::types::MessagePaidMessagePriceChanged>),
    /// A price for direct messages was changed in the channel chat
    #[serde(rename(serialize = "messageDirectMessagePriceChanged", deserialize = "messageDirectMessagePriceChanged"))]
    MessageDirectMessagePriceChanged(Box<crate::types::MessageDirectMessagePriceChanged>),
    /// Some tasks from a checklist were marked as done or not done
    #[serde(rename(serialize = "messageChecklistTasksDone", deserialize = "messageChecklistTasksDone"))]
    MessageChecklistTasksDone(Box<crate::types::MessageChecklistTasksDone>),
    /// Some tasks were added to a checklist
    #[serde(rename(serialize = "messageChecklistTasksAdded", deserialize = "messageChecklistTasksAdded"))]
    MessageChecklistTasksAdded(Box<crate::types::MessageChecklistTasksAdded>),
    /// Approval of suggested post has failed, because the user who proposed the post didn't have enough funds
    #[serde(rename(serialize = "messageSuggestedPostApprovalFailed", deserialize = "messageSuggestedPostApprovalFailed"))]
    MessageSuggestedPostApprovalFailed(Box<crate::types::MessageSuggestedPostApprovalFailed>),
    /// A suggested post was approved
    #[serde(rename(serialize = "messageSuggestedPostApproved", deserialize = "messageSuggestedPostApproved"))]
    MessageSuggestedPostApproved(Box<crate::types::MessageSuggestedPostApproved>),
    /// A suggested post was declined
    #[serde(rename(serialize = "messageSuggestedPostDeclined", deserialize = "messageSuggestedPostDeclined"))]
    MessageSuggestedPostDeclined(Box<crate::types::MessageSuggestedPostDeclined>),
    /// A suggested post was published for getOption("suggested_post_lifetime_min") seconds and payment for the post was received
    #[serde(rename(serialize = "messageSuggestedPostPaid", deserialize = "messageSuggestedPostPaid"))]
    MessageSuggestedPostPaid(Box<crate::types::MessageSuggestedPostPaid>),
    /// A suggested post was refunded
    #[serde(rename(serialize = "messageSuggestedPostRefunded", deserialize = "messageSuggestedPostRefunded"))]
    MessageSuggestedPostRefunded(Box<crate::types::MessageSuggestedPostRefunded>),
    /// A contact has registered with Telegram
    #[serde(rename(serialize = "messageContactRegistered", deserialize = "messageContactRegistered"))]
    MessageContactRegistered,
    /// The current user shared users who were requested by the bot
    #[serde(rename(serialize = "messageUsersShared", deserialize = "messageUsersShared"))]
    MessageUsersShared(Box<crate::types::MessageUsersShared>),
    /// The current user shared a chat, which was requested by the bot
    #[serde(rename(serialize = "messageChatShared", deserialize = "messageChatShared"))]
    MessageChatShared(Box<crate::types::MessageChatShared>),
    /// The user allowed the bot to send messages
    #[serde(rename(serialize = "messageBotWriteAccessAllowed", deserialize = "messageBotWriteAccessAllowed"))]
    MessageBotWriteAccessAllowed(Box<crate::types::MessageBotWriteAccessAllowed>),
    /// Data from a Web App has been sent to a bot
    #[serde(rename(serialize = "messageWebAppDataSent", deserialize = "messageWebAppDataSent"))]
    MessageWebAppDataSent(Box<crate::types::MessageWebAppDataSent>),
    /// Data from a Web App has been received; for bots only
    #[serde(rename(serialize = "messageWebAppDataReceived", deserialize = "messageWebAppDataReceived"))]
    MessageWebAppDataReceived(Box<crate::types::MessageWebAppDataReceived>),
    /// Telegram Passport data has been sent to a bot
    #[serde(rename(serialize = "messagePassportDataSent", deserialize = "messagePassportDataSent"))]
    MessagePassportDataSent(Box<crate::types::MessagePassportDataSent>),
    /// Telegram Passport data has been received; for bots only
    #[serde(rename(serialize = "messagePassportDataReceived", deserialize = "messagePassportDataReceived"))]
    MessagePassportDataReceived(Box<crate::types::MessagePassportDataReceived>),
    /// A user in the chat came within proximity alert range
    #[serde(rename(serialize = "messageProximityAlertTriggered", deserialize = "messageProximityAlertTriggered"))]
    MessageProximityAlertTriggered(Box<crate::types::MessageProximityAlertTriggered>),
    /// A message content that is not supported in the current TDLib version
    #[serde(rename(serialize = "messageUnsupported", deserialize = "messageUnsupported"))]
    MessageUnsupported,
}

impl MessageContent {
    /// Convenience constructor to create a [`MessageContent::MessageText`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_text(val: crate::types::MessageText) -> Self {
        Self::MessageText(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageRichMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_rich_message(val: crate::types::MessageRichMessage) -> Self {
        Self::MessageRichMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageAnimation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_animation(val: crate::types::MessageAnimation) -> Self {
        Self::MessageAnimation(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageAudio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_audio(val: crate::types::MessageAudio) -> Self {
        Self::MessageAudio(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageDocument`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_document(val: crate::types::MessageDocument) -> Self {
        Self::MessageDocument(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePaidMedia`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_paid_media(val: crate::types::MessagePaidMedia) -> Self {
        Self::MessagePaidMedia(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePhoto`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_photo(val: crate::types::MessagePhoto) -> Self {
        Self::MessagePhoto(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_sticker(val: crate::types::MessageSticker) -> Self {
        Self::MessageSticker(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageVideo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_video(val: crate::types::MessageVideo) -> Self {
        Self::MessageVideo(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageVideoNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_video_note(val: crate::types::MessageVideoNote) -> Self {
        Self::MessageVideoNote(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageVoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_voice_note(val: crate::types::MessageVoiceNote) -> Self {
        Self::MessageVoiceNote(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageLiveLocation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_live_location(val: crate::types::MessageLiveLocation) -> Self {
        Self::MessageLiveLocation(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageLocation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_location(val: crate::types::MessageLocation) -> Self {
        Self::MessageLocation(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageVenue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_venue(val: crate::types::MessageVenue) -> Self {
        Self::MessageVenue(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageContact`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_contact(val: crate::types::MessageContact) -> Self {
        Self::MessageContact(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageAnimatedEmoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_animated_emoji(val: crate::types::MessageAnimatedEmoji) -> Self {
        Self::MessageAnimatedEmoji(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageDice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_dice(val: crate::types::MessageDice) -> Self {
        Self::MessageDice(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGame`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_game(val: crate::types::MessageGame) -> Self {
        Self::MessageGame(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePoll`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_poll(val: crate::types::MessagePoll) -> Self {
        Self::MessagePoll(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageStakeDice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_stake_dice(val: crate::types::MessageStakeDice) -> Self {
        Self::MessageStakeDice(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageStory`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_story(val: crate::types::MessageStory) -> Self {
        Self::MessageStory(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChecklist`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_checklist(val: crate::types::MessageChecklist) -> Self {
        Self::MessageChecklist(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageInvoice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_invoice(val: crate::types::MessageInvoice) -> Self {
        Self::MessageInvoice(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_call(val: crate::types::MessageCall) -> Self {
        Self::MessageCall(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGroupCall`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_group_call(val: crate::types::MessageGroupCall) -> Self {
        Self::MessageGroupCall(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageVideoChatScheduled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_video_chat_scheduled(val: crate::types::MessageVideoChatScheduled) -> Self {
        Self::MessageVideoChatScheduled(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageVideoChatStarted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_video_chat_started(val: crate::types::MessageVideoChatStarted) -> Self {
        Self::MessageVideoChatStarted(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageVideoChatEnded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_video_chat_ended(val: crate::types::MessageVideoChatEnded) -> Self {
        Self::MessageVideoChatEnded(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageInviteVideoChatParticipants`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_invite_video_chat_participants(val: crate::types::MessageInviteVideoChatParticipants) -> Self {
        Self::MessageInviteVideoChatParticipants(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePollOptionAdded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_poll_option_added(val: crate::types::MessagePollOptionAdded) -> Self {
        Self::MessagePollOptionAdded(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePollOptionDeleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_poll_option_deleted(val: crate::types::MessagePollOptionDeleted) -> Self {
        Self::MessagePollOptionDeleted(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageBasicGroupChatCreate`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_basic_group_chat_create(val: crate::types::MessageBasicGroupChatCreate) -> Self {
        Self::MessageBasicGroupChatCreate(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSupergroupChatCreate`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_supergroup_chat_create(val: crate::types::MessageSupergroupChatCreate) -> Self {
        Self::MessageSupergroupChatCreate(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatChangeTitle`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_change_title(val: crate::types::MessageChatChangeTitle) -> Self {
        Self::MessageChatChangeTitle(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatChangePhoto`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_change_photo(val: crate::types::MessageChatChangePhoto) -> Self {
        Self::MessageChatChangePhoto(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatOwnerLeft`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_owner_left(val: crate::types::MessageChatOwnerLeft) -> Self {
        Self::MessageChatOwnerLeft(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatOwnerChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_owner_changed(val: crate::types::MessageChatOwnerChanged) -> Self {
        Self::MessageChatOwnerChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatHasProtectedContentToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_has_protected_content_toggled(val: crate::types::MessageChatHasProtectedContentToggled) -> Self {
        Self::MessageChatHasProtectedContentToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatHasProtectedContentDisableRequested`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_has_protected_content_disable_requested(val: crate::types::MessageChatHasProtectedContentDisableRequested) -> Self {
        Self::MessageChatHasProtectedContentDisableRequested(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatAddMembers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_add_members(val: crate::types::MessageChatAddMembers) -> Self {
        Self::MessageChatAddMembers(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatJoinFromCommunity`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_join_from_community(val: crate::types::MessageChatJoinFromCommunity) -> Self {
        Self::MessageChatJoinFromCommunity(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatDeleteMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_delete_member(val: crate::types::MessageChatDeleteMember) -> Self {
        Self::MessageChatDeleteMember(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatAddedToCommunity`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_added_to_community(val: crate::types::MessageChatAddedToCommunity) -> Self {
        Self::MessageChatAddedToCommunity(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatUpgradeTo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_upgrade_to(val: crate::types::MessageChatUpgradeTo) -> Self {
        Self::MessageChatUpgradeTo(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatUpgradeFrom`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_upgrade_from(val: crate::types::MessageChatUpgradeFrom) -> Self {
        Self::MessageChatUpgradeFrom(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePinMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_pin_message(val: crate::types::MessagePinMessage) -> Self {
        Self::MessagePinMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatSetBackground`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_set_background(val: crate::types::MessageChatSetBackground) -> Self {
        Self::MessageChatSetBackground(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatSetTheme`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_set_theme(val: crate::types::MessageChatSetTheme) -> Self {
        Self::MessageChatSetTheme(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatSetMessageAutoDeleteTime`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_set_message_auto_delete_time(val: crate::types::MessageChatSetMessageAutoDeleteTime) -> Self {
        Self::MessageChatSetMessageAutoDeleteTime(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatBoost`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_boost(val: crate::types::MessageChatBoost) -> Self {
        Self::MessageChatBoost(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageForumTopicCreated`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_forum_topic_created(val: crate::types::MessageForumTopicCreated) -> Self {
        Self::MessageForumTopicCreated(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageForumTopicEdited`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_forum_topic_edited(val: crate::types::MessageForumTopicEdited) -> Self {
        Self::MessageForumTopicEdited(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageForumTopicIsClosedToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_forum_topic_is_closed_toggled(val: crate::types::MessageForumTopicIsClosedToggled) -> Self {
        Self::MessageForumTopicIsClosedToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageForumTopicIsHiddenToggled`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_forum_topic_is_hidden_toggled(val: crate::types::MessageForumTopicIsHiddenToggled) -> Self {
        Self::MessageForumTopicIsHiddenToggled(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSuggestProfilePhoto`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_suggest_profile_photo(val: crate::types::MessageSuggestProfilePhoto) -> Self {
        Self::MessageSuggestProfilePhoto(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSuggestBirthdate`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_suggest_birthdate(val: crate::types::MessageSuggestBirthdate) -> Self {
        Self::MessageSuggestBirthdate(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageCustomServiceAction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_custom_service_action(val: crate::types::MessageCustomServiceAction) -> Self {
        Self::MessageCustomServiceAction(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGameScore`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_game_score(val: crate::types::MessageGameScore) -> Self {
        Self::MessageGameScore(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageManagedBotCreated`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_managed_bot_created(val: crate::types::MessageManagedBotCreated) -> Self {
        Self::MessageManagedBotCreated(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePaymentSuccessful`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_payment_successful(val: crate::types::MessagePaymentSuccessful) -> Self {
        Self::MessagePaymentSuccessful(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePaymentSuccessfulBot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_payment_successful_bot(val: crate::types::MessagePaymentSuccessfulBot) -> Self {
        Self::MessagePaymentSuccessfulBot(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePaymentRefunded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_payment_refunded(val: crate::types::MessagePaymentRefunded) -> Self {
        Self::MessagePaymentRefunded(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGiftedPremium`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_gifted_premium(val: crate::types::MessageGiftedPremium) -> Self {
        Self::MessageGiftedPremium(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePremiumGiftCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_premium_gift_code(val: crate::types::MessagePremiumGiftCode) -> Self {
        Self::MessagePremiumGiftCode(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGiveawayCreated`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_giveaway_created(val: crate::types::MessageGiveawayCreated) -> Self {
        Self::MessageGiveawayCreated(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGiveaway`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_giveaway(val: crate::types::MessageGiveaway) -> Self {
        Self::MessageGiveaway(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGiveawayCompleted`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_giveaway_completed(val: crate::types::MessageGiveawayCompleted) -> Self {
        Self::MessageGiveawayCompleted(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGiveawayWinners`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_giveaway_winners(val: crate::types::MessageGiveawayWinners) -> Self {
        Self::MessageGiveawayWinners(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGiftedStars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_gifted_stars(val: crate::types::MessageGiftedStars) -> Self {
        Self::MessageGiftedStars(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGiftedGrams`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_gifted_grams(val: crate::types::MessageGiftedGrams) -> Self {
        Self::MessageGiftedGrams(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGiveawayPrizeStars`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_giveaway_prize_stars(val: crate::types::MessageGiveawayPrizeStars) -> Self {
        Self::MessageGiveawayPrizeStars(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_gift(val: crate::types::MessageGift) -> Self {
        Self::MessageGift(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageUpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_upgraded_gift(val: crate::types::MessageUpgradedGift) -> Self {
        Self::MessageUpgradedGift(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageRefundedUpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_refunded_upgraded_gift(val: crate::types::MessageRefundedUpgradedGift) -> Self {
        Self::MessageRefundedUpgradedGift(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageUpgradedGiftPurchaseOffer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_upgraded_gift_purchase_offer(val: crate::types::MessageUpgradedGiftPurchaseOffer) -> Self {
        Self::MessageUpgradedGiftPurchaseOffer(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageUpgradedGiftPurchaseOfferRejected`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_upgraded_gift_purchase_offer_rejected(val: crate::types::MessageUpgradedGiftPurchaseOfferRejected) -> Self {
        Self::MessageUpgradedGiftPurchaseOfferRejected(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePaidMessagesRefunded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_paid_messages_refunded(val: crate::types::MessagePaidMessagesRefunded) -> Self {
        Self::MessagePaidMessagesRefunded(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePaidMessagePriceChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_paid_message_price_changed(val: crate::types::MessagePaidMessagePriceChanged) -> Self {
        Self::MessagePaidMessagePriceChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageDirectMessagePriceChanged`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_direct_message_price_changed(val: crate::types::MessageDirectMessagePriceChanged) -> Self {
        Self::MessageDirectMessagePriceChanged(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChecklistTasksDone`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_checklist_tasks_done(val: crate::types::MessageChecklistTasksDone) -> Self {
        Self::MessageChecklistTasksDone(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChecklistTasksAdded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_checklist_tasks_added(val: crate::types::MessageChecklistTasksAdded) -> Self {
        Self::MessageChecklistTasksAdded(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSuggestedPostApprovalFailed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_suggested_post_approval_failed(val: crate::types::MessageSuggestedPostApprovalFailed) -> Self {
        Self::MessageSuggestedPostApprovalFailed(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSuggestedPostApproved`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_suggested_post_approved(val: crate::types::MessageSuggestedPostApproved) -> Self {
        Self::MessageSuggestedPostApproved(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSuggestedPostDeclined`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_suggested_post_declined(val: crate::types::MessageSuggestedPostDeclined) -> Self {
        Self::MessageSuggestedPostDeclined(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSuggestedPostPaid`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_suggested_post_paid(val: crate::types::MessageSuggestedPostPaid) -> Self {
        Self::MessageSuggestedPostPaid(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageSuggestedPostRefunded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_suggested_post_refunded(val: crate::types::MessageSuggestedPostRefunded) -> Self {
        Self::MessageSuggestedPostRefunded(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageUsersShared`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_users_shared(val: crate::types::MessageUsersShared) -> Self {
        Self::MessageUsersShared(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageChatShared`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_chat_shared(val: crate::types::MessageChatShared) -> Self {
        Self::MessageChatShared(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageBotWriteAccessAllowed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_bot_write_access_allowed(val: crate::types::MessageBotWriteAccessAllowed) -> Self {
        Self::MessageBotWriteAccessAllowed(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageWebAppDataSent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_web_app_data_sent(val: crate::types::MessageWebAppDataSent) -> Self {
        Self::MessageWebAppDataSent(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageWebAppDataReceived`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_web_app_data_received(val: crate::types::MessageWebAppDataReceived) -> Self {
        Self::MessageWebAppDataReceived(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePassportDataSent`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_passport_data_sent(val: crate::types::MessagePassportDataSent) -> Self {
        Self::MessagePassportDataSent(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessagePassportDataReceived`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_passport_data_received(val: crate::types::MessagePassportDataReceived) -> Self {
        Self::MessagePassportDataReceived(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageContent::MessageProximityAlertTriggered`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_proximity_alert_triggered(val: crate::types::MessageProximityAlertTriggered) -> Self {
        Self::MessageProximityAlertTriggered(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageText`] into [`MessageContent`].
impl From<crate::types::MessageText> for MessageContent {
    fn from(val: crate::types::MessageText) -> Self {
        Self::MessageText(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageRichMessage`] into [`MessageContent`].
impl From<crate::types::MessageRichMessage> for MessageContent {
    fn from(val: crate::types::MessageRichMessage) -> Self {
        Self::MessageRichMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageAnimation`] into [`MessageContent`].
impl From<crate::types::MessageAnimation> for MessageContent {
    fn from(val: crate::types::MessageAnimation) -> Self {
        Self::MessageAnimation(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageAudio`] into [`MessageContent`].
impl From<crate::types::MessageAudio> for MessageContent {
    fn from(val: crate::types::MessageAudio) -> Self {
        Self::MessageAudio(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageDocument`] into [`MessageContent`].
impl From<crate::types::MessageDocument> for MessageContent {
    fn from(val: crate::types::MessageDocument) -> Self {
        Self::MessageDocument(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePaidMedia`] into [`MessageContent`].
impl From<crate::types::MessagePaidMedia> for MessageContent {
    fn from(val: crate::types::MessagePaidMedia) -> Self {
        Self::MessagePaidMedia(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePhoto`] into [`MessageContent`].
impl From<crate::types::MessagePhoto> for MessageContent {
    fn from(val: crate::types::MessagePhoto) -> Self {
        Self::MessagePhoto(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSticker`] into [`MessageContent`].
impl From<crate::types::MessageSticker> for MessageContent {
    fn from(val: crate::types::MessageSticker) -> Self {
        Self::MessageSticker(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageVideo`] into [`MessageContent`].
impl From<crate::types::MessageVideo> for MessageContent {
    fn from(val: crate::types::MessageVideo) -> Self {
        Self::MessageVideo(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageVideoNote`] into [`MessageContent`].
impl From<crate::types::MessageVideoNote> for MessageContent {
    fn from(val: crate::types::MessageVideoNote) -> Self {
        Self::MessageVideoNote(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageVoiceNote`] into [`MessageContent`].
impl From<crate::types::MessageVoiceNote> for MessageContent {
    fn from(val: crate::types::MessageVoiceNote) -> Self {
        Self::MessageVoiceNote(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageLiveLocation`] into [`MessageContent`].
impl From<crate::types::MessageLiveLocation> for MessageContent {
    fn from(val: crate::types::MessageLiveLocation) -> Self {
        Self::MessageLiveLocation(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageLocation`] into [`MessageContent`].
impl From<crate::types::MessageLocation> for MessageContent {
    fn from(val: crate::types::MessageLocation) -> Self {
        Self::MessageLocation(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageVenue`] into [`MessageContent`].
impl From<crate::types::MessageVenue> for MessageContent {
    fn from(val: crate::types::MessageVenue) -> Self {
        Self::MessageVenue(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageContact`] into [`MessageContent`].
impl From<crate::types::MessageContact> for MessageContent {
    fn from(val: crate::types::MessageContact) -> Self {
        Self::MessageContact(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageAnimatedEmoji`] into [`MessageContent`].
impl From<crate::types::MessageAnimatedEmoji> for MessageContent {
    fn from(val: crate::types::MessageAnimatedEmoji) -> Self {
        Self::MessageAnimatedEmoji(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageDice`] into [`MessageContent`].
impl From<crate::types::MessageDice> for MessageContent {
    fn from(val: crate::types::MessageDice) -> Self {
        Self::MessageDice(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGame`] into [`MessageContent`].
impl From<crate::types::MessageGame> for MessageContent {
    fn from(val: crate::types::MessageGame) -> Self {
        Self::MessageGame(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePoll`] into [`MessageContent`].
impl From<crate::types::MessagePoll> for MessageContent {
    fn from(val: crate::types::MessagePoll) -> Self {
        Self::MessagePoll(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageStakeDice`] into [`MessageContent`].
impl From<crate::types::MessageStakeDice> for MessageContent {
    fn from(val: crate::types::MessageStakeDice) -> Self {
        Self::MessageStakeDice(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageStory`] into [`MessageContent`].
impl From<crate::types::MessageStory> for MessageContent {
    fn from(val: crate::types::MessageStory) -> Self {
        Self::MessageStory(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChecklist`] into [`MessageContent`].
impl From<crate::types::MessageChecklist> for MessageContent {
    fn from(val: crate::types::MessageChecklist) -> Self {
        Self::MessageChecklist(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageInvoice`] into [`MessageContent`].
impl From<crate::types::MessageInvoice> for MessageContent {
    fn from(val: crate::types::MessageInvoice) -> Self {
        Self::MessageInvoice(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageCall`] into [`MessageContent`].
impl From<crate::types::MessageCall> for MessageContent {
    fn from(val: crate::types::MessageCall) -> Self {
        Self::MessageCall(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGroupCall`] into [`MessageContent`].
impl From<crate::types::MessageGroupCall> for MessageContent {
    fn from(val: crate::types::MessageGroupCall) -> Self {
        Self::MessageGroupCall(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageVideoChatScheduled`] into [`MessageContent`].
impl From<crate::types::MessageVideoChatScheduled> for MessageContent {
    fn from(val: crate::types::MessageVideoChatScheduled) -> Self {
        Self::MessageVideoChatScheduled(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageVideoChatStarted`] into [`MessageContent`].
impl From<crate::types::MessageVideoChatStarted> for MessageContent {
    fn from(val: crate::types::MessageVideoChatStarted) -> Self {
        Self::MessageVideoChatStarted(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageVideoChatEnded`] into [`MessageContent`].
impl From<crate::types::MessageVideoChatEnded> for MessageContent {
    fn from(val: crate::types::MessageVideoChatEnded) -> Self {
        Self::MessageVideoChatEnded(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageInviteVideoChatParticipants`] into [`MessageContent`].
impl From<crate::types::MessageInviteVideoChatParticipants> for MessageContent {
    fn from(val: crate::types::MessageInviteVideoChatParticipants) -> Self {
        Self::MessageInviteVideoChatParticipants(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePollOptionAdded`] into [`MessageContent`].
impl From<crate::types::MessagePollOptionAdded> for MessageContent {
    fn from(val: crate::types::MessagePollOptionAdded) -> Self {
        Self::MessagePollOptionAdded(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePollOptionDeleted`] into [`MessageContent`].
impl From<crate::types::MessagePollOptionDeleted> for MessageContent {
    fn from(val: crate::types::MessagePollOptionDeleted) -> Self {
        Self::MessagePollOptionDeleted(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageBasicGroupChatCreate`] into [`MessageContent`].
impl From<crate::types::MessageBasicGroupChatCreate> for MessageContent {
    fn from(val: crate::types::MessageBasicGroupChatCreate) -> Self {
        Self::MessageBasicGroupChatCreate(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSupergroupChatCreate`] into [`MessageContent`].
impl From<crate::types::MessageSupergroupChatCreate> for MessageContent {
    fn from(val: crate::types::MessageSupergroupChatCreate) -> Self {
        Self::MessageSupergroupChatCreate(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatChangeTitle`] into [`MessageContent`].
impl From<crate::types::MessageChatChangeTitle> for MessageContent {
    fn from(val: crate::types::MessageChatChangeTitle) -> Self {
        Self::MessageChatChangeTitle(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatChangePhoto`] into [`MessageContent`].
impl From<crate::types::MessageChatChangePhoto> for MessageContent {
    fn from(val: crate::types::MessageChatChangePhoto) -> Self {
        Self::MessageChatChangePhoto(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatOwnerLeft`] into [`MessageContent`].
impl From<crate::types::MessageChatOwnerLeft> for MessageContent {
    fn from(val: crate::types::MessageChatOwnerLeft) -> Self {
        Self::MessageChatOwnerLeft(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatOwnerChanged`] into [`MessageContent`].
impl From<crate::types::MessageChatOwnerChanged> for MessageContent {
    fn from(val: crate::types::MessageChatOwnerChanged) -> Self {
        Self::MessageChatOwnerChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatHasProtectedContentToggled`] into [`MessageContent`].
impl From<crate::types::MessageChatHasProtectedContentToggled> for MessageContent {
    fn from(val: crate::types::MessageChatHasProtectedContentToggled) -> Self {
        Self::MessageChatHasProtectedContentToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatHasProtectedContentDisableRequested`] into [`MessageContent`].
impl From<crate::types::MessageChatHasProtectedContentDisableRequested> for MessageContent {
    fn from(val: crate::types::MessageChatHasProtectedContentDisableRequested) -> Self {
        Self::MessageChatHasProtectedContentDisableRequested(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatAddMembers`] into [`MessageContent`].
impl From<crate::types::MessageChatAddMembers> for MessageContent {
    fn from(val: crate::types::MessageChatAddMembers) -> Self {
        Self::MessageChatAddMembers(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatJoinFromCommunity`] into [`MessageContent`].
impl From<crate::types::MessageChatJoinFromCommunity> for MessageContent {
    fn from(val: crate::types::MessageChatJoinFromCommunity) -> Self {
        Self::MessageChatJoinFromCommunity(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatDeleteMember`] into [`MessageContent`].
impl From<crate::types::MessageChatDeleteMember> for MessageContent {
    fn from(val: crate::types::MessageChatDeleteMember) -> Self {
        Self::MessageChatDeleteMember(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatAddedToCommunity`] into [`MessageContent`].
impl From<crate::types::MessageChatAddedToCommunity> for MessageContent {
    fn from(val: crate::types::MessageChatAddedToCommunity) -> Self {
        Self::MessageChatAddedToCommunity(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatUpgradeTo`] into [`MessageContent`].
impl From<crate::types::MessageChatUpgradeTo> for MessageContent {
    fn from(val: crate::types::MessageChatUpgradeTo) -> Self {
        Self::MessageChatUpgradeTo(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatUpgradeFrom`] into [`MessageContent`].
impl From<crate::types::MessageChatUpgradeFrom> for MessageContent {
    fn from(val: crate::types::MessageChatUpgradeFrom) -> Self {
        Self::MessageChatUpgradeFrom(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePinMessage`] into [`MessageContent`].
impl From<crate::types::MessagePinMessage> for MessageContent {
    fn from(val: crate::types::MessagePinMessage) -> Self {
        Self::MessagePinMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatSetBackground`] into [`MessageContent`].
impl From<crate::types::MessageChatSetBackground> for MessageContent {
    fn from(val: crate::types::MessageChatSetBackground) -> Self {
        Self::MessageChatSetBackground(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatSetTheme`] into [`MessageContent`].
impl From<crate::types::MessageChatSetTheme> for MessageContent {
    fn from(val: crate::types::MessageChatSetTheme) -> Self {
        Self::MessageChatSetTheme(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatSetMessageAutoDeleteTime`] into [`MessageContent`].
impl From<crate::types::MessageChatSetMessageAutoDeleteTime> for MessageContent {
    fn from(val: crate::types::MessageChatSetMessageAutoDeleteTime) -> Self {
        Self::MessageChatSetMessageAutoDeleteTime(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatBoost`] into [`MessageContent`].
impl From<crate::types::MessageChatBoost> for MessageContent {
    fn from(val: crate::types::MessageChatBoost) -> Self {
        Self::MessageChatBoost(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageForumTopicCreated`] into [`MessageContent`].
impl From<crate::types::MessageForumTopicCreated> for MessageContent {
    fn from(val: crate::types::MessageForumTopicCreated) -> Self {
        Self::MessageForumTopicCreated(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageForumTopicEdited`] into [`MessageContent`].
impl From<crate::types::MessageForumTopicEdited> for MessageContent {
    fn from(val: crate::types::MessageForumTopicEdited) -> Self {
        Self::MessageForumTopicEdited(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageForumTopicIsClosedToggled`] into [`MessageContent`].
impl From<crate::types::MessageForumTopicIsClosedToggled> for MessageContent {
    fn from(val: crate::types::MessageForumTopicIsClosedToggled) -> Self {
        Self::MessageForumTopicIsClosedToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageForumTopicIsHiddenToggled`] into [`MessageContent`].
impl From<crate::types::MessageForumTopicIsHiddenToggled> for MessageContent {
    fn from(val: crate::types::MessageForumTopicIsHiddenToggled) -> Self {
        Self::MessageForumTopicIsHiddenToggled(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSuggestProfilePhoto`] into [`MessageContent`].
impl From<crate::types::MessageSuggestProfilePhoto> for MessageContent {
    fn from(val: crate::types::MessageSuggestProfilePhoto) -> Self {
        Self::MessageSuggestProfilePhoto(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSuggestBirthdate`] into [`MessageContent`].
impl From<crate::types::MessageSuggestBirthdate> for MessageContent {
    fn from(val: crate::types::MessageSuggestBirthdate) -> Self {
        Self::MessageSuggestBirthdate(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageCustomServiceAction`] into [`MessageContent`].
impl From<crate::types::MessageCustomServiceAction> for MessageContent {
    fn from(val: crate::types::MessageCustomServiceAction) -> Self {
        Self::MessageCustomServiceAction(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGameScore`] into [`MessageContent`].
impl From<crate::types::MessageGameScore> for MessageContent {
    fn from(val: crate::types::MessageGameScore) -> Self {
        Self::MessageGameScore(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageManagedBotCreated`] into [`MessageContent`].
impl From<crate::types::MessageManagedBotCreated> for MessageContent {
    fn from(val: crate::types::MessageManagedBotCreated) -> Self {
        Self::MessageManagedBotCreated(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePaymentSuccessful`] into [`MessageContent`].
impl From<crate::types::MessagePaymentSuccessful> for MessageContent {
    fn from(val: crate::types::MessagePaymentSuccessful) -> Self {
        Self::MessagePaymentSuccessful(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePaymentSuccessfulBot`] into [`MessageContent`].
impl From<crate::types::MessagePaymentSuccessfulBot> for MessageContent {
    fn from(val: crate::types::MessagePaymentSuccessfulBot) -> Self {
        Self::MessagePaymentSuccessfulBot(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePaymentRefunded`] into [`MessageContent`].
impl From<crate::types::MessagePaymentRefunded> for MessageContent {
    fn from(val: crate::types::MessagePaymentRefunded) -> Self {
        Self::MessagePaymentRefunded(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGiftedPremium`] into [`MessageContent`].
impl From<crate::types::MessageGiftedPremium> for MessageContent {
    fn from(val: crate::types::MessageGiftedPremium) -> Self {
        Self::MessageGiftedPremium(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePremiumGiftCode`] into [`MessageContent`].
impl From<crate::types::MessagePremiumGiftCode> for MessageContent {
    fn from(val: crate::types::MessagePremiumGiftCode) -> Self {
        Self::MessagePremiumGiftCode(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGiveawayCreated`] into [`MessageContent`].
impl From<crate::types::MessageGiveawayCreated> for MessageContent {
    fn from(val: crate::types::MessageGiveawayCreated) -> Self {
        Self::MessageGiveawayCreated(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGiveaway`] into [`MessageContent`].
impl From<crate::types::MessageGiveaway> for MessageContent {
    fn from(val: crate::types::MessageGiveaway) -> Self {
        Self::MessageGiveaway(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGiveawayCompleted`] into [`MessageContent`].
impl From<crate::types::MessageGiveawayCompleted> for MessageContent {
    fn from(val: crate::types::MessageGiveawayCompleted) -> Self {
        Self::MessageGiveawayCompleted(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGiveawayWinners`] into [`MessageContent`].
impl From<crate::types::MessageGiveawayWinners> for MessageContent {
    fn from(val: crate::types::MessageGiveawayWinners) -> Self {
        Self::MessageGiveawayWinners(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGiftedStars`] into [`MessageContent`].
impl From<crate::types::MessageGiftedStars> for MessageContent {
    fn from(val: crate::types::MessageGiftedStars) -> Self {
        Self::MessageGiftedStars(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGiftedGrams`] into [`MessageContent`].
impl From<crate::types::MessageGiftedGrams> for MessageContent {
    fn from(val: crate::types::MessageGiftedGrams) -> Self {
        Self::MessageGiftedGrams(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGiveawayPrizeStars`] into [`MessageContent`].
impl From<crate::types::MessageGiveawayPrizeStars> for MessageContent {
    fn from(val: crate::types::MessageGiveawayPrizeStars) -> Self {
        Self::MessageGiveawayPrizeStars(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageGift`] into [`MessageContent`].
impl From<crate::types::MessageGift> for MessageContent {
    fn from(val: crate::types::MessageGift) -> Self {
        Self::MessageGift(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageUpgradedGift`] into [`MessageContent`].
impl From<crate::types::MessageUpgradedGift> for MessageContent {
    fn from(val: crate::types::MessageUpgradedGift) -> Self {
        Self::MessageUpgradedGift(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageRefundedUpgradedGift`] into [`MessageContent`].
impl From<crate::types::MessageRefundedUpgradedGift> for MessageContent {
    fn from(val: crate::types::MessageRefundedUpgradedGift) -> Self {
        Self::MessageRefundedUpgradedGift(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageUpgradedGiftPurchaseOffer`] into [`MessageContent`].
impl From<crate::types::MessageUpgradedGiftPurchaseOffer> for MessageContent {
    fn from(val: crate::types::MessageUpgradedGiftPurchaseOffer) -> Self {
        Self::MessageUpgradedGiftPurchaseOffer(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageUpgradedGiftPurchaseOfferRejected`] into [`MessageContent`].
impl From<crate::types::MessageUpgradedGiftPurchaseOfferRejected> for MessageContent {
    fn from(val: crate::types::MessageUpgradedGiftPurchaseOfferRejected) -> Self {
        Self::MessageUpgradedGiftPurchaseOfferRejected(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePaidMessagesRefunded`] into [`MessageContent`].
impl From<crate::types::MessagePaidMessagesRefunded> for MessageContent {
    fn from(val: crate::types::MessagePaidMessagesRefunded) -> Self {
        Self::MessagePaidMessagesRefunded(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePaidMessagePriceChanged`] into [`MessageContent`].
impl From<crate::types::MessagePaidMessagePriceChanged> for MessageContent {
    fn from(val: crate::types::MessagePaidMessagePriceChanged) -> Self {
        Self::MessagePaidMessagePriceChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageDirectMessagePriceChanged`] into [`MessageContent`].
impl From<crate::types::MessageDirectMessagePriceChanged> for MessageContent {
    fn from(val: crate::types::MessageDirectMessagePriceChanged) -> Self {
        Self::MessageDirectMessagePriceChanged(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChecklistTasksDone`] into [`MessageContent`].
impl From<crate::types::MessageChecklistTasksDone> for MessageContent {
    fn from(val: crate::types::MessageChecklistTasksDone) -> Self {
        Self::MessageChecklistTasksDone(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChecklistTasksAdded`] into [`MessageContent`].
impl From<crate::types::MessageChecklistTasksAdded> for MessageContent {
    fn from(val: crate::types::MessageChecklistTasksAdded) -> Self {
        Self::MessageChecklistTasksAdded(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSuggestedPostApprovalFailed`] into [`MessageContent`].
impl From<crate::types::MessageSuggestedPostApprovalFailed> for MessageContent {
    fn from(val: crate::types::MessageSuggestedPostApprovalFailed) -> Self {
        Self::MessageSuggestedPostApprovalFailed(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSuggestedPostApproved`] into [`MessageContent`].
impl From<crate::types::MessageSuggestedPostApproved> for MessageContent {
    fn from(val: crate::types::MessageSuggestedPostApproved) -> Self {
        Self::MessageSuggestedPostApproved(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSuggestedPostDeclined`] into [`MessageContent`].
impl From<crate::types::MessageSuggestedPostDeclined> for MessageContent {
    fn from(val: crate::types::MessageSuggestedPostDeclined) -> Self {
        Self::MessageSuggestedPostDeclined(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSuggestedPostPaid`] into [`MessageContent`].
impl From<crate::types::MessageSuggestedPostPaid> for MessageContent {
    fn from(val: crate::types::MessageSuggestedPostPaid) -> Self {
        Self::MessageSuggestedPostPaid(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSuggestedPostRefunded`] into [`MessageContent`].
impl From<crate::types::MessageSuggestedPostRefunded> for MessageContent {
    fn from(val: crate::types::MessageSuggestedPostRefunded) -> Self {
        Self::MessageSuggestedPostRefunded(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageUsersShared`] into [`MessageContent`].
impl From<crate::types::MessageUsersShared> for MessageContent {
    fn from(val: crate::types::MessageUsersShared) -> Self {
        Self::MessageUsersShared(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageChatShared`] into [`MessageContent`].
impl From<crate::types::MessageChatShared> for MessageContent {
    fn from(val: crate::types::MessageChatShared) -> Self {
        Self::MessageChatShared(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageBotWriteAccessAllowed`] into [`MessageContent`].
impl From<crate::types::MessageBotWriteAccessAllowed> for MessageContent {
    fn from(val: crate::types::MessageBotWriteAccessAllowed) -> Self {
        Self::MessageBotWriteAccessAllowed(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageWebAppDataSent`] into [`MessageContent`].
impl From<crate::types::MessageWebAppDataSent> for MessageContent {
    fn from(val: crate::types::MessageWebAppDataSent) -> Self {
        Self::MessageWebAppDataSent(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageWebAppDataReceived`] into [`MessageContent`].
impl From<crate::types::MessageWebAppDataReceived> for MessageContent {
    fn from(val: crate::types::MessageWebAppDataReceived) -> Self {
        Self::MessageWebAppDataReceived(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePassportDataSent`] into [`MessageContent`].
impl From<crate::types::MessagePassportDataSent> for MessageContent {
    fn from(val: crate::types::MessagePassportDataSent) -> Self {
        Self::MessagePassportDataSent(Box::new(val))
    }
}

/// Converts a [`crate::types::MessagePassportDataReceived`] into [`MessageContent`].
impl From<crate::types::MessagePassportDataReceived> for MessageContent {
    fn from(val: crate::types::MessagePassportDataReceived) -> Self {
        Self::MessagePassportDataReceived(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageProximityAlertTriggered`] into [`MessageContent`].
impl From<crate::types::MessageProximityAlertTriggered> for MessageContent {
    fn from(val: crate::types::MessageProximityAlertTriggered) -> Self {
        Self::MessageProximityAlertTriggered(Box::new(val))
    }
}

/// Represents a part of the text which must be formatted differently
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextEntityType {
    /// A mention of a user, a supergroup, or a channel by their username
    #[serde(rename(serialize = "textEntityTypeMention", deserialize = "textEntityTypeMention"))]
    Mention,
    /// A hashtag text, beginning with "#" and optionally containing a chat username at the end
    #[serde(rename(serialize = "textEntityTypeHashtag", deserialize = "textEntityTypeHashtag"))]
    Hashtag,
    /// A cashtag text, beginning with "$", consisting of capital English letters (e.g., "$USD"), and optionally containing a chat username at the end
    #[serde(rename(serialize = "textEntityTypeCashtag", deserialize = "textEntityTypeCashtag"))]
    Cashtag,
    /// A bot command, beginning with "/"
    #[serde(rename(serialize = "textEntityTypeBotCommand", deserialize = "textEntityTypeBotCommand"))]
    BotCommand,
    /// An HTTP URL
    #[serde(rename(serialize = "textEntityTypeUrl", deserialize = "textEntityTypeUrl"))]
    Url,
    /// An email address
    #[serde(rename(serialize = "textEntityTypeEmailAddress", deserialize = "textEntityTypeEmailAddress"))]
    EmailAddress,
    /// A phone number
    #[serde(rename(serialize = "textEntityTypePhoneNumber", deserialize = "textEntityTypePhoneNumber"))]
    PhoneNumber,
    /// A bank card number. The getBankCardInfo method can be used to get information about the bank card
    #[serde(rename(serialize = "textEntityTypeBankCardNumber", deserialize = "textEntityTypeBankCardNumber"))]
    BankCardNumber,
    /// A bold text
    #[serde(rename(serialize = "textEntityTypeBold", deserialize = "textEntityTypeBold"))]
    Bold,
    /// An italic text
    #[serde(rename(serialize = "textEntityTypeItalic", deserialize = "textEntityTypeItalic"))]
    Italic,
    /// An underlined text
    #[serde(rename(serialize = "textEntityTypeUnderline", deserialize = "textEntityTypeUnderline"))]
    Underline,
    /// A strikethrough text
    #[serde(rename(serialize = "textEntityTypeStrikethrough", deserialize = "textEntityTypeStrikethrough"))]
    Strikethrough,
    /// A spoiler text
    #[serde(rename(serialize = "textEntityTypeSpoiler", deserialize = "textEntityTypeSpoiler"))]
    Spoiler,
    /// Text that must be formatted as if inside a code HTML tag
    #[serde(rename(serialize = "textEntityTypeCode", deserialize = "textEntityTypeCode"))]
    Code,
    /// Text that must be formatted as if inside a pre HTML tag
    #[serde(rename(serialize = "textEntityTypePre", deserialize = "textEntityTypePre"))]
    Pre,
    /// Text that must be formatted as if inside pre, and code HTML tags
    #[serde(rename(serialize = "textEntityTypePreCode", deserialize = "textEntityTypePreCode"))]
    PreCode(Box<crate::types::TextEntityTypePreCode>),
    /// Text that must be formatted as if inside a blockquote HTML tag; not supported in secret chats
    #[serde(rename(serialize = "textEntityTypeBlockQuote", deserialize = "textEntityTypeBlockQuote"))]
    BlockQuote,
    /// Text that must be formatted as if inside a blockquote HTML tag and collapsed by default to 3 lines with the ability to show full text; not supported in secret chats
    #[serde(rename(serialize = "textEntityTypeExpandableBlockQuote", deserialize = "textEntityTypeExpandableBlockQuote"))]
    ExpandableBlockQuote,
    /// A text description shown instead of a raw URL
    #[serde(rename(serialize = "textEntityTypeTextUrl", deserialize = "textEntityTypeTextUrl"))]
    TextUrl(Box<crate::types::TextEntityTypeTextUrl>),
    /// A text shows instead of a raw mention of the user (e.g., when the user has no username)
    #[serde(rename(serialize = "textEntityTypeMentionName", deserialize = "textEntityTypeMentionName"))]
    MentionName(Box<crate::types::TextEntityTypeMentionName>),
    /// A custom emoji. The text behind a custom emoji must be an emoji. Only premium users can use premium custom emoji
    #[serde(rename(serialize = "textEntityTypeCustomEmoji", deserialize = "textEntityTypeCustomEmoji"))]
    CustomEmoji(Box<crate::types::TextEntityTypeCustomEmoji>),
    /// A media timestamp
    #[serde(rename(serialize = "textEntityTypeMediaTimestamp", deserialize = "textEntityTypeMediaTimestamp"))]
    MediaTimestamp(Box<crate::types::TextEntityTypeMediaTimestamp>),
    /// A date and time
    #[serde(rename(serialize = "textEntityTypeDateTime", deserialize = "textEntityTypeDateTime"))]
    DateTime(Box<crate::types::TextEntityTypeDateTime>),
}

impl TextEntityType {
    /// Convenience constructor to create a [`TextEntityType::PreCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn pre_code(val: crate::types::TextEntityTypePreCode) -> Self {
        Self::PreCode(Box::new(val))
    }

    /// Convenience constructor to create a [`TextEntityType::TextUrl`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_url(val: crate::types::TextEntityTypeTextUrl) -> Self {
        Self::TextUrl(Box::new(val))
    }

    /// Convenience constructor to create a [`TextEntityType::MentionName`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn mention_name(val: crate::types::TextEntityTypeMentionName) -> Self {
        Self::MentionName(Box::new(val))
    }

    /// Convenience constructor to create a [`TextEntityType::CustomEmoji`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn custom_emoji(val: crate::types::TextEntityTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }

    /// Convenience constructor to create a [`TextEntityType::MediaTimestamp`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn media_timestamp(val: crate::types::TextEntityTypeMediaTimestamp) -> Self {
        Self::MediaTimestamp(Box::new(val))
    }

    /// Convenience constructor to create a [`TextEntityType::DateTime`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn date_time(val: crate::types::TextEntityTypeDateTime) -> Self {
        Self::DateTime(Box::new(val))
    }

}

/// Converts a [`crate::types::TextEntityTypePreCode`] into [`TextEntityType`].
impl From<crate::types::TextEntityTypePreCode> for TextEntityType {
    fn from(val: crate::types::TextEntityTypePreCode) -> Self {
        Self::PreCode(Box::new(val))
    }
}

/// Converts a [`crate::types::TextEntityTypeTextUrl`] into [`TextEntityType`].
impl From<crate::types::TextEntityTypeTextUrl> for TextEntityType {
    fn from(val: crate::types::TextEntityTypeTextUrl) -> Self {
        Self::TextUrl(Box::new(val))
    }
}

/// Converts a [`crate::types::TextEntityTypeMentionName`] into [`TextEntityType`].
impl From<crate::types::TextEntityTypeMentionName> for TextEntityType {
    fn from(val: crate::types::TextEntityTypeMentionName) -> Self {
        Self::MentionName(Box::new(val))
    }
}

/// Converts a [`crate::types::TextEntityTypeCustomEmoji`] into [`TextEntityType`].
impl From<crate::types::TextEntityTypeCustomEmoji> for TextEntityType {
    fn from(val: crate::types::TextEntityTypeCustomEmoji) -> Self {
        Self::CustomEmoji(Box::new(val))
    }
}

/// Converts a [`crate::types::TextEntityTypeMediaTimestamp`] into [`TextEntityType`].
impl From<crate::types::TextEntityTypeMediaTimestamp> for TextEntityType {
    fn from(val: crate::types::TextEntityTypeMediaTimestamp) -> Self {
        Self::MediaTimestamp(Box::new(val))
    }
}

/// Converts a [`crate::types::TextEntityTypeDateTime`] into [`TextEntityType`].
impl From<crate::types::TextEntityTypeDateTime> for TextEntityType {
    fn from(val: crate::types::TextEntityTypeDateTime) -> Self {
        Self::DateTime(Box::new(val))
    }
}

/// Contains information about the time when a scheduled message will be sent
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSchedulingState {
    /// The message will be sent at the specified date
    #[serde(rename(serialize = "messageSchedulingStateSendAtDate", deserialize = "messageSchedulingStateSendAtDate"))]
    SendAtDate(Box<crate::types::MessageSchedulingStateSendAtDate>),
    /// The message will be sent when the other user is online. Applicable to private chats only and when the exact online status of the other user is known
    #[serde(rename(serialize = "messageSchedulingStateSendWhenOnline", deserialize = "messageSchedulingStateSendWhenOnline"))]
    SendWhenOnline,
    /// The message will be sent when the video in the message is converted and optimized; can be used only by the server
    #[serde(rename(serialize = "messageSchedulingStateSendWhenVideoProcessed", deserialize = "messageSchedulingStateSendWhenVideoProcessed"))]
    SendWhenVideoProcessed(Box<crate::types::MessageSchedulingStateSendWhenVideoProcessed>),
}

impl MessageSchedulingState {
    /// Convenience constructor to create a [`MessageSchedulingState::SendAtDate`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn send_at_date(val: crate::types::MessageSchedulingStateSendAtDate) -> Self {
        Self::SendAtDate(Box::new(val))
    }

    /// Convenience constructor to create a [`MessageSchedulingState::SendWhenVideoProcessed`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn send_when_video_processed(val: crate::types::MessageSchedulingStateSendWhenVideoProcessed) -> Self {
        Self::SendWhenVideoProcessed(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageSchedulingStateSendAtDate`] into [`MessageSchedulingState`].
impl From<crate::types::MessageSchedulingStateSendAtDate> for MessageSchedulingState {
    fn from(val: crate::types::MessageSchedulingStateSendAtDate) -> Self {
        Self::SendAtDate(Box::new(val))
    }
}

/// Converts a [`crate::types::MessageSchedulingStateSendWhenVideoProcessed`] into [`MessageSchedulingState`].
impl From<crate::types::MessageSchedulingStateSendWhenVideoProcessed> for MessageSchedulingState {
    fn from(val: crate::types::MessageSchedulingStateSendWhenVideoProcessed) -> Self {
        Self::SendWhenVideoProcessed(Box::new(val))
    }
}

/// Describes when a message will be self-destructed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSelfDestructType {
    /// The message will be self-destructed in the specified time after its content was opened
    #[serde(rename(serialize = "messageSelfDestructTypeTimer", deserialize = "messageSelfDestructTypeTimer"))]
    Timer(Box<crate::types::MessageSelfDestructTypeTimer>),
    /// The message can be opened only once and will be self-destructed once closed
    #[serde(rename(serialize = "messageSelfDestructTypeImmediately", deserialize = "messageSelfDestructTypeImmediately"))]
    Immediately,
}

impl MessageSelfDestructType {
    /// Convenience constructor to create a [`MessageSelfDestructType::Timer`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn timer(val: crate::types::MessageSelfDestructTypeTimer) -> Self {
        Self::Timer(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageSelfDestructTypeTimer`] into [`MessageSelfDestructType`].
impl From<crate::types::MessageSelfDestructTypeTimer> for MessageSelfDestructType {
    fn from(val: crate::types::MessageSelfDestructTypeTimer) -> Self {
        Self::Timer(Box::new(val))
    }
}

/// TDLib `MessageSendOptions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageSendOptions {
    /// Options to be used when a message is sent
    #[serde(rename(serialize = "messageSendOptions", deserialize = "messageSendOptions"))]
    MessageSendOptions(Box<crate::types::MessageSendOptions>),
}

impl MessageSendOptions {
    /// Convenience constructor to create a [`MessageSendOptions::MessageSendOptions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_send_options(val: crate::types::MessageSendOptions) -> Self {
        Self::MessageSendOptions(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageSendOptions`] into [`MessageSendOptions`].
impl From<crate::types::MessageSendOptions> for MessageSendOptions {
    fn from(val: crate::types::MessageSendOptions) -> Self {
        Self::MessageSendOptions(Box::new(val))
    }
}

/// TDLib `MessageCopyOptions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageCopyOptions {
    /// Options to be used when a message content is copied without reference to the original sender. Service messages, messages with messageInvoice, messagePaidMedia, messageGiveaway, or messageGiveawayWinners content can't be copied
    #[serde(rename(serialize = "messageCopyOptions", deserialize = "messageCopyOptions"))]
    MessageCopyOptions(Box<crate::types::MessageCopyOptions>),
}

impl MessageCopyOptions {
    /// Convenience constructor to create a [`MessageCopyOptions::MessageCopyOptions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_copy_options(val: crate::types::MessageCopyOptions) -> Self {
        Self::MessageCopyOptions(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageCopyOptions`] into [`MessageCopyOptions`].
impl From<crate::types::MessageCopyOptions> for MessageCopyOptions {
    fn from(val: crate::types::MessageCopyOptions) -> Self {
        Self::MessageCopyOptions(Box::new(val))
    }
}

/// The content of a message to send
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputMessageContent {
    /// A text message
    #[serde(rename(serialize = "inputMessageText", deserialize = "inputMessageText"))]
    InputMessageText(Box<crate::types::InputMessageText>),
    /// A rich message
    #[serde(rename(serialize = "inputMessageRichMessage", deserialize = "inputMessageRichMessage"))]
    InputMessageRichMessage(Box<crate::types::InputMessageRichMessage>),
    /// An animation message (GIF-style).
    #[serde(rename(serialize = "inputMessageAnimation", deserialize = "inputMessageAnimation"))]
    InputMessageAnimation(Box<crate::types::InputMessageAnimation>),
    /// An audio message
    #[serde(rename(serialize = "inputMessageAudio", deserialize = "inputMessageAudio"))]
    InputMessageAudio(Box<crate::types::InputMessageAudio>),
    /// A document message (general file)
    #[serde(rename(serialize = "inputMessageDocument", deserialize = "inputMessageDocument"))]
    InputMessageDocument(Box<crate::types::InputMessageDocument>),
    /// A message with paid media; can be used only in channel chats with supergroupFullInfo.has_paid_media_allowed
    #[serde(rename(serialize = "inputMessagePaidMedia", deserialize = "inputMessagePaidMedia"))]
    InputMessagePaidMedia(Box<crate::types::InputMessagePaidMedia>),
    /// A photo message
    #[serde(rename(serialize = "inputMessagePhoto", deserialize = "inputMessagePhoto"))]
    InputMessagePhoto(Box<crate::types::InputMessagePhoto>),
    /// A sticker message
    #[serde(rename(serialize = "inputMessageSticker", deserialize = "inputMessageSticker"))]
    InputMessageSticker(Box<crate::types::InputMessageSticker>),
    /// A video message
    #[serde(rename(serialize = "inputMessageVideo", deserialize = "inputMessageVideo"))]
    InputMessageVideo(Box<crate::types::InputMessageVideo>),
    /// A video note message
    #[serde(rename(serialize = "inputMessageVideoNote", deserialize = "inputMessageVideoNote"))]
    InputMessageVideoNote(Box<crate::types::InputMessageVideoNote>),
    /// A voice note message
    #[serde(rename(serialize = "inputMessageVoiceNote", deserialize = "inputMessageVoiceNote"))]
    InputMessageVoiceNote(Box<crate::types::InputMessageVoiceNote>),
    /// A message with a live location
    #[serde(rename(serialize = "inputMessageLiveLocation", deserialize = "inputMessageLiveLocation"))]
    InputMessageLiveLocation(Box<crate::types::InputMessageLiveLocation>),
    /// A message with a location
    #[serde(rename(serialize = "inputMessageLocation", deserialize = "inputMessageLocation"))]
    InputMessageLocation(Box<crate::types::InputMessageLocation>),
    /// A message with information about a venue
    #[serde(rename(serialize = "inputMessageVenue", deserialize = "inputMessageVenue"))]
    InputMessageVenue(Box<crate::types::InputMessageVenue>),
    /// A message containing a user contact
    #[serde(rename(serialize = "inputMessageContact", deserialize = "inputMessageContact"))]
    InputMessageContact(Box<crate::types::InputMessageContact>),
    /// A dice message
    #[serde(rename(serialize = "inputMessageDice", deserialize = "inputMessageDice"))]
    InputMessageDice(Box<crate::types::InputMessageDice>),
    /// A message with a game; not supported for channels or secret chats
    #[serde(rename(serialize = "inputMessageGame", deserialize = "inputMessageGame"))]
    InputMessageGame(Box<crate::types::InputMessageGame>),
    /// A message with an invoice; can be used only by bots
    #[serde(rename(serialize = "inputMessageInvoice", deserialize = "inputMessageInvoice"))]
    InputMessageInvoice(Box<crate::types::InputMessageInvoice>),
    /// A message with a poll. Polls can't be sent to secret chats and channel direct messages chats. Polls can be sent to a private chat only if the chat is a chat with a bot or the Saved Messages chat
    #[serde(rename(serialize = "inputMessagePoll", deserialize = "inputMessagePoll"))]
    InputMessagePoll(Box<crate::types::InputMessagePoll>),
    /// A stake dice message
    #[serde(rename(serialize = "inputMessageStakeDice", deserialize = "inputMessageStakeDice"))]
    InputMessageStakeDice(Box<crate::types::InputMessageStakeDice>),
    /// A message with a forwarded story. Stories can't be forwarded to secret chats. A story can be forwarded only if story.can_be_forwarded
    #[serde(rename(serialize = "inputMessageStory", deserialize = "inputMessageStory"))]
    InputMessageStory(Box<crate::types::InputMessageStory>),
    /// A message with a checklist. Checklists can't be sent to secret chats, channel chats and channel direct messages chats; for Telegram Premium users only
    #[serde(rename(serialize = "inputMessageChecklist", deserialize = "inputMessageChecklist"))]
    InputMessageChecklist(Box<crate::types::InputMessageChecklist>),
    /// A forwarded message
    #[serde(rename(serialize = "inputMessageForwarded", deserialize = "inputMessageForwarded"))]
    InputMessageForwarded(Box<crate::types::InputMessageForwarded>),
}

impl InputMessageContent {
    /// Convenience constructor to create a [`InputMessageContent::InputMessageText`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_text(val: crate::types::InputMessageText) -> Self {
        Self::InputMessageText(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageRichMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_rich_message(val: crate::types::InputMessageRichMessage) -> Self {
        Self::InputMessageRichMessage(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageAnimation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_animation(val: crate::types::InputMessageAnimation) -> Self {
        Self::InputMessageAnimation(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageAudio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_audio(val: crate::types::InputMessageAudio) -> Self {
        Self::InputMessageAudio(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageDocument`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_document(val: crate::types::InputMessageDocument) -> Self {
        Self::InputMessageDocument(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessagePaidMedia`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_paid_media(val: crate::types::InputMessagePaidMedia) -> Self {
        Self::InputMessagePaidMedia(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessagePhoto`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_photo(val: crate::types::InputMessagePhoto) -> Self {
        Self::InputMessagePhoto(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageSticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_sticker(val: crate::types::InputMessageSticker) -> Self {
        Self::InputMessageSticker(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageVideo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_video(val: crate::types::InputMessageVideo) -> Self {
        Self::InputMessageVideo(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageVideoNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_video_note(val: crate::types::InputMessageVideoNote) -> Self {
        Self::InputMessageVideoNote(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageVoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_voice_note(val: crate::types::InputMessageVoiceNote) -> Self {
        Self::InputMessageVoiceNote(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageLiveLocation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_live_location(val: crate::types::InputMessageLiveLocation) -> Self {
        Self::InputMessageLiveLocation(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageLocation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_location(val: crate::types::InputMessageLocation) -> Self {
        Self::InputMessageLocation(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageVenue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_venue(val: crate::types::InputMessageVenue) -> Self {
        Self::InputMessageVenue(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageContact`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_contact(val: crate::types::InputMessageContact) -> Self {
        Self::InputMessageContact(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageDice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_dice(val: crate::types::InputMessageDice) -> Self {
        Self::InputMessageDice(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageGame`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_game(val: crate::types::InputMessageGame) -> Self {
        Self::InputMessageGame(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageInvoice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_invoice(val: crate::types::InputMessageInvoice) -> Self {
        Self::InputMessageInvoice(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessagePoll`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_poll(val: crate::types::InputMessagePoll) -> Self {
        Self::InputMessagePoll(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageStakeDice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_stake_dice(val: crate::types::InputMessageStakeDice) -> Self {
        Self::InputMessageStakeDice(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageStory`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_story(val: crate::types::InputMessageStory) -> Self {
        Self::InputMessageStory(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageChecklist`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_checklist(val: crate::types::InputMessageChecklist) -> Self {
        Self::InputMessageChecklist(Box::new(val))
    }

    /// Convenience constructor to create a [`InputMessageContent::InputMessageForwarded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_message_forwarded(val: crate::types::InputMessageForwarded) -> Self {
        Self::InputMessageForwarded(Box::new(val))
    }

}

/// Converts a [`crate::types::InputMessageText`] into [`InputMessageContent`].
impl From<crate::types::InputMessageText> for InputMessageContent {
    fn from(val: crate::types::InputMessageText) -> Self {
        Self::InputMessageText(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageRichMessage`] into [`InputMessageContent`].
impl From<crate::types::InputMessageRichMessage> for InputMessageContent {
    fn from(val: crate::types::InputMessageRichMessage) -> Self {
        Self::InputMessageRichMessage(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageAnimation`] into [`InputMessageContent`].
impl From<crate::types::InputMessageAnimation> for InputMessageContent {
    fn from(val: crate::types::InputMessageAnimation) -> Self {
        Self::InputMessageAnimation(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageAudio`] into [`InputMessageContent`].
impl From<crate::types::InputMessageAudio> for InputMessageContent {
    fn from(val: crate::types::InputMessageAudio) -> Self {
        Self::InputMessageAudio(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageDocument`] into [`InputMessageContent`].
impl From<crate::types::InputMessageDocument> for InputMessageContent {
    fn from(val: crate::types::InputMessageDocument) -> Self {
        Self::InputMessageDocument(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessagePaidMedia`] into [`InputMessageContent`].
impl From<crate::types::InputMessagePaidMedia> for InputMessageContent {
    fn from(val: crate::types::InputMessagePaidMedia) -> Self {
        Self::InputMessagePaidMedia(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessagePhoto`] into [`InputMessageContent`].
impl From<crate::types::InputMessagePhoto> for InputMessageContent {
    fn from(val: crate::types::InputMessagePhoto) -> Self {
        Self::InputMessagePhoto(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageSticker`] into [`InputMessageContent`].
impl From<crate::types::InputMessageSticker> for InputMessageContent {
    fn from(val: crate::types::InputMessageSticker) -> Self {
        Self::InputMessageSticker(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageVideo`] into [`InputMessageContent`].
impl From<crate::types::InputMessageVideo> for InputMessageContent {
    fn from(val: crate::types::InputMessageVideo) -> Self {
        Self::InputMessageVideo(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageVideoNote`] into [`InputMessageContent`].
impl From<crate::types::InputMessageVideoNote> for InputMessageContent {
    fn from(val: crate::types::InputMessageVideoNote) -> Self {
        Self::InputMessageVideoNote(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageVoiceNote`] into [`InputMessageContent`].
impl From<crate::types::InputMessageVoiceNote> for InputMessageContent {
    fn from(val: crate::types::InputMessageVoiceNote) -> Self {
        Self::InputMessageVoiceNote(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageLiveLocation`] into [`InputMessageContent`].
impl From<crate::types::InputMessageLiveLocation> for InputMessageContent {
    fn from(val: crate::types::InputMessageLiveLocation) -> Self {
        Self::InputMessageLiveLocation(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageLocation`] into [`InputMessageContent`].
impl From<crate::types::InputMessageLocation> for InputMessageContent {
    fn from(val: crate::types::InputMessageLocation) -> Self {
        Self::InputMessageLocation(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageVenue`] into [`InputMessageContent`].
impl From<crate::types::InputMessageVenue> for InputMessageContent {
    fn from(val: crate::types::InputMessageVenue) -> Self {
        Self::InputMessageVenue(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageContact`] into [`InputMessageContent`].
impl From<crate::types::InputMessageContact> for InputMessageContent {
    fn from(val: crate::types::InputMessageContact) -> Self {
        Self::InputMessageContact(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageDice`] into [`InputMessageContent`].
impl From<crate::types::InputMessageDice> for InputMessageContent {
    fn from(val: crate::types::InputMessageDice) -> Self {
        Self::InputMessageDice(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageGame`] into [`InputMessageContent`].
impl From<crate::types::InputMessageGame> for InputMessageContent {
    fn from(val: crate::types::InputMessageGame) -> Self {
        Self::InputMessageGame(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageInvoice`] into [`InputMessageContent`].
impl From<crate::types::InputMessageInvoice> for InputMessageContent {
    fn from(val: crate::types::InputMessageInvoice) -> Self {
        Self::InputMessageInvoice(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessagePoll`] into [`InputMessageContent`].
impl From<crate::types::InputMessagePoll> for InputMessageContent {
    fn from(val: crate::types::InputMessagePoll) -> Self {
        Self::InputMessagePoll(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageStakeDice`] into [`InputMessageContent`].
impl From<crate::types::InputMessageStakeDice> for InputMessageContent {
    fn from(val: crate::types::InputMessageStakeDice) -> Self {
        Self::InputMessageStakeDice(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageStory`] into [`InputMessageContent`].
impl From<crate::types::InputMessageStory> for InputMessageContent {
    fn from(val: crate::types::InputMessageStory) -> Self {
        Self::InputMessageStory(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageChecklist`] into [`InputMessageContent`].
impl From<crate::types::InputMessageChecklist> for InputMessageContent {
    fn from(val: crate::types::InputMessageChecklist) -> Self {
        Self::InputMessageChecklist(Box::new(val))
    }
}

/// Converts a [`crate::types::InputMessageForwarded`] into [`InputMessageContent`].
impl From<crate::types::InputMessageForwarded> for InputMessageContent {
    fn from(val: crate::types::InputMessageForwarded) -> Self {
        Self::InputMessageForwarded(Box::new(val))
    }
}

/// TDLib `MessageProperties` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageProperties {
    /// Contains properties of a message and describes actions that can be done with the message right now
    #[serde(rename(serialize = "messageProperties", deserialize = "messageProperties"))]
    MessageProperties(Box<crate::types::MessageProperties>),
}

impl MessageProperties {
    /// Convenience constructor to create a [`MessageProperties::MessageProperties`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_properties(val: crate::types::MessageProperties) -> Self {
        Self::MessageProperties(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageProperties`] into [`MessageProperties`].
impl From<crate::types::MessageProperties> for MessageProperties {
    fn from(val: crate::types::MessageProperties) -> Self {
        Self::MessageProperties(Box::new(val))
    }
}

/// Represents a filter for message search results
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SearchMessagesFilter {
    /// Returns all found messages, no filter is applied
    #[serde(rename(serialize = "searchMessagesFilterEmpty", deserialize = "searchMessagesFilterEmpty"))]
    Empty,
    /// Returns only animation messages
    #[serde(rename(serialize = "searchMessagesFilterAnimation", deserialize = "searchMessagesFilterAnimation"))]
    Animation,
    /// Returns only audio messages
    #[serde(rename(serialize = "searchMessagesFilterAudio", deserialize = "searchMessagesFilterAudio"))]
    Audio,
    /// Returns only document messages
    #[serde(rename(serialize = "searchMessagesFilterDocument", deserialize = "searchMessagesFilterDocument"))]
    Document,
    /// Returns only photo messages
    #[serde(rename(serialize = "searchMessagesFilterPhoto", deserialize = "searchMessagesFilterPhoto"))]
    Photo,
    /// Returns only poll messages
    #[serde(rename(serialize = "searchMessagesFilterPoll", deserialize = "searchMessagesFilterPoll"))]
    Poll,
    /// Returns only video messages
    #[serde(rename(serialize = "searchMessagesFilterVideo", deserialize = "searchMessagesFilterVideo"))]
    Video,
    /// Returns only voice note messages
    #[serde(rename(serialize = "searchMessagesFilterVoiceNote", deserialize = "searchMessagesFilterVoiceNote"))]
    VoiceNote,
    /// Returns only photo and video messages
    #[serde(rename(serialize = "searchMessagesFilterPhotoAndVideo", deserialize = "searchMessagesFilterPhotoAndVideo"))]
    PhotoAndVideo,
    /// Returns only messages containing URLs
    #[serde(rename(serialize = "searchMessagesFilterUrl", deserialize = "searchMessagesFilterUrl"))]
    Url,
    /// Returns only messages containing chat photos
    #[serde(rename(serialize = "searchMessagesFilterChatPhoto", deserialize = "searchMessagesFilterChatPhoto"))]
    ChatPhoto,
    /// Returns only video note messages
    #[serde(rename(serialize = "searchMessagesFilterVideoNote", deserialize = "searchMessagesFilterVideoNote"))]
    VideoNote,
    /// Returns only voice and video note messages
    #[serde(rename(serialize = "searchMessagesFilterVoiceAndVideoNote", deserialize = "searchMessagesFilterVoiceAndVideoNote"))]
    VoiceAndVideoNote,
    /// Returns only messages with mentions of the current user, or messages that are replies to their messages
    #[serde(rename(serialize = "searchMessagesFilterMention", deserialize = "searchMessagesFilterMention"))]
    Mention,
    /// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query or by the sending user
    #[serde(rename(serialize = "searchMessagesFilterUnreadMention", deserialize = "searchMessagesFilterUnreadMention"))]
    UnreadMention,
    /// Returns only messages with unread reactions for the current user. When using this filter the results can't be additionally filtered by a query or by the sending user
    #[serde(rename(serialize = "searchMessagesFilterUnreadReaction", deserialize = "searchMessagesFilterUnreadReaction"))]
    UnreadReaction,
    /// Returns only messages with unread poll votes for the current user. When using this filter the results can't be additionally filtered by a query or by the sending user
    #[serde(rename(serialize = "searchMessagesFilterUnreadPollVote", deserialize = "searchMessagesFilterUnreadPollVote"))]
    UnreadPollVote,
    /// Returns only failed to send messages. This filter can be used only if the message database is used
    #[serde(rename(serialize = "searchMessagesFilterFailedToSend", deserialize = "searchMessagesFilterFailedToSend"))]
    FailedToSend,
    /// Returns only pinned messages
    #[serde(rename(serialize = "searchMessagesFilterPinned", deserialize = "searchMessagesFilterPinned"))]
    Pinned,
}

/// Represents a filter for type of the chats in which to search for messages
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SearchMessagesChatTypeFilter {
    /// Returns only messages in private chats
    #[serde(rename(serialize = "searchMessagesChatTypeFilterPrivate", deserialize = "searchMessagesChatTypeFilterPrivate"))]
    Private,
    /// Returns only messages in basic group and supergroup chats
    #[serde(rename(serialize = "searchMessagesChatTypeFilterGroup", deserialize = "searchMessagesChatTypeFilterGroup"))]
    Group,
    /// Returns only messages in channel chats
    #[serde(rename(serialize = "searchMessagesChatTypeFilterChannel", deserialize = "searchMessagesChatTypeFilterChannel"))]
    Channel,
    /// Returns only messages in the specified community
    #[serde(rename(serialize = "searchMessagesChatTypeFilterCommunity", deserialize = "searchMessagesChatTypeFilterCommunity"))]
    Community(Box<crate::types::SearchMessagesChatTypeFilterCommunity>),
}

impl SearchMessagesChatTypeFilter {
    /// Convenience constructor to create a [`SearchMessagesChatTypeFilter::Community`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn community(val: crate::types::SearchMessagesChatTypeFilterCommunity) -> Self {
        Self::Community(Box::new(val))
    }

}

/// Converts a [`crate::types::SearchMessagesChatTypeFilterCommunity`] into [`SearchMessagesChatTypeFilter`].
impl From<crate::types::SearchMessagesChatTypeFilterCommunity> for SearchMessagesChatTypeFilter {
    fn from(val: crate::types::SearchMessagesChatTypeFilterCommunity) -> Self {
        Self::Community(Box::new(val))
    }
}

/// TDLib `QuickReplyMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum QuickReplyMessage {
    /// Describes a message that can be used for quick reply
    #[serde(rename(serialize = "quickReplyMessage", deserialize = "quickReplyMessage"))]
    QuickReplyMessage(Box<crate::types::QuickReplyMessage>),
}

impl QuickReplyMessage {
    /// Convenience constructor to create a [`QuickReplyMessage::QuickReplyMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quick_reply_message(val: crate::types::QuickReplyMessage) -> Self {
        Self::QuickReplyMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::QuickReplyMessage`] into [`QuickReplyMessage`].
impl From<crate::types::QuickReplyMessage> for QuickReplyMessage {
    fn from(val: crate::types::QuickReplyMessage) -> Self {
        Self::QuickReplyMessage(Box::new(val))
    }
}

/// TDLib `QuickReplyMessages` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum QuickReplyMessages {
    /// Contains a list of quick reply messages
    #[serde(rename(serialize = "quickReplyMessages", deserialize = "quickReplyMessages"))]
    QuickReplyMessages(Box<crate::types::QuickReplyMessages>),
}

impl QuickReplyMessages {
    /// Convenience constructor to create a [`QuickReplyMessages::QuickReplyMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn quick_reply_messages(val: crate::types::QuickReplyMessages) -> Self {
        Self::QuickReplyMessages(Box::new(val))
    }

}

/// Converts a [`crate::types::QuickReplyMessages`] into [`QuickReplyMessages`].
impl From<crate::types::QuickReplyMessages> for QuickReplyMessages {
    fn from(val: crate::types::QuickReplyMessages) -> Self {
        Self::QuickReplyMessages(Box::new(val))
    }
}

/// TDLib `WelcomeMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WelcomeMessage {
    /// Describes a set up welcome message
    #[serde(rename(serialize = "welcomeMessage", deserialize = "welcomeMessage"))]
    WelcomeMessage(Box<crate::types::WelcomeMessage>),
}

impl WelcomeMessage {
    /// Convenience constructor to create a [`WelcomeMessage::WelcomeMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn welcome_message(val: crate::types::WelcomeMessage) -> Self {
        Self::WelcomeMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::WelcomeMessage`] into [`WelcomeMessage`].
impl From<crate::types::WelcomeMessage> for WelcomeMessage {
    fn from(val: crate::types::WelcomeMessage) -> Self {
        Self::WelcomeMessage(Box::new(val))
    }
}

/// TDLib `AddedReaction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AddedReaction {
    /// Represents a reaction applied to a message
    #[serde(rename(serialize = "addedReaction", deserialize = "addedReaction"))]
    AddedReaction(Box<crate::types::AddedReaction>),
}

impl AddedReaction {
    /// Convenience constructor to create a [`AddedReaction::AddedReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn added_reaction(val: crate::types::AddedReaction) -> Self {
        Self::AddedReaction(Box::new(val))
    }

}

/// Converts a [`crate::types::AddedReaction`] into [`AddedReaction`].
impl From<crate::types::AddedReaction> for AddedReaction {
    fn from(val: crate::types::AddedReaction) -> Self {
        Self::AddedReaction(Box::new(val))
    }
}

/// TDLib `AddedReactions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AddedReactions {
    /// Represents a list of reactions added to a message
    #[serde(rename(serialize = "addedReactions", deserialize = "addedReactions"))]
    AddedReactions(Box<crate::types::AddedReactions>),
}

impl AddedReactions {
    /// Convenience constructor to create a [`AddedReactions::AddedReactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn added_reactions(val: crate::types::AddedReactions) -> Self {
        Self::AddedReactions(Box::new(val))
    }

}

/// Converts a [`crate::types::AddedReactions`] into [`AddedReactions`].
impl From<crate::types::AddedReactions> for AddedReactions {
    fn from(val: crate::types::AddedReactions) -> Self {
        Self::AddedReactions(Box::new(val))
    }
}

/// TDLib `AvailableReaction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AvailableReaction {
    /// Represents an available reaction
    #[serde(rename(serialize = "availableReaction", deserialize = "availableReaction"))]
    AvailableReaction(Box<crate::types::AvailableReaction>),
}

impl AvailableReaction {
    /// Convenience constructor to create a [`AvailableReaction::AvailableReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn available_reaction(val: crate::types::AvailableReaction) -> Self {
        Self::AvailableReaction(Box::new(val))
    }

}

/// Converts a [`crate::types::AvailableReaction`] into [`AvailableReaction`].
impl From<crate::types::AvailableReaction> for AvailableReaction {
    fn from(val: crate::types::AvailableReaction) -> Self {
        Self::AvailableReaction(Box::new(val))
    }
}

/// TDLib `AvailableReactions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AvailableReactions {
    /// Represents a list of reactions that can be added to a message
    #[serde(rename(serialize = "availableReactions", deserialize = "availableReactions"))]
    AvailableReactions(Box<crate::types::AvailableReactions>),
}

impl AvailableReactions {
    /// Convenience constructor to create a [`AvailableReactions::AvailableReactions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn available_reactions(val: crate::types::AvailableReactions) -> Self {
        Self::AvailableReactions(Box::new(val))
    }

}

/// Converts a [`crate::types::AvailableReactions`] into [`AvailableReactions`].
impl From<crate::types::AvailableReactions> for AvailableReactions {
    fn from(val: crate::types::AvailableReactions) -> Self {
        Self::AvailableReactions(Box::new(val))
    }
}

/// Describes why the current user can't add reactions to the message, despite some other users can
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReactionUnavailabilityReason {
    /// The user is an anonymous administrator in the supergroup, but isn't a creator of it, so they can't vote on behalf of the supergroup
    #[serde(rename(serialize = "reactionUnavailabilityReasonAnonymousAdministrator", deserialize = "reactionUnavailabilityReasonAnonymousAdministrator"))]
    AnonymousAdministrator,
    /// The user isn't a member of the supergroup and can't send messages and reactions there without joining
    #[serde(rename(serialize = "reactionUnavailabilityReasonGuest", deserialize = "reactionUnavailabilityReasonGuest"))]
    Guest,
    /// The user is restricted in the chat
    #[serde(rename(serialize = "reactionUnavailabilityReasonRestricted", deserialize = "reactionUnavailabilityReasonRestricted"))]
    Restricted,
}

/// TDLib `InlineMessageId` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineMessageId {
    /// Contains identifier of a sent guest message
    #[serde(rename(serialize = "inlineMessageId", deserialize = "inlineMessageId"))]
    InlineMessageId(Box<crate::types::InlineMessageId>),
}

impl InlineMessageId {
    /// Convenience constructor to create a [`InlineMessageId::InlineMessageId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn inline_message_id(val: crate::types::InlineMessageId) -> Self {
        Self::InlineMessageId(Box::new(val))
    }

}

/// Converts a [`crate::types::InlineMessageId`] into [`InlineMessageId`].
impl From<crate::types::InlineMessageId> for InlineMessageId {
    fn from(val: crate::types::InlineMessageId) -> Self {
        Self::InlineMessageId(Box::new(val))
    }
}

/// TDLib `PreparedInlineMessageId` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PreparedInlineMessageId {
    /// Represents an inline message that can be sent via the bot
    #[serde(rename(serialize = "preparedInlineMessageId", deserialize = "preparedInlineMessageId"))]
    PreparedInlineMessageId(Box<crate::types::PreparedInlineMessageId>),
}

impl PreparedInlineMessageId {
    /// Convenience constructor to create a [`PreparedInlineMessageId::PreparedInlineMessageId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn prepared_inline_message_id(val: crate::types::PreparedInlineMessageId) -> Self {
        Self::PreparedInlineMessageId(Box::new(val))
    }

}

/// Converts a [`crate::types::PreparedInlineMessageId`] into [`PreparedInlineMessageId`].
impl From<crate::types::PreparedInlineMessageId> for PreparedInlineMessageId {
    fn from(val: crate::types::PreparedInlineMessageId) -> Self {
        Self::PreparedInlineMessageId(Box::new(val))
    }
}

/// TDLib `PreparedInlineMessage` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PreparedInlineMessage {
    /// Represents a ready to send inline message. Use sendInlineQueryResultMessage to send the message
    #[serde(rename(serialize = "preparedInlineMessage", deserialize = "preparedInlineMessage"))]
    PreparedInlineMessage(Box<crate::types::PreparedInlineMessage>),
}

impl PreparedInlineMessage {
    /// Convenience constructor to create a [`PreparedInlineMessage::PreparedInlineMessage`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn prepared_inline_message(val: crate::types::PreparedInlineMessage) -> Self {
        Self::PreparedInlineMessage(Box::new(val))
    }

}

/// Converts a [`crate::types::PreparedInlineMessage`] into [`PreparedInlineMessage`].
impl From<crate::types::PreparedInlineMessage> for PreparedInlineMessage {
    fn from(val: crate::types::PreparedInlineMessage) -> Self {
        Self::PreparedInlineMessage(Box::new(val))
    }
}

/// Contains content of a push message notification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PushMessageContent {
    /// A general message with hidden content
    #[serde(rename(serialize = "pushMessageContentHidden", deserialize = "pushMessageContentHidden"))]
    Hidden(Box<crate::types::PushMessageContentHidden>),
    /// An animation message (GIF-style).
    #[serde(rename(serialize = "pushMessageContentAnimation", deserialize = "pushMessageContentAnimation"))]
    Animation(Box<crate::types::PushMessageContentAnimation>),
    /// An audio message
    #[serde(rename(serialize = "pushMessageContentAudio", deserialize = "pushMessageContentAudio"))]
    Audio(Box<crate::types::PushMessageContentAudio>),
    /// A message with a user contact
    #[serde(rename(serialize = "pushMessageContentContact", deserialize = "pushMessageContentContact"))]
    Contact(Box<crate::types::PushMessageContentContact>),
    /// A contact has registered with Telegram
    #[serde(rename(serialize = "pushMessageContentContactRegistered", deserialize = "pushMessageContentContactRegistered"))]
    ContactRegistered(Box<crate::types::PushMessageContentContactRegistered>),
    /// A document message (a general file)
    #[serde(rename(serialize = "pushMessageContentDocument", deserialize = "pushMessageContentDocument"))]
    Document(Box<crate::types::PushMessageContentDocument>),
    /// A message with a game
    #[serde(rename(serialize = "pushMessageContentGame", deserialize = "pushMessageContentGame"))]
    Game(Box<crate::types::PushMessageContentGame>),
    /// A new high score was achieved in a game
    #[serde(rename(serialize = "pushMessageContentGameScore", deserialize = "pushMessageContentGameScore"))]
    GameScore(Box<crate::types::PushMessageContentGameScore>),
    /// A message with an invoice from a bot
    #[serde(rename(serialize = "pushMessageContentInvoice", deserialize = "pushMessageContentInvoice"))]
    Invoice(Box<crate::types::PushMessageContentInvoice>),
    /// A message with a location
    #[serde(rename(serialize = "pushMessageContentLocation", deserialize = "pushMessageContentLocation"))]
    Location(Box<crate::types::PushMessageContentLocation>),
    /// A message with paid media
    #[serde(rename(serialize = "pushMessageContentPaidMedia", deserialize = "pushMessageContentPaidMedia"))]
    PaidMedia(Box<crate::types::PushMessageContentPaidMedia>),
    /// A photo message
    #[serde(rename(serialize = "pushMessageContentPhoto", deserialize = "pushMessageContentPhoto"))]
    Photo(Box<crate::types::PushMessageContentPhoto>),
    /// A message with a poll
    #[serde(rename(serialize = "pushMessageContentPoll", deserialize = "pushMessageContentPoll"))]
    Poll(Box<crate::types::PushMessageContentPoll>),
    /// A message with a Telegram Premium gift code created for the user
    #[serde(rename(serialize = "pushMessageContentPremiumGiftCode", deserialize = "pushMessageContentPremiumGiftCode"))]
    PremiumGiftCode(Box<crate::types::PushMessageContentPremiumGiftCode>),
    /// A message with a giveaway
    #[serde(rename(serialize = "pushMessageContentGiveaway", deserialize = "pushMessageContentGiveaway"))]
    Giveaway(Box<crate::types::PushMessageContentGiveaway>),
    /// A message with a gift
    #[serde(rename(serialize = "pushMessageContentGift", deserialize = "pushMessageContentGift"))]
    Gift(Box<crate::types::PushMessageContentGift>),
    /// A message with an upgraded gift
    #[serde(rename(serialize = "pushMessageContentUpgradedGift", deserialize = "pushMessageContentUpgradedGift"))]
    UpgradedGift(Box<crate::types::PushMessageContentUpgradedGift>),
    /// A screenshot of a message in the chat has been taken
    #[serde(rename(serialize = "pushMessageContentScreenshotTaken", deserialize = "pushMessageContentScreenshotTaken"))]
    ScreenshotTaken,
    /// A message with a sticker
    #[serde(rename(serialize = "pushMessageContentSticker", deserialize = "pushMessageContentSticker"))]
    Sticker(Box<crate::types::PushMessageContentSticker>),
    /// A message with a story
    #[serde(rename(serialize = "pushMessageContentStory", deserialize = "pushMessageContentStory"))]
    Story(Box<crate::types::PushMessageContentStory>),
    /// A text message
    #[serde(rename(serialize = "pushMessageContentText", deserialize = "pushMessageContentText"))]
    Text(Box<crate::types::PushMessageContentText>),
    /// A message with a checklist
    #[serde(rename(serialize = "pushMessageContentChecklist", deserialize = "pushMessageContentChecklist"))]
    Checklist(Box<crate::types::PushMessageContentChecklist>),
    /// A video message
    #[serde(rename(serialize = "pushMessageContentVideo", deserialize = "pushMessageContentVideo"))]
    Video(Box<crate::types::PushMessageContentVideo>),
    /// A video note message
    #[serde(rename(serialize = "pushMessageContentVideoNote", deserialize = "pushMessageContentVideoNote"))]
    VideoNote(Box<crate::types::PushMessageContentVideoNote>),
    /// A voice note message
    #[serde(rename(serialize = "pushMessageContentVoiceNote", deserialize = "pushMessageContentVoiceNote"))]
    VoiceNote(Box<crate::types::PushMessageContentVoiceNote>),
    /// A newly created basic group
    #[serde(rename(serialize = "pushMessageContentBasicGroupChatCreate", deserialize = "pushMessageContentBasicGroupChatCreate"))]
    BasicGroupChatCreate,
    /// A video chat or live stream was started
    #[serde(rename(serialize = "pushMessageContentVideoChatStarted", deserialize = "pushMessageContentVideoChatStarted"))]
    VideoChatStarted,
    /// A video chat or live stream has ended
    #[serde(rename(serialize = "pushMessageContentVideoChatEnded", deserialize = "pushMessageContentVideoChatEnded"))]
    VideoChatEnded,
    /// An invitation of participants to a video chat or live stream
    #[serde(rename(serialize = "pushMessageContentInviteVideoChatParticipants", deserialize = "pushMessageContentInviteVideoChatParticipants"))]
    InviteVideoChatParticipants(Box<crate::types::PushMessageContentInviteVideoChatParticipants>),
    /// New chat members were invited to a group
    #[serde(rename(serialize = "pushMessageContentChatAddMembers", deserialize = "pushMessageContentChatAddMembers"))]
    ChatAddMembers(Box<crate::types::PushMessageContentChatAddMembers>),
    /// A chat photo was edited
    #[serde(rename(serialize = "pushMessageContentChatChangePhoto", deserialize = "pushMessageContentChatChangePhoto"))]
    ChatChangePhoto,
    /// A chat title was edited
    #[serde(rename(serialize = "pushMessageContentChatChangeTitle", deserialize = "pushMessageContentChatChangeTitle"))]
    ChatChangeTitle(Box<crate::types::PushMessageContentChatChangeTitle>),
    /// A chat background was edited
    #[serde(rename(serialize = "pushMessageContentChatSetBackground", deserialize = "pushMessageContentChatSetBackground"))]
    ChatSetBackground(Box<crate::types::PushMessageContentChatSetBackground>),
    /// A chat theme was edited
    #[serde(rename(serialize = "pushMessageContentChatSetTheme", deserialize = "pushMessageContentChatSetTheme"))]
    ChatSetTheme(Box<crate::types::PushMessageContentChatSetTheme>),
    /// A chat member was deleted
    #[serde(rename(serialize = "pushMessageContentChatDeleteMember", deserialize = "pushMessageContentChatDeleteMember"))]
    ChatDeleteMember(Box<crate::types::PushMessageContentChatDeleteMember>),
    /// A new member joined the chat via an invite link
    #[serde(rename(serialize = "pushMessageContentChatJoinByLink", deserialize = "pushMessageContentChatJoinByLink"))]
    ChatJoinByLink,
    /// A new member was accepted to the chat by an administrator
    #[serde(rename(serialize = "pushMessageContentChatJoinByRequest", deserialize = "pushMessageContentChatJoinByRequest"))]
    ChatJoinByRequest,
    /// A new recurring payment was made by the current user
    #[serde(rename(serialize = "pushMessageContentRecurringPayment", deserialize = "pushMessageContentRecurringPayment"))]
    RecurringPayment(Box<crate::types::PushMessageContentRecurringPayment>),
    /// A profile photo was suggested to the user
    #[serde(rename(serialize = "pushMessageContentSuggestProfilePhoto", deserialize = "pushMessageContentSuggestProfilePhoto"))]
    SuggestProfilePhoto,
    /// A birthdate was suggested to be set
    #[serde(rename(serialize = "pushMessageContentSuggestBirthdate", deserialize = "pushMessageContentSuggestBirthdate"))]
    SuggestBirthdate,
    /// A user in the chat came within proximity alert range from the current user
    #[serde(rename(serialize = "pushMessageContentProximityAlertTriggered", deserialize = "pushMessageContentProximityAlertTriggered"))]
    ProximityAlertTriggered(Box<crate::types::PushMessageContentProximityAlertTriggered>),
    /// Some tasks were added to a checklist
    #[serde(rename(serialize = "pushMessageContentChecklistTasksAdded", deserialize = "pushMessageContentChecklistTasksAdded"))]
    ChecklistTasksAdded(Box<crate::types::PushMessageContentChecklistTasksAdded>),
    /// Some tasks from a checklist were marked as done or not done
    #[serde(rename(serialize = "pushMessageContentChecklistTasksDone", deserialize = "pushMessageContentChecklistTasksDone"))]
    ChecklistTasksDone(Box<crate::types::PushMessageContentChecklistTasksDone>),
    /// An option was added to a poll
    #[serde(rename(serialize = "pushMessageContentPollOptionAdded", deserialize = "pushMessageContentPollOptionAdded"))]
    PollOptionAdded(Box<crate::types::PushMessageContentPollOptionAdded>),
    /// A forwarded messages
    #[serde(rename(serialize = "pushMessageContentMessageForwards", deserialize = "pushMessageContentMessageForwards"))]
    MessageForwards(Box<crate::types::PushMessageContentMessageForwards>),
    /// A media album
    #[serde(rename(serialize = "pushMessageContentMediaAlbum", deserialize = "pushMessageContentMediaAlbum"))]
    MediaAlbum(Box<crate::types::PushMessageContentMediaAlbum>),
}

impl PushMessageContent {
    /// Convenience constructor to create a [`PushMessageContent::Hidden`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn hidden(val: crate::types::PushMessageContentHidden) -> Self {
        Self::Hidden(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Animation`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn animation(val: crate::types::PushMessageContentAnimation) -> Self {
        Self::Animation(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Audio`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn audio(val: crate::types::PushMessageContentAudio) -> Self {
        Self::Audio(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Contact`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contact(val: crate::types::PushMessageContentContact) -> Self {
        Self::Contact(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ContactRegistered`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contact_registered(val: crate::types::PushMessageContentContactRegistered) -> Self {
        Self::ContactRegistered(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Document`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn document(val: crate::types::PushMessageContentDocument) -> Self {
        Self::Document(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Game`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game(val: crate::types::PushMessageContentGame) -> Self {
        Self::Game(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::GameScore`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn game_score(val: crate::types::PushMessageContentGameScore) -> Self {
        Self::GameScore(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Invoice`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn invoice(val: crate::types::PushMessageContentInvoice) -> Self {
        Self::Invoice(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Location`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location(val: crate::types::PushMessageContentLocation) -> Self {
        Self::Location(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::PaidMedia`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn paid_media(val: crate::types::PushMessageContentPaidMedia) -> Self {
        Self::PaidMedia(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::PushMessageContentPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Poll`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll(val: crate::types::PushMessageContentPoll) -> Self {
        Self::Poll(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::PremiumGiftCode`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn premium_gift_code(val: crate::types::PushMessageContentPremiumGiftCode) -> Self {
        Self::PremiumGiftCode(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Giveaway`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn giveaway(val: crate::types::PushMessageContentGiveaway) -> Self {
        Self::Giveaway(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Gift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn gift(val: crate::types::PushMessageContentGift) -> Self {
        Self::Gift(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::UpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift(val: crate::types::PushMessageContentUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Sticker`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn sticker(val: crate::types::PushMessageContentSticker) -> Self {
        Self::Sticker(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::PushMessageContentStory) -> Self {
        Self::Story(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Text`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text(val: crate::types::PushMessageContentText) -> Self {
        Self::Text(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Checklist`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn checklist(val: crate::types::PushMessageContentChecklist) -> Self {
        Self::Checklist(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::PushMessageContentVideo) -> Self {
        Self::Video(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::VideoNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video_note(val: crate::types::PushMessageContentVideoNote) -> Self {
        Self::VideoNote(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::VoiceNote`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn voice_note(val: crate::types::PushMessageContentVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::InviteVideoChatParticipants`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn invite_video_chat_participants(val: crate::types::PushMessageContentInviteVideoChatParticipants) -> Self {
        Self::InviteVideoChatParticipants(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ChatAddMembers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_add_members(val: crate::types::PushMessageContentChatAddMembers) -> Self {
        Self::ChatAddMembers(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ChatChangeTitle`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_change_title(val: crate::types::PushMessageContentChatChangeTitle) -> Self {
        Self::ChatChangeTitle(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ChatSetBackground`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_set_background(val: crate::types::PushMessageContentChatSetBackground) -> Self {
        Self::ChatSetBackground(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ChatSetTheme`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_set_theme(val: crate::types::PushMessageContentChatSetTheme) -> Self {
        Self::ChatSetTheme(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ChatDeleteMember`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_delete_member(val: crate::types::PushMessageContentChatDeleteMember) -> Self {
        Self::ChatDeleteMember(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::RecurringPayment`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn recurring_payment(val: crate::types::PushMessageContentRecurringPayment) -> Self {
        Self::RecurringPayment(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ProximityAlertTriggered`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn proximity_alert_triggered(val: crate::types::PushMessageContentProximityAlertTriggered) -> Self {
        Self::ProximityAlertTriggered(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ChecklistTasksAdded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn checklist_tasks_added(val: crate::types::PushMessageContentChecklistTasksAdded) -> Self {
        Self::ChecklistTasksAdded(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::ChecklistTasksDone`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn checklist_tasks_done(val: crate::types::PushMessageContentChecklistTasksDone) -> Self {
        Self::ChecklistTasksDone(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::PollOptionAdded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn poll_option_added(val: crate::types::PushMessageContentPollOptionAdded) -> Self {
        Self::PollOptionAdded(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::MessageForwards`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_forwards(val: crate::types::PushMessageContentMessageForwards) -> Self {
        Self::MessageForwards(Box::new(val))
    }

    /// Convenience constructor to create a [`PushMessageContent::MediaAlbum`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn media_album(val: crate::types::PushMessageContentMediaAlbum) -> Self {
        Self::MediaAlbum(Box::new(val))
    }

}

/// Converts a [`crate::types::PushMessageContentHidden`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentHidden> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentHidden) -> Self {
        Self::Hidden(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentAnimation`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentAnimation> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentAnimation) -> Self {
        Self::Animation(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentAudio`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentAudio> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentAudio) -> Self {
        Self::Audio(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentContact`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentContact> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentContact) -> Self {
        Self::Contact(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentContactRegistered`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentContactRegistered> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentContactRegistered) -> Self {
        Self::ContactRegistered(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentDocument`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentDocument> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentDocument) -> Self {
        Self::Document(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentGame`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentGame> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentGame) -> Self {
        Self::Game(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentGameScore`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentGameScore> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentGameScore) -> Self {
        Self::GameScore(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentInvoice`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentInvoice> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentInvoice) -> Self {
        Self::Invoice(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentLocation`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentLocation> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentLocation) -> Self {
        Self::Location(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentPaidMedia`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentPaidMedia> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentPaidMedia) -> Self {
        Self::PaidMedia(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentPhoto`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentPhoto> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentPoll`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentPoll> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentPoll) -> Self {
        Self::Poll(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentPremiumGiftCode`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentPremiumGiftCode> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentPremiumGiftCode) -> Self {
        Self::PremiumGiftCode(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentGiveaway`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentGiveaway> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentGiveaway) -> Self {
        Self::Giveaway(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentGift`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentGift> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentGift) -> Self {
        Self::Gift(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentUpgradedGift`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentUpgradedGift> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentSticker`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentSticker> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentSticker) -> Self {
        Self::Sticker(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentStory`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentStory> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentStory) -> Self {
        Self::Story(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentText`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentText> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentText) -> Self {
        Self::Text(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentChecklist`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentChecklist> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentChecklist) -> Self {
        Self::Checklist(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentVideo`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentVideo> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentVideoNote`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentVideoNote> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentVideoNote) -> Self {
        Self::VideoNote(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentVoiceNote`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentVoiceNote> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentVoiceNote) -> Self {
        Self::VoiceNote(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentInviteVideoChatParticipants`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentInviteVideoChatParticipants> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentInviteVideoChatParticipants) -> Self {
        Self::InviteVideoChatParticipants(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentChatAddMembers`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentChatAddMembers> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentChatAddMembers) -> Self {
        Self::ChatAddMembers(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentChatChangeTitle`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentChatChangeTitle> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentChatChangeTitle) -> Self {
        Self::ChatChangeTitle(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentChatSetBackground`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentChatSetBackground> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentChatSetBackground) -> Self {
        Self::ChatSetBackground(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentChatSetTheme`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentChatSetTheme> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentChatSetTheme) -> Self {
        Self::ChatSetTheme(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentChatDeleteMember`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentChatDeleteMember> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentChatDeleteMember) -> Self {
        Self::ChatDeleteMember(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentRecurringPayment`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentRecurringPayment> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentRecurringPayment) -> Self {
        Self::RecurringPayment(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentProximityAlertTriggered`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentProximityAlertTriggered> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentProximityAlertTriggered) -> Self {
        Self::ProximityAlertTriggered(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentChecklistTasksAdded`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentChecklistTasksAdded> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentChecklistTasksAdded) -> Self {
        Self::ChecklistTasksAdded(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentChecklistTasksDone`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentChecklistTasksDone> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentChecklistTasksDone) -> Self {
        Self::ChecklistTasksDone(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentPollOptionAdded`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentPollOptionAdded> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentPollOptionAdded) -> Self {
        Self::PollOptionAdded(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentMessageForwards`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentMessageForwards> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentMessageForwards) -> Self {
        Self::MessageForwards(Box::new(val))
    }
}

/// Converts a [`crate::types::PushMessageContentMediaAlbum`] into [`PushMessageContent`].
impl From<crate::types::PushMessageContentMediaAlbum> for PushMessageContent {
    fn from(val: crate::types::PushMessageContentMediaAlbum) -> Self {
        Self::MediaAlbum(Box::new(val))
    }
}

/// Describes result of canSendMessageToUser
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanSendMessageToUserResult {
    /// The user can be messaged
    #[serde(rename(serialize = "canSendMessageToUserResultOk", deserialize = "canSendMessageToUserResultOk"))]
    Ok,
    /// The user can be messaged, but the messages are paid
    #[serde(rename(serialize = "canSendMessageToUserResultUserHasPaidMessages", deserialize = "canSendMessageToUserResultUserHasPaidMessages"))]
    UserHasPaidMessages(Box<crate::types::CanSendMessageToUserResultUserHasPaidMessages>),
    /// The user can't be messaged, because they are deleted or unknown
    #[serde(rename(serialize = "canSendMessageToUserResultUserIsDeleted", deserialize = "canSendMessageToUserResultUserIsDeleted"))]
    UserIsDeleted,
    /// The user can't be messaged, because they restrict new chats with non-contacts
    #[serde(rename(serialize = "canSendMessageToUserResultUserRestrictsNewChats", deserialize = "canSendMessageToUserResultUserRestrictsNewChats"))]
    UserRestrictsNewChats,
}

impl CanSendMessageToUserResult {
    /// Convenience constructor to create a [`CanSendMessageToUserResult::UserHasPaidMessages`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_has_paid_messages(val: crate::types::CanSendMessageToUserResultUserHasPaidMessages) -> Self {
        Self::UserHasPaidMessages(Box::new(val))
    }

}

/// Converts a [`crate::types::CanSendMessageToUserResultUserHasPaidMessages`] into [`CanSendMessageToUserResult`].
impl From<crate::types::CanSendMessageToUserResultUserHasPaidMessages> for CanSendMessageToUserResult {
    fn from(val: crate::types::CanSendMessageToUserResultUserHasPaidMessages) -> Self {
        Self::UserHasPaidMessages(Box::new(val))
    }
}

/// TDLib `MessageAutoDeleteTime` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageAutoDeleteTime {
    /// Contains default auto-delete timer setting for new chats
    #[serde(rename(serialize = "messageAutoDeleteTime", deserialize = "messageAutoDeleteTime"))]
    MessageAutoDeleteTime(Box<crate::types::MessageAutoDeleteTime>),
}

impl MessageAutoDeleteTime {
    /// Convenience constructor to create a [`MessageAutoDeleteTime::MessageAutoDeleteTime`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_auto_delete_time(val: crate::types::MessageAutoDeleteTime) -> Self {
        Self::MessageAutoDeleteTime(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageAutoDeleteTime`] into [`MessageAutoDeleteTime`].
impl From<crate::types::MessageAutoDeleteTime> for MessageAutoDeleteTime {
    fn from(val: crate::types::MessageAutoDeleteTime) -> Self {
        Self::MessageAutoDeleteTime(Box::new(val))
    }
}

/// TDLib `MessageLink` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageLink {
    /// Contains an HTTPS link to a message in a supergroup or channel, or a forum topic
    #[serde(rename(serialize = "messageLink", deserialize = "messageLink"))]
    MessageLink(Box<crate::types::MessageLink>),
}

impl MessageLink {
    /// Convenience constructor to create a [`MessageLink::MessageLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_link(val: crate::types::MessageLink) -> Self {
        Self::MessageLink(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageLink`] into [`MessageLink`].
impl From<crate::types::MessageLink> for MessageLink {
    fn from(val: crate::types::MessageLink) -> Self {
        Self::MessageLink(Box::new(val))
    }
}

/// TDLib `MessageLinkInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageLinkInfo {
    /// Contains information about a link to a message or a forum topic in a chat
    #[serde(rename(serialize = "messageLinkInfo", deserialize = "messageLinkInfo"))]
    MessageLinkInfo(Box<crate::types::MessageLinkInfo>),
}

impl MessageLinkInfo {
    /// Convenience constructor to create a [`MessageLinkInfo::MessageLinkInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_link_info(val: crate::types::MessageLinkInfo) -> Self {
        Self::MessageLinkInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageLinkInfo`] into [`MessageLinkInfo`].
impl From<crate::types::MessageLinkInfo> for MessageLinkInfo {
    fn from(val: crate::types::MessageLinkInfo) -> Self {
        Self::MessageLinkInfo(Box::new(val))
    }
}

/// TDLib `Text` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Text {
    /// Contains some text
    #[serde(rename(serialize = "text", deserialize = "text"))]
    Text(Box<crate::types::Text>),
}

impl Text {
    /// Convenience constructor to create a [`Text::Text`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text(val: crate::types::Text) -> Self {
        Self::Text(Box::new(val))
    }

}

/// Converts a [`crate::types::Text`] into [`Text`].
impl From<crate::types::Text> for Text {
    fn from(val: crate::types::Text) -> Self {
        Self::Text(Box::new(val))
    }
}

/// Describes the way the text needs to be parsed for text entities
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TextParseMode {
    /// The text uses Markdown-style formatting
    #[serde(rename(serialize = "textParseModeMarkdown", deserialize = "textParseModeMarkdown"))]
    Markdown(Box<crate::types::TextParseModeMarkdown>),
    /// The text uses HTML-style formatting. The same as Telegram Bot API "HTML" parse mode
    #[serde(rename(serialize = "textParseModeHTML", deserialize = "textParseModeHTML"))]
    Html,
}

impl TextParseMode {
    /// Convenience constructor to create a [`TextParseMode::Markdown`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn markdown(val: crate::types::TextParseModeMarkdown) -> Self {
        Self::Markdown(Box::new(val))
    }

}

/// Converts a [`crate::types::TextParseModeMarkdown`] into [`TextParseMode`].
impl From<crate::types::TextParseModeMarkdown> for TextParseMode {
    fn from(val: crate::types::TextParseModeMarkdown) -> Self {
        Self::Markdown(Box::new(val))
    }
}

/// TDLib `ChatStatisticsMessageSenderInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatStatisticsMessageSenderInfo {
    /// Contains statistics about messages sent by a user
    #[serde(rename(serialize = "chatStatisticsMessageSenderInfo", deserialize = "chatStatisticsMessageSenderInfo"))]
    ChatStatisticsMessageSenderInfo(Box<crate::types::ChatStatisticsMessageSenderInfo>),
}

impl ChatStatisticsMessageSenderInfo {
    /// Convenience constructor to create a [`ChatStatisticsMessageSenderInfo::ChatStatisticsMessageSenderInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_statistics_message_sender_info(val: crate::types::ChatStatisticsMessageSenderInfo) -> Self {
        Self::ChatStatisticsMessageSenderInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatStatisticsMessageSenderInfo`] into [`ChatStatisticsMessageSenderInfo`].
impl From<crate::types::ChatStatisticsMessageSenderInfo> for ChatStatisticsMessageSenderInfo {
    fn from(val: crate::types::ChatStatisticsMessageSenderInfo) -> Self {
        Self::ChatStatisticsMessageSenderInfo(Box::new(val))
    }
}

/// TDLib `MessageStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MessageStatistics {
    /// A detailed statistics about a message
    #[serde(rename(serialize = "messageStatistics", deserialize = "messageStatistics"))]
    MessageStatistics(Box<crate::types::MessageStatistics>),
}

impl MessageStatistics {
    /// Convenience constructor to create a [`MessageStatistics::MessageStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message_statistics(val: crate::types::MessageStatistics) -> Self {
        Self::MessageStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::MessageStatistics`] into [`MessageStatistics`].
impl From<crate::types::MessageStatistics> for MessageStatistics {
    fn from(val: crate::types::MessageStatistics) -> Self {
        Self::MessageStatistics(Box::new(val))
    }
}

