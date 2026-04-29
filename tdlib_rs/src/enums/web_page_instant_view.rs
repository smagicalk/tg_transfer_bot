#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum WebPageInstantView {
    /// Describes an instant view page for a web page
    #[serde(rename(serialize = "webPageInstantView", deserialize = "webPageInstantView"))]
    WebPageInstantView(crate::types::WebPageInstantView),
}
