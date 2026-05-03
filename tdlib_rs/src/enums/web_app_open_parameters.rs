#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebAppOpenParameters {
    /// Options to be used when a Web App is opened
    #[serde(rename(
        serialize = "webAppOpenParameters",
        deserialize = "webAppOpenParameters"
    ))]
    WebAppOpenParameters(crate::types::WebAppOpenParameters),
}
