#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The application language section
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SettingsSectionLanguage {
    /// Subsection of the section; may be one of "", "show-button" for Show Translate Button toggle,
    /// "translate-chats" for Translate Entire Chats toggle, "do-not-translate" - for Do Not Translate language list
    pub subsection: String,
}
