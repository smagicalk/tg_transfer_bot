#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LogTags {
    /// Contains a list of available TDLib internal log tags
    #[serde(rename(serialize = "logTags", deserialize = "logTags"))]
    LogTags(crate::types::LogTags),
}
