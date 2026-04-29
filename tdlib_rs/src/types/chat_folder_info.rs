#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains basic information about a chat folder
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatFolderInfo {
    /// Unique chat folder identifier
    pub id: i32,
    /// The name of the folder
    pub name: crate::types::ChatFolderName,
    /// The chosen or default icon for the chat folder
    pub icon: crate::types::ChatFolderIcon,
    /// The identifier of the chosen color for the chat folder icon; from -1 to 6. If -1, then color is disabled
    pub color_id: i32,
    /// True, if at least one link has been created for the folder
    pub is_shareable: bool,
    /// True, if the chat folder has invite links created by the current user
    pub has_my_invite_links: bool,
}
