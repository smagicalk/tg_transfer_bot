//!
//! TDLib `story` domain types.
//!
//! Types, enums, and functions for Telegram Stories and story interactions.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a storyboard for a video
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct VideoStoryboard {
    /// A JPEG file that contains tiled previews of video
    pub storyboard_file: crate::types::File,
    /// Width of a tile
    pub width: i32,
    /// Height of a tile
    pub height: i32,
    /// File that describes mapping of position in the video to a tile in the JPEG file
    pub map_file: crate::types::File,
}

/// The chat has an active live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ActiveStoryStateLive {
    /// Identifier of the active live story
    pub story_id: i32,
}

/// The chat has some unread active stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ActiveStoryStateUnread {
}

/// The chat has active stories, all of which were read
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ActiveStoryStateRead {
}

/// Contains a list of users and chats that spend most money on paid messages and reactions in a live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LiveStoryDonors {
    /// Total amount of spend Telegram Stars
    pub total_star_count: i64,
    /// List of top donors in the live story
    pub top_donors: Vec<crate::types::PaidReactor>,
}

/// Describes a story replied by a given message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageReplyToStory {
    /// The identifier of the poster of the story
    pub story_poster_chat_id: i64,
    /// The identifier of the story
    pub story_id: i32,
}

/// Describes a story to be replied
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageReplyToStory {
    /// The identifier of the poster of the story. Currently, stories can be replied only in the chat that posted the story; channel stories can't be replied
    pub story_poster_chat_id: i64,
    /// The identifier of the story
    pub story_id: i32,
}

/// The message is from a chat history
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceChatHistory {
}

/// The message is from history of a message thread
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceMessageThreadHistory {
}

/// The message is from history of a forum topic
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceForumTopicHistory {
}

/// The message is from history of a topic in a channel direct messages chat administered by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceDirectMessagesChatTopicHistory {
}

/// The message is from chat, message thread or forum topic history preview
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageSourceHistoryPreview {
}

/// The link is a link to a live story group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeLiveStory {
    /// The identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
}

/// The link is a link to a story. Link preview description is unavailable
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeStory {
    /// The identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
}

/// The link is a link to an album of stories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LinkPreviewTypeStoryAlbum {
    /// Icon of the album; may be null if none
    pub photo_icon: Option<crate::types::Photo>,
    /// Video icon of the album; may be null if none
    pub video_icon: Option<crate::types::Video>,
}

/// A message with a forwarded story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageStory {
    /// Identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
    /// True, if the story was automatically forwarded because of a mention of the user
    pub via_mention: bool,
}

/// A message with a forwarded story. Stories can't be forwarded to secret chats. A story can be forwarded only if story.can_be_forwarded
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageStory {
    /// Identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
}

/// Describes position of a clickable rectangle area on a story media
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaPosition {
    /// The abscissa of the rectangle's center, as a percentage of the media width
    pub x_percentage: f64,
    /// The ordinate of the rectangle's center, as a percentage of the media height
    pub y_percentage: f64,
    /// The width of the rectangle, as a percentage of the media width
    pub width_percentage: f64,
    /// The height of the rectangle, as a percentage of the media height
    pub height_percentage: f64,
    /// Clockwise rotation angle of the rectangle, in degrees; 0-360
    pub rotation_angle: f64,
    /// The radius of the rectangle corner rounding, as a percentage of the media width
    pub corner_radius_percentage: f64,
}

/// An area pointing to a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeLocation {
    /// The location
    pub location: crate::types::Location,
    /// Address of the location; may be null if unknown
    pub address: Option<crate::types::LocationAddress>,
}

/// An area pointing to a venue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeVenue {
    /// Information about the venue
    pub venue: crate::types::Venue,
}

/// An area pointing to a suggested reaction. App needs to show a clickable reaction on the area and call setStoryReaction when the area is clicked
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the reaction
    pub reaction_type: crate::enums::ReactionType,
    /// Number of times the reaction was added
    pub total_count: i32,
    /// True, if reaction has a dark background
    pub is_dark: bool,
    /// True, if reaction corner is flipped
    pub is_flipped: bool,
}

