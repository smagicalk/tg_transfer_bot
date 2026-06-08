#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a Web App
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct WebAppInfo {
    /// Unique identifier for the Web App launch
    #[serde_as(as = "DisplayFromStr")]
    pub launch_id: i64,
    /// A Web App URL to open in a web view
    pub url: String,
}
