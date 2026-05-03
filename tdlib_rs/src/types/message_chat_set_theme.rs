#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A theme in the chat has been changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatSetTheme {
    /// New theme for the chat; may be null if chat theme was reset to the default one
    pub theme: Option<crate::enums::ChatTheme>,
}