/// An area pointing to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeMessage {
    /// Identifier of the chat with the message
    pub chat_id: i64,
    /// Identifier of the message
    pub message_id: i64,
}

/// An area pointing to a HTTP or tg: link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeLink {
    /// HTTP or tg: URL to be opened when the area is clicked
    pub url: String,
}

/// An area with information about weather
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeWeather {
    /// Temperature, in degree Celsius
    pub temperature: f64,
    /// Emoji representing the weather
    pub emoji: String,
    /// A color of the area background in the ARGB format
    pub background_color: i32,
}

/// An area with an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAreaTypeUpgradedGift {
    /// Unique name of the upgraded gift
    pub gift_name: String,
}

/// Describes a clickable rectangle area on a story media
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryArea {
    /// Position of the area
    pub position: crate::types::StoryAreaPosition,
    /// Type of the area
    #[serde(rename = "type")]
    pub r#type: crate::enums::StoryAreaType,
}

/// An area pointing to a location
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeLocation {
    /// The location
    pub location: crate::types::Location,
    /// Address of the location; pass null if unknown
    pub address: Option<crate::types::LocationAddress>,
}

/// An area pointing to a venue found by the bot getOption("venue_search_bot_username")
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeFoundVenue {
    /// Identifier of the inline query, used to found the venue
    #[serde_as(as = "DisplayFromStr")]
    pub query_id: i64,
    /// Identifier of the inline query result
    pub result_id: String,
}

/// An area pointing to a venue already added to the story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypePreviousVenue {
    /// Provider of the venue
    pub venue_provider: String,
    /// Identifier of the venue in the provider database
    pub venue_id: String,
}

/// An area pointing to a suggested reaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeSuggestedReaction {
    /// Type of the reaction
    pub reaction_type: crate::enums::ReactionType,
    /// True, if reaction has a dark background
    pub is_dark: bool,
    /// True, if reaction corner is flipped
    pub is_flipped: bool,
}

/// An area pointing to a message
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeMessage {
    /// Identifier of the chat with the message. Currently, the chat must be a supergroup or a channel chat
    pub chat_id: i64,
    /// Identifier of the message. Use messageProperties.can_be_shared_in_story to check whether the message is suitable
    pub message_id: i64,
}

/// An area pointing to a HTTP or tg: link
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeLink {
    /// HTTP or tg: URL to be opened when the area is clicked
    pub url: String,
}

/// An area with information about weather
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeWeather {
    /// Temperature, in degree Celsius
    pub temperature: f64,
    /// Emoji representing the weather
    pub emoji: String,
    /// A color of the area background in the ARGB format
    pub background_color: i32,
}

/// An area with an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreaTypeUpgradedGift {
    /// Unique name of the upgraded gift
    pub gift_name: String,
}

/// Describes a clickable rectangle area on a story media to be added
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputStoryArea {
    /// Position of the area
    pub position: crate::types::StoryAreaPosition,
    /// Type of the area
    #[serde(rename = "type")]
    pub r#type: crate::enums::InputStoryAreaType,
}

/// Contains a list of story areas to be added
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputStoryAreas {
    /// List of input story areas. Currently, a story can have
    /// up to 10 inputStoryAreaTypeLocation, inputStoryAreaTypeFoundVenue, and inputStoryAreaTypePreviousVenue areas,
    /// up to getOption("story_suggested_reaction_area_count_max") inputStoryAreaTypeSuggestedReaction areas,
    /// up to 1 inputStoryAreaTypeMessage area,
    /// up to getOption("story_link_area_count_max") inputStoryAreaTypeLink areas if the current user is a Telegram Premium user,
    /// up to 3 inputStoryAreaTypeWeather areas, and
    /// up to 1 inputStoryAreaTypeUpgradedGift area
    pub areas: Vec<crate::types::InputStoryArea>,
}

