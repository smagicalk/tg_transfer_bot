#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The live story was successfully posted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StartLiveStoryResultOk {
    /// The live story
    pub story: crate::types::Story,
}
