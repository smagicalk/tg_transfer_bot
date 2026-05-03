#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The story can be viewed by certain specified users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryPrivacySettingsSelectedUsers {
    /// Identifiers of the users; always unknown and empty for non-owned stories
    pub user_ids: Vec<i64>,
}
