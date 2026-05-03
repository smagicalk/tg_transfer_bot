#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockCaption {
    /// Contains a caption of another block
    #[serde(rename(serialize = "pageBlockCaption", deserialize = "pageBlockCaption"))]
    PageBlockCaption(crate::types::PageBlockCaption),
}
