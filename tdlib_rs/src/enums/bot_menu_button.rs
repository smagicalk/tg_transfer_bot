#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotMenuButton {
    /// Describes a button to be shown instead of bot commands menu button
    #[serde(rename(serialize = "botMenuButton", deserialize = "botMenuButton"))]
    BotMenuButton(crate::types::BotMenuButton),
}
