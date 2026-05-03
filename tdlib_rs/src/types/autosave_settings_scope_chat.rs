#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Autosave settings applied to a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AutosaveSettingsScopeChat {
    /// Chat identifier
    pub chat_id: i64,
}
