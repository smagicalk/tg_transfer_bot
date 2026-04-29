#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LinkPreviewOptions {
    /// Options to be used for generation of a link preview
    #[serde(rename(serialize = "linkPreviewOptions", deserialize = "linkPreviewOptions"))]
    LinkPreviewOptions(crate::types::LinkPreviewOptions),
}
