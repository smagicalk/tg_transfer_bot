#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebApp {
    /// Describes a Web App. Use getInternalLink with internalLinkTypeWebApp to share the Web App
    #[serde(rename(serialize = "webApp", deserialize = "webApp"))]
    WebApp(crate::types::WebApp),
}
