#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StartLiveStoryResult {
    /// The live story was successfully posted
    #[serde(rename(serialize = "startLiveStoryResultOk", deserialize = "startLiveStoryResultOk"))]
    Ok(crate::types::StartLiveStoryResultOk),
    /// The live story failed to post with an error to be handled
    #[serde(rename(serialize = "startLiveStoryResultFail", deserialize = "startLiveStoryResultFail"))]
    Fail(crate::types::StartLiveStoryResultFail),
}
