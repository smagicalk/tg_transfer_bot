#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The story can be viewed by all contacts except chosen users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryPrivacySettingsContacts {
    /// User identifiers of the contacts that can't see the story; always unknown and empty for non-owned stories
    pub except_user_ids: Vec<i64>,
}
