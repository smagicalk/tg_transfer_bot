#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebAppOpenMode {
    /// The Web App is opened in the compact mode
    #[serde(rename(serialize = "webAppOpenModeCompact", deserialize = "webAppOpenModeCompact"))]
    Compact,
    /// The Web App is opened in the full-size mode
    #[serde(rename(serialize = "webAppOpenModeFullSize", deserialize = "webAppOpenModeFullSize"))]
    FullSize,
    /// The Web App is opened in the full-screen mode
    #[serde(rename(serialize = "webAppOpenModeFullScreen", deserialize = "webAppOpenModeFullScreen"))]
    FullScreen,
}
