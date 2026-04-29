#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebAppInfo {
    /// Contains information about a Web App
    #[serde(rename(serialize = "webAppInfo", deserialize = "webAppInfo"))]
    WebAppInfo(crate::types::WebAppInfo),
}
