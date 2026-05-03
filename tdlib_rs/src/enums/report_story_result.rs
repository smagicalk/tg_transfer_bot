#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportStoryResult {
    /// The story was reported successfully
    #[serde(rename(serialize = "reportStoryResultOk", deserialize = "reportStoryResultOk"))]
    Ok,
    /// The user must choose an option to report the story and repeat request with the chosen option
    #[serde(rename(
        serialize = "reportStoryResultOptionRequired",
        deserialize = "reportStoryResultOptionRequired"
    ))]
    OptionRequired(crate::types::ReportStoryResultOptionRequired),
    /// The user must add additional text details to the report
    #[serde(rename(
        serialize = "reportStoryResultTextRequired",
        deserialize = "reportStoryResultTextRequired"
    ))]
    TextRequired(crate::types::ReportStoryResultTextRequired),
}
