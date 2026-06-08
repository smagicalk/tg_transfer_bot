#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportOption {
    /// Describes an option to report an entity to Telegram
    #[serde(rename(serialize = "reportOption", deserialize = "reportOption"))]
    ReportOption(crate::types::ReportOption),
}
