#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to open the story posting interface
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeNewStory {
    /// The type of the content of the story to post; may be null if unspecified
    pub content_type: Option<crate::enums::StoryContentType>,
}
