#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundWebApp {
    /// Contains information about a Web App found by its short name
    #[serde(rename(serialize = "foundWebApp", deserialize = "foundWebApp"))]
    FoundWebApp(crate::types::FoundWebApp),
}
