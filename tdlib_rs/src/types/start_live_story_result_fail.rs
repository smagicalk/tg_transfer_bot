#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The live story failed to post with an error to be handled
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StartLiveStoryResultFail {
    /// Type of the error; other error types may be returned as regular errors
    pub error_type: crate::enums::CanPostStoryResult,
}
