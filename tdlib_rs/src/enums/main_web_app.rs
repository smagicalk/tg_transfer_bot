#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum MainWebApp {
    /// Contains information about the main Web App of a bot
    #[serde(rename(serialize = "mainWebApp", deserialize = "mainWebApp"))]
    MainWebApp(crate::types::MainWebApp),
}