/// Describes a video file posted as a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryVideo {
    /// Duration of the video, in seconds
    pub duration: f64,
    /// Video width
    pub width: i32,
    /// Video height
    pub height: i32,
    /// True, if stickers were added to the video. The list of corresponding sticker sets can be received using getAttachedStickerSets
    pub has_stickers: bool,
    /// True, if the video has no sound
    pub is_animation: bool,
    /// Video minithumbnail; may be null
    pub minithumbnail: Option<crate::types::Minithumbnail>,
    /// Video thumbnail in JPEG or MPEG4 format; may be null
    pub thumbnail: Option<crate::types::Thumbnail>,
    /// Size of file prefix, which is expected to be preloaded, in bytes
    pub preload_prefix_size: i32,
    /// Timestamp of the frame used as video thumbnail
    pub cover_frame_timestamp: f64,
    /// File containing the video
    pub video: crate::types::File,
}

/// A photo story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentTypePhoto {
}

/// A video story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentTypeVideo {
}

/// A live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentTypeLive {
}

/// A story of unknown content type
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentTypeUnsupported {
}

/// A photo story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentPhoto {
    /// The photo
    pub photo: crate::types::Photo,
}

/// A video story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryContentVideo {
    /// The video in MPEG4 format
    pub video: crate::types::StoryVideo,
    /// Alternative version of the video in MPEG4 format, encoded with H.264 codec; may be null
    pub alternative_video: Option<crate::types::StoryVideo>,
}

/// A live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentLive {
    /// Identifier of the corresponding group call. The group call can be received through the method getGroupCall
    pub group_call_id: i32,
    /// True, if the call is an RTMP stream instead of an ordinary group call
    pub is_rtmp_stream: bool,
}

/// A story content that is not supported in the current TDLib version
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryContentUnsupported {
}

/// A photo story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputStoryContentPhoto {
    /// Photo to send. The photo must be at most 10 MB in size. The photo size must be 1080x1920
    pub photo: crate::enums::InputFile,
    /// File identifiers of the stickers added to the photo, if applicable
    pub added_sticker_file_ids: Vec<i32>,
}

/// A video story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct InputStoryContentVideo {
    /// Video to be sent. The video size must be 720x1280. The video must be streamable and stored in MPEG4 format, after encoding with H.265 codec and key frames added each second
    pub video: crate::enums::InputFile,
    /// File identifiers of the stickers added to the video, if applicable
    pub added_sticker_file_ids: Vec<i32>,
    /// Precise duration of the video, in seconds; 0-60
    pub duration: f64,
    /// Timestamp of the frame, which will be used as video thumbnail
    pub cover_frame_timestamp: f64,
    /// True, if the video has no sound
    pub is_animation: bool,
}

/// The list of stories, shown in the main chat list and folder chat lists
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryListMain {
}

/// The list of stories, shown in the Archive chat list
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryListArchive {
}

/// The original story was a public story that was posted by a known chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryOriginPublicStory {
    /// Identifier of the chat that posted original story
    pub chat_id: i64,
    /// Story identifier of the original story
    pub story_id: i32,
}

/// The original story was posted by an unknown user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryOriginHiddenUser {
    /// Name of the user or the chat that posted the story
    pub poster_name: String,
}

/// Contains information about original story that was reposted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryRepostInfo {
    /// Origin of the story that was reposted
    pub origin: crate::enums::StoryOrigin,
    /// True, if story content was modified during reposting; otherwise, story wasn't modified
    pub is_content_modified: bool,
}

/// Contains information about interactions with a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractionInfo {
    /// Number of times the story was viewed
    pub view_count: i32,
    /// Number of times the story was forwarded; 0 if none or unknown
    pub forward_count: i32,
    /// Number of reactions added to the story; 0 if none or unknown
    pub reaction_count: i32,
    /// Identifiers of at most 3 recent viewers of the story
    pub recent_viewer_user_ids: Vec<i64>,
}

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
    /// True, if users who viewed the story can't be received, because the story has expired more than getOption("story_viewers_expiration_delay") seconds ago
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

