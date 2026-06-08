#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputMessageContent {
    /// A text message
    #[serde(rename(serialize = "inputMessageText", deserialize = "inputMessageText"))]
    InputMessageText(crate::types::InputMessageText),
    /// An animation message (GIF-style).
    #[serde(rename(
        serialize = "inputMessageAnimation",
        deserialize = "inputMessageAnimation"
    ))]
    InputMessageAnimation(crate::types::InputMessageAnimation),
    /// An audio message
    #[serde(rename(serialize = "inputMessageAudio", deserialize = "inputMessageAudio"))]
    InputMessageAudio(crate::types::InputMessageAudio),
    /// A document message (general file)
    #[serde(rename(
        serialize = "inputMessageDocument",
        deserialize = "inputMessageDocument"
    ))]
    InputMessageDocument(crate::types::InputMessageDocument),
    /// A message with paid media; can be used only in channel chats with supergroupFullInfo.has_paid_media_allowed
    #[serde(rename(
        serialize = "inputMessagePaidMedia",
        deserialize = "inputMessagePaidMedia"
    ))]
    InputMessagePaidMedia(crate::types::InputMessagePaidMedia),
    /// A photo message
    #[serde(rename(serialize = "inputMessagePhoto", deserialize = "inputMessagePhoto"))]
    InputMessagePhoto(crate::types::InputMessagePhoto),
    /// A sticker message
    #[serde(rename(serialize = "inputMessageSticker", deserialize = "inputMessageSticker"))]
    InputMessageSticker(crate::types::InputMessageSticker),
    /// A video message
    #[serde(rename(serialize = "inputMessageVideo", deserialize = "inputMessageVideo"))]
    InputMessageVideo(crate::types::InputMessageVideo),
    /// A video note message
    #[serde(rename(
        serialize = "inputMessageVideoNote",
        deserialize = "inputMessageVideoNote"
    ))]
    InputMessageVideoNote(crate::types::InputMessageVideoNote),
    /// A voice note message
    #[serde(rename(
        serialize = "inputMessageVoiceNote",
        deserialize = "inputMessageVoiceNote"
    ))]
    InputMessageVoiceNote(crate::types::InputMessageVoiceNote),
    /// A message with a location
    #[serde(rename(
        serialize = "inputMessageLocation",
        deserialize = "inputMessageLocation"
    ))]
    InputMessageLocation(crate::types::InputMessageLocation),
    /// A message with information about a venue
    #[serde(rename(serialize = "inputMessageVenue", deserialize = "inputMessageVenue"))]
    InputMessageVenue(crate::types::InputMessageVenue),
    /// A message containing a user contact
    #[serde(rename(serialize = "inputMessageContact", deserialize = "inputMessageContact"))]
    InputMessageContact(crate::types::InputMessageContact),
    /// A dice message
    #[serde(rename(serialize = "inputMessageDice", deserialize = "inputMessageDice"))]
    InputMessageDice(crate::types::InputMessageDice),
    /// A message with a game; not supported for channels or secret chats
    #[serde(rename(serialize = "inputMessageGame", deserialize = "inputMessageGame"))]
    InputMessageGame(crate::types::InputMessageGame),
    /// A message with an invoice; can be used only by bots
    #[serde(rename(serialize = "inputMessageInvoice", deserialize = "inputMessageInvoice"))]
    InputMessageInvoice(crate::types::InputMessageInvoice),
    /// A message with a poll. Polls can't be sent to secret chats and channel direct messages chats. Polls can be sent to a private chat only if the chat is a chat with a bot or the Saved Messages chat
    #[serde(rename(serialize = "inputMessagePoll", deserialize = "inputMessagePoll"))]
    InputMessagePoll(crate::types::InputMessagePoll),
    /// A stake dice message
    #[serde(rename(
        serialize = "inputMessageStakeDice",
        deserialize = "inputMessageStakeDice"
    ))]
    InputMessageStakeDice(crate::types::InputMessageStakeDice),
    /// A message with a forwarded story. Stories can't be forwarded to secret chats. A story can be forwarded only if story.can_be_forwarded
    #[serde(rename(serialize = "inputMessageStory", deserialize = "inputMessageStory"))]
    InputMessageStory(crate::types::InputMessageStory),
    /// A message with a checklist. Checklists can't be sent to secret chats, channel chats and channel direct messages chats; for Telegram Premium users only
    #[serde(rename(
        serialize = "inputMessageChecklist",
        deserialize = "inputMessageChecklist"
    ))]
    InputMessageChecklist(crate::types::InputMessageChecklist),
    /// A forwarded message
    #[serde(rename(
        serialize = "inputMessageForwarded",
        deserialize = "inputMessageForwarded"
    ))]
    InputMessageForwarded(crate::types::InputMessageForwarded),
}
