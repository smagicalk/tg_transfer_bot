#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The link is a link to an album of stories. Call searchPublicChat with the given username, then call getStoryAlbumStories with the received chat identifier
/// and the given story album identifier, then show the story album if received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeStoryAlbum {
    /// Username of the owner of the story album
    pub story_album_owner_username: String,
    /// Story album identifier
    pub story_album_id: i32,
}
