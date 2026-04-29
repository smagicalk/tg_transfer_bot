#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A custom suggestion to be shown at the top of the chat list
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionCustom {
    /// Unique name of the suggestion
    pub name: String,
    /// Title of the suggestion
    pub title: crate::types::FormattedText,
    /// Description of the suggestion
    pub description: crate::types::FormattedText,
    /// The link to open when the suggestion is clicked
    pub url: String,
}
