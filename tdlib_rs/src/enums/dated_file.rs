#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DatedFile {
    /// File with the date it was uploaded
    #[serde(rename(serialize = "datedFile", deserialize = "datedFile"))]
    DatedFile(crate::types::DatedFile),
}
