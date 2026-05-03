#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a passkey
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Passkey {
    /// Unique identifier of the passkey
    pub id: String,
    /// Name of the passkey
    pub name: String,
    /// Point in time (Unix timestamp) when the passkey was added
    pub addition_date: i32,
    /// Point in time (Unix timestamp) when the passkey was used last time; 0 if never
    pub last_usage_date: i32,
    /// Identifier of the custom emoji that is used as the icon of the software, which created the passkey; 0 if unknown
    #[serde_as(as = "DisplayFromStr")]
    pub software_icon_custom_emoji_id: i64,
}
