#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user must choose an option to report the chat and repeat request with the chosen option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportChatResultOptionRequired {
    /// Title for the option choice
    pub title: String,
    /// List of available options
    pub options: Vec<crate::types::ReportOption>,
}
