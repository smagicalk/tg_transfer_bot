#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat folder settings section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionChatFolders {
    /// Subsection of the section; may be one of
    /// "", "edit", "create", "add-recommended", "show-tags", "tab-view"
    pub subsection: String,
}
