#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a live story. Call searchPublicChat with the given chat username, then getChatActiveStories to get active stories in the chat,
/// then find a live story among active stories of the chat, and then joinLiveStory to join the live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeLiveStory {
    /// Username of the poster of the story
    pub story_poster_username: String,
}