/// Represents a list of stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Stories {
    /// Approximate total number of stories found
    pub total_count: i32,
    /// The list of stories
    pub stories: Vec<crate::types::Story>,
    /// Identifiers of the pinned stories; returned only in getChatPostedToChatPageStories with from_story_id == 0
    pub pinned_story_ids: Vec<i32>,
}

/// Contains a list of stories found by a search
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundStories {
    /// Approximate total number of stories found
    pub total_count: i32,
    /// List of stories
    pub stories: Vec<crate::types::Story>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Describes album of stories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryAlbum {
    /// Unique identifier of the album
    pub id: i32,
    /// Name of the album
    pub name: String,
    /// Icon of the album; may be null if none
    pub photo_icon: Option<crate::types::Photo>,
    /// Video icon of the album; may be null if none
    pub video_icon: Option<crate::types::Video>,
}

/// Represents a list of story albums
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryAlbums {
    /// List of story albums
    pub albums: Vec<crate::types::StoryAlbum>,
}

/// Contains identifier of a story along with identifier of the chat that posted it
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryFullId {
    /// Identifier of the chat that posted the story
    pub poster_chat_id: i64,
    /// Unique story identifier among stories of the chat
    pub story_id: i32,
}

/// Contains basic information about a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryInfo {
    /// Unique story identifier among stories of the chat
    pub story_id: i32,
    /// Point in time (Unix timestamp) when the story was published
    pub date: i32,
    /// True, if the story is available only to close friends
    pub is_for_close_friends: bool,
    /// True, if the story is a live story
    pub is_live: bool,
}

/// Describes active stories posted by a chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatActiveStories {
    /// Identifier of the chat that posted the stories
    pub chat_id: i64,
    /// Identifier of the story list in which the stories are shown; may be null if the stories aren't shown in a story list
    pub list: Option<crate::enums::StoryList>,
    /// A parameter used to determine order of the stories in the story list; 0 if the stories don't need to be shown in the story list. Stories must be sorted by the pair (order, story_poster_chat_id) in descending order
    pub order: i64,
    /// True, if the stories are shown in the main story list and can be archived; otherwise, the stories can be hidden from the main story list
    /// only by calling removeTopChat with topChatCategoryUsers and the chat_id. Stories of the current user can't be archived nor hidden using removeTopChat
    pub can_be_archived: bool,
    /// Identifier of the last read active story
    pub max_read_story_id: i32,
    /// Basic information about the stories; use getStory to get full information about the stories. The stories are in chronological order (i.e., in order of increasing story identifiers)
    pub stories: Vec<crate::types::StoryInfo>,
}

/// A view of the story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractionTypeView {
    /// Type of the reaction that was chosen by the viewer; may be null if none
    pub chosen_reaction_type: Option<crate::enums::ReactionType>,
}

/// A forward of the story as a message
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractionTypeForward {
    /// The message with story forward
    pub message: crate::types::Message,
}

/// A repost of the story as a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractionTypeRepost {
    /// The reposted story
    pub story: crate::types::Story,
}

/// Represents interaction with a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryInteraction {
    /// Identifier of the user or chat that made the interaction
    pub actor_id: crate::enums::MessageSender,
    /// Approximate point in time (Unix timestamp) when the interaction happened
    pub interaction_date: i32,
    /// Block list to which the actor is added; may be null if none or for chat stories
    pub block_list: Option<crate::enums::BlockList>,
    /// Type of the interaction
    #[serde(rename = "type")]
    pub r#type: crate::enums::StoryInteractionType,
}

/// Represents a list of interactions with a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryInteractions {
    /// Approximate total number of interactions found
    pub total_count: i32,
    /// Approximate total number of found forwards and reposts; always 0 for chat stories
    pub total_forward_count: i32,
    /// Approximate total number of found reactions; always 0 for chat stories
    pub total_reaction_count: i32,
    /// List of story interactions
    pub interactions: Vec<crate::types::StoryInteraction>,
    /// The offset for the next request. If empty, then there are no more results
    pub next_offset: String,
}

