#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Basic information about a Saved Messages topic has changed. This update is guaranteed to come before the topic identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateSavedMessagesTopic {
    /// New data about the topic
    pub topic: crate::types::SavedMessagesTopic,
}
