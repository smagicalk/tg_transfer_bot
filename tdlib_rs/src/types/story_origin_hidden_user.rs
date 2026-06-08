#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The original story was posted by an unknown user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryOriginHiddenUser {
    /// Name of the user or the chat that posted the story
    pub poster_name: String,
}
