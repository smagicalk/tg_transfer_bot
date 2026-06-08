#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A theme based on an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputChatThemeGift {
    /// Name of the upgraded gift. A gift can be used only in one chat in a time.
    /// When the same gift is used in another chat, theme in the previous chat is reset to default
    pub name: String,
}
