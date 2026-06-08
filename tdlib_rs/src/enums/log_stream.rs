#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LogStream {
    /// The log is written to stderr or an OS specific log
    #[serde(rename(serialize = "logStreamDefault", deserialize = "logStreamDefault"))]
    Default,
    /// The log is written to a file
    #[serde(rename(serialize = "logStreamFile", deserialize = "logStreamFile"))]
    File(crate::types::LogStreamFile),
    /// The log is written nowhere
    #[serde(rename(serialize = "logStreamEmpty", deserialize = "logStreamEmpty"))]
    Empty,
}
