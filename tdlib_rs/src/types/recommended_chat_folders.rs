#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains a list of recommended chat folders
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct RecommendedChatFolders {
    /// List of recommended chat folders
    pub chat_folders: Vec<crate::types::RecommendedChatFolder>,
}
