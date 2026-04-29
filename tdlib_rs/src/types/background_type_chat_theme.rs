#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A background from a chat theme based on an emoji; can be used only as a chat background in channels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BackgroundTypeChatTheme {
    /// Name of the emoji chat theme
    pub theme_name: String,
}
