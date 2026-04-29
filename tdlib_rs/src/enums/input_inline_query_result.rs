#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputInlineQueryResult {
    /// Represents a link to an animated GIF or an animated (i.e., without sound) H.264/MPEG-4 AVC video
    #[serde(rename(serialize = "inputInlineQueryResultAnimation", deserialize = "inputInlineQueryResultAnimation"))]
    Animation(crate::types::InputInlineQueryResultAnimation),
    /// Represents a link to an article or web page
    #[serde(rename(serialize = "inputInlineQueryResultArticle", deserialize = "inputInlineQueryResultArticle"))]
    Article(crate::types::InputInlineQueryResultArticle),
    /// Represents a link to an MP3 audio file
    #[serde(rename(serialize = "inputInlineQueryResultAudio", deserialize = "inputInlineQueryResultAudio"))]
    Audio(crate::types::InputInlineQueryResultAudio),
    /// Represents a user contact
    #[serde(rename(serialize = "inputInlineQueryResultContact", deserialize = "inputInlineQueryResultContact"))]
    Contact(crate::types::InputInlineQueryResultContact),
    /// Represents a link to a file
    #[serde(rename(serialize = "inputInlineQueryResultDocument", deserialize = "inputInlineQueryResultDocument"))]
    Document(crate::types::InputInlineQueryResultDocument),
    /// Represents a game
    #[serde(rename(serialize = "inputInlineQueryResultGame", deserialize = "inputInlineQueryResultGame"))]
    Game(crate::types::InputInlineQueryResultGame),
    /// Represents a point on the map
    #[serde(rename(serialize = "inputInlineQueryResultLocation", deserialize = "inputInlineQueryResultLocation"))]
    Location(crate::types::InputInlineQueryResultLocation),
    /// Represents link to a JPEG image
    #[serde(rename(serialize = "inputInlineQueryResultPhoto", deserialize = "inputInlineQueryResultPhoto"))]
    Photo(crate::types::InputInlineQueryResultPhoto),
    /// Represents a link to a WEBP, TGS, or WEBM sticker
    #[serde(rename(serialize = "inputInlineQueryResultSticker", deserialize = "inputInlineQueryResultSticker"))]
    Sticker(crate::types::InputInlineQueryResultSticker),
    /// Represents information about a venue
    #[serde(rename(serialize = "inputInlineQueryResultVenue", deserialize = "inputInlineQueryResultVenue"))]
    Venue(crate::types::InputInlineQueryResultVenue),
    /// Represents a link to a page containing an embedded video player or a video file
    #[serde(rename(serialize = "inputInlineQueryResultVideo", deserialize = "inputInlineQueryResultVideo"))]
    Video(crate::types::InputInlineQueryResultVideo),
    /// Represents a link to an opus-encoded audio file within an OGG container, single channel audio
    #[serde(rename(serialize = "inputInlineQueryResultVoiceNote", deserialize = "inputInlineQueryResultVoiceNote"))]
    VoiceNote(crate::types::InputInlineQueryResultVoiceNote),
}
