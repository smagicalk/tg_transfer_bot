#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A story failed to post. If the story posting is canceled, then updateStoryDeleted will be received instead of this update
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryPostFailed {
    /// The failed to post story
    pub story: crate::types::Story,
    /// The cause of the story posting failure
    pub error: crate::types::Error,
    /// Type of the error; may be null if unknown
    pub error_type: Option<crate::enums::CanPostStoryResult>,
}
