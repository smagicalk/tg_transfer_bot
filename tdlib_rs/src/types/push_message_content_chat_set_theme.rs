#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A chat theme was edited
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentChatSetTheme {
    /// If non-empty, human-readable name of the new theme. Otherwise, the chat theme was reset to the default one
    pub name: String,
}
