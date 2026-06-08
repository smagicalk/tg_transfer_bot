#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A story can be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultOk {
    /// Number of stories that can be posted by the user
    pub story_count: i32,
}
