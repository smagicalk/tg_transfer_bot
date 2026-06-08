#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat theme based on an upgraded gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ChatThemeGift {
    /// The chat theme
    pub gift_theme: crate::types::GiftChatTheme,
}
