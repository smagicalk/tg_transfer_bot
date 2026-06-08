#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Represents a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Story {
    /// Unique story identifier among stories posted by the given chat
    pub id: i32,
    /// Identifier of the chat that posted the story
    pub poster_chat_id: i64,
    /// Identifier of the user or chat that posted the story; may be null if the story is posted on behalf of the poster_chat_id
    pub poster_id: Option<crate::enums::MessageSender>,
    /// Point in time (Unix timestamp) when the story was published
    pub date: i32,
    /// True, if the story is being posted by the current user
    pub is_being_posted: bool,
    /// True, if the story is being edited by the current user
    pub is_being_edited: bool,
    /// True, if the story was edited
    pub is_edited: bool,
    /// True, if the story is saved in the profile of the chat that posted it and will be available there after expiration
    pub is_posted_to_chat_page: bool,
    /// True, if the story is visible only for the current user
    pub is_visible_only_for_self: bool,
    /// True, if the story can be added to an album using createStoryAlbum and addStoryAlbumStories
    pub can_be_added_to_album: bool,
    /// True, if the story can be deleted
    pub can_be_deleted: bool,
    /// True, if the story can be edited
    pub can_be_edited: bool,
    /// True, if the story can be forwarded as a message or reposted as a story. Otherwise, screenshotting and saving of the story content must be also forbidden
    pub can_be_forwarded: bool,
    /// True, if the story can be replied in the chat with the user who posted the story
    pub can_be_replied: bool,
    /// True, if the story privacy settings can be changed
    pub can_set_privacy_settings: bool,
    /// True, if the story's is_posted_to_chat_page value can be changed
    pub can_toggle_is_posted_to_chat_page: bool,
    /// True, if the story statistics are available through getStoryStatistics
    pub can_get_statistics: bool,
    /// True, if interactions with the story can be received through getStoryInteractions
    pub can_get_interactions: bool,
    /// True, if users viewed the story can't be received, because the story has expired more than getOption("story_viewers_expiration_delay") seconds ago
    pub has_expired_viewers: bool,
    /// Information about the original story; may be null if the story wasn't reposted
    pub repost_info: Option<crate::types::StoryRepostInfo>,
    /// Information about interactions with the story; may be null if the story isn't owned or there were no interactions
    pub interaction_info: Option<crate::types::StoryInteractionInfo>,
    /// Type of the chosen reaction; may be null if none
    pub chosen_reaction_type: Option<crate::enums::ReactionType>,
    /// Privacy rules affecting story visibility; may be approximate for non-owned stories
    pub privacy_settings: crate::enums::StoryPrivacySettings,
    /// Content of the story
    pub content: crate::enums::StoryContent,
    /// Clickable areas to be shown on the story content
    pub areas: Vec<crate::types::StoryArea>,
    /// Caption of the story
    pub caption: crate::types::FormattedText,
    /// Identifiers of story albums to which the story is added; only for manageable stories
    pub album_ids: Vec<i32>,
}
