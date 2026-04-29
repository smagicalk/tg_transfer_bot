#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about original story that was reposted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryRepostInfo {
    /// Origin of the story that was reposted
    pub origin: crate::enums::StoryOrigin,
    /// True, if story content was modified during reposting; otherwise, story wasn't modified
    pub is_content_modified: bool,
}