/// Contains a public repost to a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PublicForwardStory {
    /// Information about the story
    pub story: crate::types::Story,
}

/// The is_all_history_available setting of a supergroup was toggled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventIsAllHistoryAvailableToggled {
    /// New value of is_all_history_available
    pub is_all_history_available: bool,
}

/// The maximum number of active stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeActiveStoryCount {
}

/// The maximum number of stories posted per week
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeWeeklyPostedStoryCount {
}

/// The maximum number of stories posted per month
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeMonthlyPostedStoryCount {
}

/// The maximum length of captions of posted stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeStoryCaptionLength {
}

/// The maximum number of suggested reaction areas on a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumLimitTypeStorySuggestedReactionAreaCount {
}

/// Allowed to use many additional features for stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumFeatureUpgradedStories {
}

/// Allowed to use many additional features for stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessFeatureUpgradedStories {
}

/// Stories of the current user are displayed before stories of non-Premium contacts, supergroups, and channels
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumStoryFeaturePriorityOrder {
}

/// The ability to hide the fact that the user viewed other's stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumStoryFeatureStealthMode {
}

/// The ability to check who opened the current user's stories after they expire
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumStoryFeaturePermanentViewsHistory {
}

/// The ability to set custom expiration duration for stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumStoryFeatureCustomExpirationDuration {
}

/// The ability to save other's unprotected stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumStoryFeatureSaveStories {
}

/// The ability to use links and formatting in story caption, and use inputStoryAreaTypeLink areas
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumStoryFeatureLinksAndFormatting {
}

/// The ability to choose better quality for viewed stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PremiumStoryFeatureVideoQuality {
}

/// A user tried to use a Premium story feature
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PremiumSourceStoryFeature {
    /// The used feature
    pub feature: crate::enums::PremiumStoryFeature,
}

/// A story can be sent
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultOk {
    /// Number of stories that can be posted by the user
    pub story_count: i32,
}

/// The user must subscribe to Telegram Premium to be able to post stories
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultPremiumNeeded {
}

/// The chat must be boosted first by Telegram Premium subscribers to post more stories. Call getChatBoostStatus to get current boost status of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultBoostNeeded {
}

/// The limit for the number of active stories exceeded. The user can buy Telegram Premium, delete an active story, or wait for the oldest story to expire
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultActiveStoryLimitExceeded {
}

/// The weekly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultWeeklyLimitExceeded {
    /// Time left before the user can post the next story, in seconds
    pub retry_after: i32,
}

/// The monthly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultMonthlyLimitExceeded {
    /// Time left before the user can post the next story, in seconds
    pub retry_after: i32,
}

/// The user or the chat has an active live story. The live story must be deleted first
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanPostStoryResultLiveStoryIsActive {
    /// Identifier of the active live story
    pub story_id: i32,
}

/// The live story was successfully posted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StartLiveStoryResultOk {
    /// The live story
    pub story: crate::types::Story,
}

/// The live story failed to post with an error to be handled
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StartLiveStoryResultFail {
    /// Type of the error; other error types may be returned as regular errors
    pub error_type: crate::enums::CanPostStoryResult,
}

/// A message with a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PushMessageContentStory {
    /// True, if the user was mentioned in the story
    pub is_mention: bool,
    /// True, if the message is a pinned message with the specified content
    pub is_pinned: bool,
}

/// The story can be viewed by everyone
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryPrivacySettingsEveryone {
    /// Identifiers of the users that can't see the story; always unknown and empty for non-owned stories
    pub except_user_ids: Vec<i64>,
}

/// The story can be viewed by all contacts except chosen users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryPrivacySettingsContacts {
    /// User identifiers of the contacts that can't see the story; always unknown and empty for non-owned stories
    pub except_user_ids: Vec<i64>,
}

