#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InlineQueryResult {
    /// Represents a link to an article or web page
    #[serde(rename(
        serialize = "inlineQueryResultArticle",
        deserialize = "inlineQueryResultArticle"
    ))]
    Article(crate::types::InlineQueryResultArticle),
    /// Represents a user contact
    #[serde(rename(
        serialize = "inlineQueryResultContact",
        deserialize = "inlineQueryResultContact"
    ))]
    Contact(crate::types::InlineQueryResultContact),
    /// Represents a point on the map
    #[serde(rename(
        serialize = "inlineQueryResultLocation",
        deserialize = "inlineQueryResultLocation"
    ))]
    Location(crate::types::InlineQueryResultLocation),
    /// Represents information about a venue
    #[serde(rename(
        serialize = "inlineQueryResultVenue",
        deserialize = "inlineQueryResultVenue"
    ))]
    Venue(crate::types::InlineQueryResultVenue),
    /// Represents information about a game
    #[serde(rename(
        serialize = "inlineQueryResultGame",
        deserialize = "inlineQueryResultGame"
    ))]
    Game(crate::types::InlineQueryResultGame),
    /// Represents an animation file
    #[serde(rename(
        serialize = "inlineQueryResultAnimation",
        deserialize = "inlineQueryResultAnimation"
    ))]
    Animation(crate::types::InlineQueryResultAnimation),
    /// Represents an audio file
    #[serde(rename(
        serialize = "inlineQueryResultAudio",
        deserialize = "inlineQueryResultAudio"
    ))]
    Audio(crate::types::InlineQueryResultAudio),
    /// Represents a document
    #[serde(rename(
        serialize = "inlineQueryResultDocument",
        deserialize = "inlineQueryResultDocument"
    ))]
    Document(crate::types::InlineQueryResultDocument),
    /// Represents a photo
    #[serde(rename(
        serialize = "inlineQueryResultPhoto",
        deserialize = "inlineQueryResultPhoto"
    ))]
    Photo(crate::types::InlineQueryResultPhoto),
    /// Represents a sticker
    #[serde(rename(
        serialize = "inlineQueryResultSticker",
        deserialize = "inlineQueryResultSticker"
    ))]
    Sticker(crate::types::InlineQueryResultSticker),
    /// Represents a video
    #[serde(rename(
        serialize = "inlineQueryResultVideo",
        deserialize = "inlineQueryResultVideo"
    ))]
    Video(crate::types::InlineQueryResultVideo),
    /// Represents a voice note
    #[serde(rename(
        serialize = "inlineQueryResultVoiceNote",
        deserialize = "inlineQueryResultVoiceNote"
    ))]
    VoiceNote(crate::types::InlineQueryResultVoiceNote),
}
