#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about the main Web App of a bot
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MainWebApp {
    /// URL of the Web App to open
    pub url: String,
    /// The mode in which the Web App must be opened
    pub mode: crate::enums::WebAppOpenMode,
}
