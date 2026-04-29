#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockRelatedArticle {
    /// Contains information about a related article
    #[serde(rename(serialize = "pageBlockRelatedArticle", deserialize = "pageBlockRelatedArticle"))]
    PageBlockRelatedArticle(crate::types::PageBlockRelatedArticle),
}