/// The story can be viewed by all close friends
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryPrivacySettingsCloseFriends {
}

/// The story can be viewed by certain specified users
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StoryPrivacySettingsSelectedUsers {
    /// Identifiers of the users; always unknown and empty for non-owned stories
    pub user_ids: Vec<i64>,
}

/// The story was reported successfully
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportStoryResultOk {
}

/// The user must choose an option to report the story and repeat request with the chosen option
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportStoryResultOptionRequired {
    /// Title for the option choice
    pub title: String,
    /// List of available options
    pub options: Vec<crate::types::ReportOption>,
}

/// The user must add additional text details to the report
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ReportStoryResultTextRequired {
    /// Option identifier for the next reportStory request
    pub option_id: String,
    /// True, if the user can skip text adding
    pub is_optional: bool,
}

/// The link is a link to a live story. Call searchPublicChat with the given chat username, then getChatActiveStories to get active stories in the chat,
/// then find a live story among active stories of the chat, and then joinLiveStory to join the live story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeLiveStory {
    /// Username of the poster of the story
    pub story_poster_username: String,
}

/// The link is a link to open the story posting interface
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeNewStory {
    /// The type of the content of the story to post; may be null if unspecified
    pub content_type: Option<crate::enums::StoryContentType>,
}

/// The link is a link to a story. Call searchPublicChat with the given poster username, then call getStory with the received chat identifier and the given story identifier, then show the story if received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeStory {
    /// Username of the poster of the story
    pub story_poster_username: String,
    /// Story identifier
    pub story_id: i32,
}

/// The link is a link to an album of stories. Call searchPublicChat with the given username, then call getStoryAlbumStories with the received chat identifier
/// and the given story album identifier, then show the story album if received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeStoryAlbum {
    /// Username of the owner of the story album
    pub story_album_owner_username: String,
    /// Story album identifier
    pub story_album_id: i32,
}

/// The block list that disallows viewing of stories of the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BlockListStories {
}

/// The file is a photo published as a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FileTypePhotoStory {
}

/// The file is a video published as a story
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FileTypeVideoStory {
}

/// Describes a story posted on behalf of the chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatStatisticsObjectTypeStory {
    /// Story identifier
    pub story_id: i32,
}

/// A detailed statistics about a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StoryStatistics {
    /// A graph containing number of story views and shares
    pub story_interaction_graph: crate::enums::StatisticalGraph,
    /// A graph containing number of story reactions
    pub story_reaction_graph: crate::enums::StatisticalGraph,
}

/// The list of top donors in live story group call has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateLiveStoryTopDonors {
    /// Identifier of the group call
    pub group_call_id: i32,
    /// New list of live story donors
    pub donors: crate::types::LiveStoryDonors,
}

/// A story was changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStory {
    /// The new information about the story
    pub story: crate::types::Story,
}

/// A story became inaccessible
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryDeleted {
    /// Identifier of the chat that posted the story
    pub story_poster_chat_id: i64,
    /// Story identifier
    pub story_id: i32,
}

/// A story has been successfully posted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryPostSucceeded {
    /// The posted story
    pub story: crate::types::Story,
    /// The previous temporary story identifier
    pub old_story_id: i32,
}

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

/// The list of active stories posted by a specific chat has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateChatActiveStories {
    /// The new list of active stories
    pub active_stories: crate::types::ChatActiveStories,
}

/// Number of chats in a story list has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryListChatCount {
    /// The story list
    pub story_list: crate::enums::StoryList,
    /// Approximate total number of chats with active stories in the list
    pub chat_count: i32,
}

/// Story stealth mode settings have changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateStoryStealthMode {
    /// Point in time (Unix timestamp) until stealth mode is active; 0 if it is disabled
    pub active_until_date: i32,
    /// Point in time (Unix timestamp) when stealth mode can be enabled again; 0 if there is no active cooldown
    pub cooldown_until_date: i32,
}

