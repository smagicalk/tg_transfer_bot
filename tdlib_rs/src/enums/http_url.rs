#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum HttpUrl {
    /// Contains an HTTP URL
    #[serde(rename(serialize = "httpUrl", deserialize = "httpUrl"))]
    HttpUrl(crate::types::HttpUrl),
}
