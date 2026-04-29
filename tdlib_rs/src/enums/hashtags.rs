#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Hashtags {
    /// Contains a list of hashtags
    #[serde(rename(serialize = "hashtags", deserialize = "hashtags"))]
    Hashtags(crate::types::Hashtags),
}
