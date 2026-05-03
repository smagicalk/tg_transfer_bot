#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AccentColor {
    /// Contains information about supported accent color for user/chat name, background of empty chat photo, replies to messages and link previews
    #[serde(rename(serialize = "accentColor", deserialize = "accentColor"))]
    AccentColor(crate::types::AccentColor),
}
