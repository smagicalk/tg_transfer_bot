#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a recommended chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RecommendedChatFolder {
    /// The chat folder
    pub folder: crate::types::ChatFolder,
    /// Chat folder description
    pub description: String,
}
