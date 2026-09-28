//!
//! TDLib `story` domain enums.
//!
//! Types, enums, and functions for Telegram Stories and story interactions.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// TDLib `VideoStoryboard` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VideoStoryboard {
    /// Describes a storyboard for a video
    #[serde(rename(serialize = "videoStoryboard", deserialize = "videoStoryboard"))]
    VideoStoryboard(Box<crate::types::VideoStoryboard>),
}

impl VideoStoryboard {
    /// Convenience constructor to create a [`VideoStoryboard::VideoStoryboard`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video_storyboard(val: crate::types::VideoStoryboard) -> Self {
        Self::VideoStoryboard(Box::new(val))
    }

}

/// Converts a [`crate::types::VideoStoryboard`] into [`VideoStoryboard`].
impl From<crate::types::VideoStoryboard> for VideoStoryboard {
    fn from(val: crate::types::VideoStoryboard) -> Self {
        Self::VideoStoryboard(Box::new(val))
    }
}

/// Describes state of active stories posted by a chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ActiveStoryState {
    /// The chat has an active live story
    #[serde(rename(serialize = "activeStoryStateLive", deserialize = "activeStoryStateLive"))]
    Live(Box<crate::types::ActiveStoryStateLive>),
    /// The chat has some unread active stories
    #[serde(rename(serialize = "activeStoryStateUnread", deserialize = "activeStoryStateUnread"))]
    Unread,
    /// The chat has active stories, all of which were read
    #[serde(rename(serialize = "activeStoryStateRead", deserialize = "activeStoryStateRead"))]
    Read,
}

impl ActiveStoryState {
    /// Convenience constructor to create a [`ActiveStoryState::Live`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn live(val: crate::types::ActiveStoryStateLive) -> Self {
        Self::Live(Box::new(val))
    }

}

/// Converts a [`crate::types::ActiveStoryStateLive`] into [`ActiveStoryState`].
impl From<crate::types::ActiveStoryStateLive> for ActiveStoryState {
    fn from(val: crate::types::ActiveStoryStateLive) -> Self {
        Self::Live(Box::new(val))
    }
}

/// TDLib `LiveStoryDonors` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LiveStoryDonors {
    /// Contains a list of users and chats that spend most money on paid messages and reactions in a live story
    #[serde(rename(serialize = "liveStoryDonors", deserialize = "liveStoryDonors"))]
    LiveStoryDonors(Box<crate::types::LiveStoryDonors>),
}

impl LiveStoryDonors {
    /// Convenience constructor to create a [`LiveStoryDonors::LiveStoryDonors`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn live_story_donors(val: crate::types::LiveStoryDonors) -> Self {
        Self::LiveStoryDonors(Box::new(val))
    }

}

/// Converts a [`crate::types::LiveStoryDonors`] into [`LiveStoryDonors`].
impl From<crate::types::LiveStoryDonors> for LiveStoryDonors {
    fn from(val: crate::types::LiveStoryDonors) -> Self {
        Self::LiveStoryDonors(Box::new(val))
    }
}

/// TDLib `StoryAreaPosition` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryAreaPosition {
    /// Describes position of a clickable rectangle area on a story media
    #[serde(rename(serialize = "storyAreaPosition", deserialize = "storyAreaPosition"))]
    StoryAreaPosition(Box<crate::types::StoryAreaPosition>),
}

impl StoryAreaPosition {
    /// Convenience constructor to create a [`StoryAreaPosition::StoryAreaPosition`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_area_position(val: crate::types::StoryAreaPosition) -> Self {
        Self::StoryAreaPosition(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryAreaPosition`] into [`StoryAreaPosition`].
impl From<crate::types::StoryAreaPosition> for StoryAreaPosition {
    fn from(val: crate::types::StoryAreaPosition) -> Self {
        Self::StoryAreaPosition(Box::new(val))
    }
}

/// Describes type of clickable area on a story media
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryAreaType {
    /// An area pointing to a location
    #[serde(rename(serialize = "storyAreaTypeLocation", deserialize = "storyAreaTypeLocation"))]
    Location(Box<crate::types::StoryAreaTypeLocation>),
    /// An area pointing to a venue
    #[serde(rename(serialize = "storyAreaTypeVenue", deserialize = "storyAreaTypeVenue"))]
    Venue(Box<crate::types::StoryAreaTypeVenue>),
    /// An area pointing to a suggested reaction. App needs to show a clickable reaction on the area and call setStoryReaction when the area is clicked
    #[serde(rename(serialize = "storyAreaTypeSuggestedReaction", deserialize = "storyAreaTypeSuggestedReaction"))]
    SuggestedReaction(Box<crate::types::StoryAreaTypeSuggestedReaction>),
    /// An area pointing to a message
    #[serde(rename(serialize = "storyAreaTypeMessage", deserialize = "storyAreaTypeMessage"))]
    Message(Box<crate::types::StoryAreaTypeMessage>),
    /// An area pointing to a HTTP or tg: link
    #[serde(rename(serialize = "storyAreaTypeLink", deserialize = "storyAreaTypeLink"))]
    Link(Box<crate::types::StoryAreaTypeLink>),
    /// An area with information about weather
    #[serde(rename(serialize = "storyAreaTypeWeather", deserialize = "storyAreaTypeWeather"))]
    Weather(Box<crate::types::StoryAreaTypeWeather>),
    /// An area with an upgraded gift
    #[serde(rename(serialize = "storyAreaTypeUpgradedGift", deserialize = "storyAreaTypeUpgradedGift"))]
    UpgradedGift(Box<crate::types::StoryAreaTypeUpgradedGift>),
}

impl StoryAreaType {
    /// Convenience constructor to create a [`StoryAreaType::Location`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location(val: crate::types::StoryAreaTypeLocation) -> Self {
        Self::Location(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryAreaType::Venue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn venue(val: crate::types::StoryAreaTypeVenue) -> Self {
        Self::Venue(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryAreaType::SuggestedReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn suggested_reaction(val: crate::types::StoryAreaTypeSuggestedReaction) -> Self {
        Self::SuggestedReaction(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryAreaType::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::StoryAreaTypeMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryAreaType::Link`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn link(val: crate::types::StoryAreaTypeLink) -> Self {
        Self::Link(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryAreaType::Weather`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn weather(val: crate::types::StoryAreaTypeWeather) -> Self {
        Self::Weather(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryAreaType::UpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift(val: crate::types::StoryAreaTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryAreaTypeLocation`] into [`StoryAreaType`].
impl From<crate::types::StoryAreaTypeLocation> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeLocation) -> Self {
        Self::Location(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryAreaTypeVenue`] into [`StoryAreaType`].
impl From<crate::types::StoryAreaTypeVenue> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeVenue) -> Self {
        Self::Venue(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryAreaTypeSuggestedReaction`] into [`StoryAreaType`].
impl From<crate::types::StoryAreaTypeSuggestedReaction> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeSuggestedReaction) -> Self {
        Self::SuggestedReaction(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryAreaTypeMessage`] into [`StoryAreaType`].
impl From<crate::types::StoryAreaTypeMessage> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryAreaTypeLink`] into [`StoryAreaType`].
impl From<crate::types::StoryAreaTypeLink> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeLink) -> Self {
        Self::Link(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryAreaTypeWeather`] into [`StoryAreaType`].
impl From<crate::types::StoryAreaTypeWeather> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeWeather) -> Self {
        Self::Weather(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryAreaTypeUpgradedGift`] into [`StoryAreaType`].
impl From<crate::types::StoryAreaTypeUpgradedGift> for StoryAreaType {
    fn from(val: crate::types::StoryAreaTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }
}

/// TDLib `StoryArea` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryArea {
    /// Describes a clickable rectangle area on a story media
    #[serde(rename(serialize = "storyArea", deserialize = "storyArea"))]
    StoryArea(Box<crate::types::StoryArea>),
}

impl StoryArea {
    /// Convenience constructor to create a [`StoryArea::StoryArea`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_area(val: crate::types::StoryArea) -> Self {
        Self::StoryArea(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryArea`] into [`StoryArea`].
impl From<crate::types::StoryArea> for StoryArea {
    fn from(val: crate::types::StoryArea) -> Self {
        Self::StoryArea(Box::new(val))
    }
}

/// Describes type of clickable area on a story media to be added
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputStoryAreaType {
    /// An area pointing to a location
    #[serde(rename(serialize = "inputStoryAreaTypeLocation", deserialize = "inputStoryAreaTypeLocation"))]
    Location(Box<crate::types::InputStoryAreaTypeLocation>),
    /// An area pointing to a venue found by the bot getOption("venue_search_bot_username")
    #[serde(rename(serialize = "inputStoryAreaTypeFoundVenue", deserialize = "inputStoryAreaTypeFoundVenue"))]
    FoundVenue(Box<crate::types::InputStoryAreaTypeFoundVenue>),
    /// An area pointing to a venue already added to the story
    #[serde(rename(serialize = "inputStoryAreaTypePreviousVenue", deserialize = "inputStoryAreaTypePreviousVenue"))]
    PreviousVenue(Box<crate::types::InputStoryAreaTypePreviousVenue>),
    /// An area pointing to a suggested reaction
    #[serde(rename(serialize = "inputStoryAreaTypeSuggestedReaction", deserialize = "inputStoryAreaTypeSuggestedReaction"))]
    SuggestedReaction(Box<crate::types::InputStoryAreaTypeSuggestedReaction>),
    /// An area pointing to a message
    #[serde(rename(serialize = "inputStoryAreaTypeMessage", deserialize = "inputStoryAreaTypeMessage"))]
    Message(Box<crate::types::InputStoryAreaTypeMessage>),
    /// An area pointing to a HTTP or tg: link
    #[serde(rename(serialize = "inputStoryAreaTypeLink", deserialize = "inputStoryAreaTypeLink"))]
    Link(Box<crate::types::InputStoryAreaTypeLink>),
    /// An area with information about weather
    #[serde(rename(serialize = "inputStoryAreaTypeWeather", deserialize = "inputStoryAreaTypeWeather"))]
    Weather(Box<crate::types::InputStoryAreaTypeWeather>),
    /// An area with an upgraded gift
    #[serde(rename(serialize = "inputStoryAreaTypeUpgradedGift", deserialize = "inputStoryAreaTypeUpgradedGift"))]
    UpgradedGift(Box<crate::types::InputStoryAreaTypeUpgradedGift>),
}

impl InputStoryAreaType {
    /// Convenience constructor to create a [`InputStoryAreaType::Location`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn location(val: crate::types::InputStoryAreaTypeLocation) -> Self {
        Self::Location(Box::new(val))
    }

    /// Convenience constructor to create a [`InputStoryAreaType::FoundVenue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_venue(val: crate::types::InputStoryAreaTypeFoundVenue) -> Self {
        Self::FoundVenue(Box::new(val))
    }

    /// Convenience constructor to create a [`InputStoryAreaType::PreviousVenue`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn previous_venue(val: crate::types::InputStoryAreaTypePreviousVenue) -> Self {
        Self::PreviousVenue(Box::new(val))
    }

    /// Convenience constructor to create a [`InputStoryAreaType::SuggestedReaction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn suggested_reaction(val: crate::types::InputStoryAreaTypeSuggestedReaction) -> Self {
        Self::SuggestedReaction(Box::new(val))
    }

    /// Convenience constructor to create a [`InputStoryAreaType::Message`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn message(val: crate::types::InputStoryAreaTypeMessage) -> Self {
        Self::Message(Box::new(val))
    }

    /// Convenience constructor to create a [`InputStoryAreaType::Link`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn link(val: crate::types::InputStoryAreaTypeLink) -> Self {
        Self::Link(Box::new(val))
    }

    /// Convenience constructor to create a [`InputStoryAreaType::Weather`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn weather(val: crate::types::InputStoryAreaTypeWeather) -> Self {
        Self::Weather(Box::new(val))
    }

    /// Convenience constructor to create a [`InputStoryAreaType::UpgradedGift`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn upgraded_gift(val: crate::types::InputStoryAreaTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }

}

/// Converts a [`crate::types::InputStoryAreaTypeLocation`] into [`InputStoryAreaType`].
impl From<crate::types::InputStoryAreaTypeLocation> for InputStoryAreaType {
    fn from(val: crate::types::InputStoryAreaTypeLocation) -> Self {
        Self::Location(Box::new(val))
    }
}

/// Converts a [`crate::types::InputStoryAreaTypeFoundVenue`] into [`InputStoryAreaType`].
impl From<crate::types::InputStoryAreaTypeFoundVenue> for InputStoryAreaType {
    fn from(val: crate::types::InputStoryAreaTypeFoundVenue) -> Self {
        Self::FoundVenue(Box::new(val))
    }
}

/// Converts a [`crate::types::InputStoryAreaTypePreviousVenue`] into [`InputStoryAreaType`].
impl From<crate::types::InputStoryAreaTypePreviousVenue> for InputStoryAreaType {
    fn from(val: crate::types::InputStoryAreaTypePreviousVenue) -> Self {
        Self::PreviousVenue(Box::new(val))
    }
}

/// Converts a [`crate::types::InputStoryAreaTypeSuggestedReaction`] into [`InputStoryAreaType`].
impl From<crate::types::InputStoryAreaTypeSuggestedReaction> for InputStoryAreaType {
    fn from(val: crate::types::InputStoryAreaTypeSuggestedReaction) -> Self {
        Self::SuggestedReaction(Box::new(val))
    }
}

/// Converts a [`crate::types::InputStoryAreaTypeMessage`] into [`InputStoryAreaType`].
impl From<crate::types::InputStoryAreaTypeMessage> for InputStoryAreaType {
    fn from(val: crate::types::InputStoryAreaTypeMessage) -> Self {
        Self::Message(Box::new(val))
    }
}

/// Converts a [`crate::types::InputStoryAreaTypeLink`] into [`InputStoryAreaType`].
impl From<crate::types::InputStoryAreaTypeLink> for InputStoryAreaType {
    fn from(val: crate::types::InputStoryAreaTypeLink) -> Self {
        Self::Link(Box::new(val))
    }
}

/// Converts a [`crate::types::InputStoryAreaTypeWeather`] into [`InputStoryAreaType`].
impl From<crate::types::InputStoryAreaTypeWeather> for InputStoryAreaType {
    fn from(val: crate::types::InputStoryAreaTypeWeather) -> Self {
        Self::Weather(Box::new(val))
    }
}

/// Converts a [`crate::types::InputStoryAreaTypeUpgradedGift`] into [`InputStoryAreaType`].
impl From<crate::types::InputStoryAreaTypeUpgradedGift> for InputStoryAreaType {
    fn from(val: crate::types::InputStoryAreaTypeUpgradedGift) -> Self {
        Self::UpgradedGift(Box::new(val))
    }
}

/// TDLib `InputStoryArea` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputStoryArea {
    /// Describes a clickable rectangle area on a story media to be added
    #[serde(rename(serialize = "inputStoryArea", deserialize = "inputStoryArea"))]
    InputStoryArea(Box<crate::types::InputStoryArea>),
}

impl InputStoryArea {
    /// Convenience constructor to create a [`InputStoryArea::InputStoryArea`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_story_area(val: crate::types::InputStoryArea) -> Self {
        Self::InputStoryArea(Box::new(val))
    }

}

/// Converts a [`crate::types::InputStoryArea`] into [`InputStoryArea`].
impl From<crate::types::InputStoryArea> for InputStoryArea {
    fn from(val: crate::types::InputStoryArea) -> Self {
        Self::InputStoryArea(Box::new(val))
    }
}

/// TDLib `InputStoryAreas` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputStoryAreas {
    /// Contains a list of story areas to be added
    #[serde(rename(serialize = "inputStoryAreas", deserialize = "inputStoryAreas"))]
    InputStoryAreas(Box<crate::types::InputStoryAreas>),
}

impl InputStoryAreas {
    /// Convenience constructor to create a [`InputStoryAreas::InputStoryAreas`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_story_areas(val: crate::types::InputStoryAreas) -> Self {
        Self::InputStoryAreas(Box::new(val))
    }

}

/// Converts a [`crate::types::InputStoryAreas`] into [`InputStoryAreas`].
impl From<crate::types::InputStoryAreas> for InputStoryAreas {
    fn from(val: crate::types::InputStoryAreas) -> Self {
        Self::InputStoryAreas(Box::new(val))
    }
}

/// TDLib `StoryVideo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryVideo {
    /// Describes a video file posted as a story
    #[serde(rename(serialize = "storyVideo", deserialize = "storyVideo"))]
    StoryVideo(Box<crate::types::StoryVideo>),
}

impl StoryVideo {
    /// Convenience constructor to create a [`StoryVideo::StoryVideo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_video(val: crate::types::StoryVideo) -> Self {
        Self::StoryVideo(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryVideo`] into [`StoryVideo`].
impl From<crate::types::StoryVideo> for StoryVideo {
    fn from(val: crate::types::StoryVideo) -> Self {
        Self::StoryVideo(Box::new(val))
    }
}

/// Contains the type of the content of a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryContentType {
    /// A photo story
    #[serde(rename(serialize = "storyContentTypePhoto", deserialize = "storyContentTypePhoto"))]
    Photo,
    /// A video story
    #[serde(rename(serialize = "storyContentTypeVideo", deserialize = "storyContentTypeVideo"))]
    Video,
    /// A live story
    #[serde(rename(serialize = "storyContentTypeLive", deserialize = "storyContentTypeLive"))]
    Live,
    /// A story of unknown content type
    #[serde(rename(serialize = "storyContentTypeUnsupported", deserialize = "storyContentTypeUnsupported"))]
    Unsupported,
}

/// Contains the content of a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryContent {
    /// A photo story
    #[serde(rename(serialize = "storyContentPhoto", deserialize = "storyContentPhoto"))]
    Photo(Box<crate::types::StoryContentPhoto>),
    /// A video story
    #[serde(rename(serialize = "storyContentVideo", deserialize = "storyContentVideo"))]
    Video(Box<crate::types::StoryContentVideo>),
    /// A live story
    #[serde(rename(serialize = "storyContentLive", deserialize = "storyContentLive"))]
    Live(Box<crate::types::StoryContentLive>),
    /// A story content that is not supported in the current TDLib version
    #[serde(rename(serialize = "storyContentUnsupported", deserialize = "storyContentUnsupported"))]
    Unsupported,
}

impl StoryContent {
    /// Convenience constructor to create a [`StoryContent::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::StoryContentPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryContent::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::StoryContentVideo) -> Self {
        Self::Video(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryContent::Live`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn live(val: crate::types::StoryContentLive) -> Self {
        Self::Live(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryContentPhoto`] into [`StoryContent`].
impl From<crate::types::StoryContentPhoto> for StoryContent {
    fn from(val: crate::types::StoryContentPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryContentVideo`] into [`StoryContent`].
impl From<crate::types::StoryContentVideo> for StoryContent {
    fn from(val: crate::types::StoryContentVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryContentLive`] into [`StoryContent`].
impl From<crate::types::StoryContentLive> for StoryContent {
    fn from(val: crate::types::StoryContentLive) -> Self {
        Self::Live(Box::new(val))
    }
}

/// The content of a story to post
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputStoryContent {
    /// A photo story
    #[serde(rename(serialize = "inputStoryContentPhoto", deserialize = "inputStoryContentPhoto"))]
    Photo(Box<crate::types::InputStoryContentPhoto>),
    /// A video story
    #[serde(rename(serialize = "inputStoryContentVideo", deserialize = "inputStoryContentVideo"))]
    Video(Box<crate::types::InputStoryContentVideo>),
}

impl InputStoryContent {
    /// Convenience constructor to create a [`InputStoryContent::Photo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn photo(val: crate::types::InputStoryContentPhoto) -> Self {
        Self::Photo(Box::new(val))
    }

    /// Convenience constructor to create a [`InputStoryContent::Video`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn video(val: crate::types::InputStoryContentVideo) -> Self {
        Self::Video(Box::new(val))
    }

}

/// Converts a [`crate::types::InputStoryContentPhoto`] into [`InputStoryContent`].
impl From<crate::types::InputStoryContentPhoto> for InputStoryContent {
    fn from(val: crate::types::InputStoryContentPhoto) -> Self {
        Self::Photo(Box::new(val))
    }
}

/// Converts a [`crate::types::InputStoryContentVideo`] into [`InputStoryContent`].
impl From<crate::types::InputStoryContentVideo> for InputStoryContent {
    fn from(val: crate::types::InputStoryContentVideo) -> Self {
        Self::Video(Box::new(val))
    }
}

/// Describes a list of stories
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryList {
    /// The list of stories, shown in the main chat list and folder chat lists
    #[serde(rename(serialize = "storyListMain", deserialize = "storyListMain"))]
    Main,
    /// The list of stories, shown in the Archive chat list
    #[serde(rename(serialize = "storyListArchive", deserialize = "storyListArchive"))]
    Archive,
}

/// Contains information about the origin of a story that was reposted
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryOrigin {
    /// The original story was a public story that was posted by a known chat
    #[serde(rename(serialize = "storyOriginPublicStory", deserialize = "storyOriginPublicStory"))]
    PublicStory(Box<crate::types::StoryOriginPublicStory>),
    /// The original story was posted by an unknown user
    #[serde(rename(serialize = "storyOriginHiddenUser", deserialize = "storyOriginHiddenUser"))]
    HiddenUser(Box<crate::types::StoryOriginHiddenUser>),
}

impl StoryOrigin {
    /// Convenience constructor to create a [`StoryOrigin::PublicStory`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn public_story(val: crate::types::StoryOriginPublicStory) -> Self {
        Self::PublicStory(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryOrigin::HiddenUser`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn hidden_user(val: crate::types::StoryOriginHiddenUser) -> Self {
        Self::HiddenUser(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryOriginPublicStory`] into [`StoryOrigin`].
impl From<crate::types::StoryOriginPublicStory> for StoryOrigin {
    fn from(val: crate::types::StoryOriginPublicStory) -> Self {
        Self::PublicStory(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryOriginHiddenUser`] into [`StoryOrigin`].
impl From<crate::types::StoryOriginHiddenUser> for StoryOrigin {
    fn from(val: crate::types::StoryOriginHiddenUser) -> Self {
        Self::HiddenUser(Box::new(val))
    }
}

/// TDLib `StoryRepostInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryRepostInfo {
    /// Contains information about original story that was reposted
    #[serde(rename(serialize = "storyRepostInfo", deserialize = "storyRepostInfo"))]
    StoryRepostInfo(Box<crate::types::StoryRepostInfo>),
}

impl StoryRepostInfo {
    /// Convenience constructor to create a [`StoryRepostInfo::StoryRepostInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_repost_info(val: crate::types::StoryRepostInfo) -> Self {
        Self::StoryRepostInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryRepostInfo`] into [`StoryRepostInfo`].
impl From<crate::types::StoryRepostInfo> for StoryRepostInfo {
    fn from(val: crate::types::StoryRepostInfo) -> Self {
        Self::StoryRepostInfo(Box::new(val))
    }
}

/// TDLib `StoryInteractionInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInteractionInfo {
    /// Contains information about interactions with a story
    #[serde(rename(serialize = "storyInteractionInfo", deserialize = "storyInteractionInfo"))]
    StoryInteractionInfo(Box<crate::types::StoryInteractionInfo>),
}

impl StoryInteractionInfo {
    /// Convenience constructor to create a [`StoryInteractionInfo::StoryInteractionInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_interaction_info(val: crate::types::StoryInteractionInfo) -> Self {
        Self::StoryInteractionInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryInteractionInfo`] into [`StoryInteractionInfo`].
impl From<crate::types::StoryInteractionInfo> for StoryInteractionInfo {
    fn from(val: crate::types::StoryInteractionInfo) -> Self {
        Self::StoryInteractionInfo(Box::new(val))
    }
}

/// TDLib `Story` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Story {
    /// Represents a story
    #[serde(rename(serialize = "story", deserialize = "story"))]
    Story(Box<crate::types::Story>),
}

impl Story {
    /// Convenience constructor to create a [`Story::Story`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story(val: crate::types::Story) -> Self {
        Self::Story(Box::new(val))
    }

}

/// Converts a [`crate::types::Story`] into [`Story`].
impl From<crate::types::Story> for Story {
    fn from(val: crate::types::Story) -> Self {
        Self::Story(Box::new(val))
    }
}

/// TDLib `Stories` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Stories {
    /// Represents a list of stories
    #[serde(rename(serialize = "stories", deserialize = "stories"))]
    Stories(Box<crate::types::Stories>),
}

impl Stories {
    /// Convenience constructor to create a [`Stories::Stories`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn stories(val: crate::types::Stories) -> Self {
        Self::Stories(Box::new(val))
    }

}

/// Converts a [`crate::types::Stories`] into [`Stories`].
impl From<crate::types::Stories> for Stories {
    fn from(val: crate::types::Stories) -> Self {
        Self::Stories(Box::new(val))
    }
}

/// TDLib `FoundStories` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundStories {
    /// Contains a list of stories found by a search
    #[serde(rename(serialize = "foundStories", deserialize = "foundStories"))]
    FoundStories(Box<crate::types::FoundStories>),
}

impl FoundStories {
    /// Convenience constructor to create a [`FoundStories::FoundStories`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_stories(val: crate::types::FoundStories) -> Self {
        Self::FoundStories(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundStories`] into [`FoundStories`].
impl From<crate::types::FoundStories> for FoundStories {
    fn from(val: crate::types::FoundStories) -> Self {
        Self::FoundStories(Box::new(val))
    }
}

/// TDLib `StoryAlbum` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryAlbum {
    /// Describes album of stories
    #[serde(rename(serialize = "storyAlbum", deserialize = "storyAlbum"))]
    StoryAlbum(Box<crate::types::StoryAlbum>),
}

impl StoryAlbum {
    /// Convenience constructor to create a [`StoryAlbum::StoryAlbum`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_album(val: crate::types::StoryAlbum) -> Self {
        Self::StoryAlbum(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryAlbum`] into [`StoryAlbum`].
impl From<crate::types::StoryAlbum> for StoryAlbum {
    fn from(val: crate::types::StoryAlbum) -> Self {
        Self::StoryAlbum(Box::new(val))
    }
}

/// TDLib `StoryAlbums` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryAlbums {
    /// Represents a list of story albums
    #[serde(rename(serialize = "storyAlbums", deserialize = "storyAlbums"))]
    StoryAlbums(Box<crate::types::StoryAlbums>),
}

impl StoryAlbums {
    /// Convenience constructor to create a [`StoryAlbums::StoryAlbums`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_albums(val: crate::types::StoryAlbums) -> Self {
        Self::StoryAlbums(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryAlbums`] into [`StoryAlbums`].
impl From<crate::types::StoryAlbums> for StoryAlbums {
    fn from(val: crate::types::StoryAlbums) -> Self {
        Self::StoryAlbums(Box::new(val))
    }
}

/// TDLib `StoryFullId` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryFullId {
    /// Contains identifier of a story along with identifier of the chat that posted it
    #[serde(rename(serialize = "storyFullId", deserialize = "storyFullId"))]
    StoryFullId(Box<crate::types::StoryFullId>),
}

impl StoryFullId {
    /// Convenience constructor to create a [`StoryFullId::StoryFullId`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_full_id(val: crate::types::StoryFullId) -> Self {
        Self::StoryFullId(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryFullId`] into [`StoryFullId`].
impl From<crate::types::StoryFullId> for StoryFullId {
    fn from(val: crate::types::StoryFullId) -> Self {
        Self::StoryFullId(Box::new(val))
    }
}

/// TDLib `StoryInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInfo {
    /// Contains basic information about a story
    #[serde(rename(serialize = "storyInfo", deserialize = "storyInfo"))]
    StoryInfo(Box<crate::types::StoryInfo>),
}

impl StoryInfo {
    /// Convenience constructor to create a [`StoryInfo::StoryInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_info(val: crate::types::StoryInfo) -> Self {
        Self::StoryInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryInfo`] into [`StoryInfo`].
impl From<crate::types::StoryInfo> for StoryInfo {
    fn from(val: crate::types::StoryInfo) -> Self {
        Self::StoryInfo(Box::new(val))
    }
}

/// TDLib `ChatActiveStories` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatActiveStories {
    /// Describes active stories posted by a chat
    #[serde(rename(serialize = "chatActiveStories", deserialize = "chatActiveStories"))]
    ChatActiveStories(Box<crate::types::ChatActiveStories>),
}

impl ChatActiveStories {
    /// Convenience constructor to create a [`ChatActiveStories::ChatActiveStories`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn chat_active_stories(val: crate::types::ChatActiveStories) -> Self {
        Self::ChatActiveStories(Box::new(val))
    }

}

/// Converts a [`crate::types::ChatActiveStories`] into [`ChatActiveStories`].
impl From<crate::types::ChatActiveStories> for ChatActiveStories {
    fn from(val: crate::types::ChatActiveStories) -> Self {
        Self::ChatActiveStories(Box::new(val))
    }
}

/// Describes type of interaction with a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInteractionType {
    /// A view of the story
    #[serde(rename(serialize = "storyInteractionTypeView", deserialize = "storyInteractionTypeView"))]
    View(Box<crate::types::StoryInteractionTypeView>),
    /// A forward of the story as a message
    #[serde(rename(serialize = "storyInteractionTypeForward", deserialize = "storyInteractionTypeForward"))]
    Forward(Box<crate::types::StoryInteractionTypeForward>),
    /// A repost of the story as a story
    #[serde(rename(serialize = "storyInteractionTypeRepost", deserialize = "storyInteractionTypeRepost"))]
    Repost(Box<crate::types::StoryInteractionTypeRepost>),
}

impl StoryInteractionType {
    /// Convenience constructor to create a [`StoryInteractionType::View`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn view(val: crate::types::StoryInteractionTypeView) -> Self {
        Self::View(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryInteractionType::Forward`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn forward(val: crate::types::StoryInteractionTypeForward) -> Self {
        Self::Forward(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryInteractionType::Repost`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn repost(val: crate::types::StoryInteractionTypeRepost) -> Self {
        Self::Repost(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryInteractionTypeView`] into [`StoryInteractionType`].
impl From<crate::types::StoryInteractionTypeView> for StoryInteractionType {
    fn from(val: crate::types::StoryInteractionTypeView) -> Self {
        Self::View(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryInteractionTypeForward`] into [`StoryInteractionType`].
impl From<crate::types::StoryInteractionTypeForward> for StoryInteractionType {
    fn from(val: crate::types::StoryInteractionTypeForward) -> Self {
        Self::Forward(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryInteractionTypeRepost`] into [`StoryInteractionType`].
impl From<crate::types::StoryInteractionTypeRepost> for StoryInteractionType {
    fn from(val: crate::types::StoryInteractionTypeRepost) -> Self {
        Self::Repost(Box::new(val))
    }
}

/// TDLib `StoryInteraction` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInteraction {
    /// Represents interaction with a story
    #[serde(rename(serialize = "storyInteraction", deserialize = "storyInteraction"))]
    StoryInteraction(Box<crate::types::StoryInteraction>),
}

impl StoryInteraction {
    /// Convenience constructor to create a [`StoryInteraction::StoryInteraction`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_interaction(val: crate::types::StoryInteraction) -> Self {
        Self::StoryInteraction(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryInteraction`] into [`StoryInteraction`].
impl From<crate::types::StoryInteraction> for StoryInteraction {
    fn from(val: crate::types::StoryInteraction) -> Self {
        Self::StoryInteraction(Box::new(val))
    }
}

/// TDLib `StoryInteractions` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryInteractions {
    /// Represents a list of interactions with a story
    #[serde(rename(serialize = "storyInteractions", deserialize = "storyInteractions"))]
    StoryInteractions(Box<crate::types::StoryInteractions>),
}

impl StoryInteractions {
    /// Convenience constructor to create a [`StoryInteractions::StoryInteractions`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_interactions(val: crate::types::StoryInteractions) -> Self {
        Self::StoryInteractions(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryInteractions`] into [`StoryInteractions`].
impl From<crate::types::StoryInteractions> for StoryInteractions {
    fn from(val: crate::types::StoryInteractions) -> Self {
        Self::StoryInteractions(Box::new(val))
    }
}

/// Describes a story feature available to Premium users
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumStoryFeature {
    /// Stories of the current user are displayed before stories of non-Premium contacts, supergroups, and channels
    #[serde(rename(serialize = "premiumStoryFeaturePriorityOrder", deserialize = "premiumStoryFeaturePriorityOrder"))]
    PriorityOrder,
    /// The ability to hide the fact that the user viewed other's stories
    #[serde(rename(serialize = "premiumStoryFeatureStealthMode", deserialize = "premiumStoryFeatureStealthMode"))]
    StealthMode,
    /// The ability to check who opened the current user's stories after they expire
    #[serde(rename(serialize = "premiumStoryFeaturePermanentViewsHistory", deserialize = "premiumStoryFeaturePermanentViewsHistory"))]
    PermanentViewsHistory,
    /// The ability to set custom expiration duration for stories
    #[serde(rename(serialize = "premiumStoryFeatureCustomExpirationDuration", deserialize = "premiumStoryFeatureCustomExpirationDuration"))]
    CustomExpirationDuration,
    /// The ability to save other's unprotected stories
    #[serde(rename(serialize = "premiumStoryFeatureSaveStories", deserialize = "premiumStoryFeatureSaveStories"))]
    SaveStories,
    /// The ability to use links and formatting in story caption, and use inputStoryAreaTypeLink areas
    #[serde(rename(serialize = "premiumStoryFeatureLinksAndFormatting", deserialize = "premiumStoryFeatureLinksAndFormatting"))]
    LinksAndFormatting,
    /// The ability to choose better quality for viewed stories
    #[serde(rename(serialize = "premiumStoryFeatureVideoQuality", deserialize = "premiumStoryFeatureVideoQuality"))]
    VideoQuality,
}

/// Represents result of checking whether the current user can post a story on behalf of the specific chat
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanPostStoryResult {
    /// A story can be sent
    #[serde(rename(serialize = "canPostStoryResultOk", deserialize = "canPostStoryResultOk"))]
    Ok(Box<crate::types::CanPostStoryResultOk>),
    /// The user must subscribe to Telegram Premium to be able to post stories
    #[serde(rename(serialize = "canPostStoryResultPremiumNeeded", deserialize = "canPostStoryResultPremiumNeeded"))]
    PremiumNeeded,
    /// The chat must be boosted first by Telegram Premium subscribers to post more stories. Call getChatBoostStatus to get current boost status of the chat
    #[serde(rename(serialize = "canPostStoryResultBoostNeeded", deserialize = "canPostStoryResultBoostNeeded"))]
    BoostNeeded,
    /// The limit for the number of active stories exceeded. The user can buy Telegram Premium, delete an active story, or wait for the oldest story to expire
    #[serde(rename(serialize = "canPostStoryResultActiveStoryLimitExceeded", deserialize = "canPostStoryResultActiveStoryLimitExceeded"))]
    ActiveStoryLimitExceeded,
    /// The weekly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
    #[serde(rename(serialize = "canPostStoryResultWeeklyLimitExceeded", deserialize = "canPostStoryResultWeeklyLimitExceeded"))]
    WeeklyLimitExceeded(Box<crate::types::CanPostStoryResultWeeklyLimitExceeded>),
    /// The monthly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
    #[serde(rename(serialize = "canPostStoryResultMonthlyLimitExceeded", deserialize = "canPostStoryResultMonthlyLimitExceeded"))]
    MonthlyLimitExceeded(Box<crate::types::CanPostStoryResultMonthlyLimitExceeded>),
    /// The user or the chat has an active live story. The live story must be deleted first
    #[serde(rename(serialize = "canPostStoryResultLiveStoryIsActive", deserialize = "canPostStoryResultLiveStoryIsActive"))]
    LiveStoryIsActive(Box<crate::types::CanPostStoryResultLiveStoryIsActive>),
}

impl CanPostStoryResult {
    /// Convenience constructor to create a [`CanPostStoryResult::Ok`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ok(val: crate::types::CanPostStoryResultOk) -> Self {
        Self::Ok(Box::new(val))
    }

    /// Convenience constructor to create a [`CanPostStoryResult::WeeklyLimitExceeded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn weekly_limit_exceeded(val: crate::types::CanPostStoryResultWeeklyLimitExceeded) -> Self {
        Self::WeeklyLimitExceeded(Box::new(val))
    }

    /// Convenience constructor to create a [`CanPostStoryResult::MonthlyLimitExceeded`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn monthly_limit_exceeded(val: crate::types::CanPostStoryResultMonthlyLimitExceeded) -> Self {
        Self::MonthlyLimitExceeded(Box::new(val))
    }

    /// Convenience constructor to create a [`CanPostStoryResult::LiveStoryIsActive`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn live_story_is_active(val: crate::types::CanPostStoryResultLiveStoryIsActive) -> Self {
        Self::LiveStoryIsActive(Box::new(val))
    }

}

/// Converts a [`crate::types::CanPostStoryResultOk`] into [`CanPostStoryResult`].
impl From<crate::types::CanPostStoryResultOk> for CanPostStoryResult {
    fn from(val: crate::types::CanPostStoryResultOk) -> Self {
        Self::Ok(Box::new(val))
    }
}

/// Converts a [`crate::types::CanPostStoryResultWeeklyLimitExceeded`] into [`CanPostStoryResult`].
impl From<crate::types::CanPostStoryResultWeeklyLimitExceeded> for CanPostStoryResult {
    fn from(val: crate::types::CanPostStoryResultWeeklyLimitExceeded) -> Self {
        Self::WeeklyLimitExceeded(Box::new(val))
    }
}

/// Converts a [`crate::types::CanPostStoryResultMonthlyLimitExceeded`] into [`CanPostStoryResult`].
impl From<crate::types::CanPostStoryResultMonthlyLimitExceeded> for CanPostStoryResult {
    fn from(val: crate::types::CanPostStoryResultMonthlyLimitExceeded) -> Self {
        Self::MonthlyLimitExceeded(Box::new(val))
    }
}

/// Converts a [`crate::types::CanPostStoryResultLiveStoryIsActive`] into [`CanPostStoryResult`].
impl From<crate::types::CanPostStoryResultLiveStoryIsActive> for CanPostStoryResult {
    fn from(val: crate::types::CanPostStoryResultLiveStoryIsActive) -> Self {
        Self::LiveStoryIsActive(Box::new(val))
    }
}

/// Represents result of starting a live story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StartLiveStoryResult {
    /// The live story was successfully posted
    #[serde(rename(serialize = "startLiveStoryResultOk", deserialize = "startLiveStoryResultOk"))]
    Ok(Box<crate::types::StartLiveStoryResultOk>),
    /// The live story failed to post with an error to be handled
    #[serde(rename(serialize = "startLiveStoryResultFail", deserialize = "startLiveStoryResultFail"))]
    Fail(Box<crate::types::StartLiveStoryResultFail>),
}

impl StartLiveStoryResult {
    /// Convenience constructor to create a [`StartLiveStoryResult::Ok`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn ok(val: crate::types::StartLiveStoryResultOk) -> Self {
        Self::Ok(Box::new(val))
    }

    /// Convenience constructor to create a [`StartLiveStoryResult::Fail`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn fail(val: crate::types::StartLiveStoryResultFail) -> Self {
        Self::Fail(Box::new(val))
    }

}

/// Converts a [`crate::types::StartLiveStoryResultOk`] into [`StartLiveStoryResult`].
impl From<crate::types::StartLiveStoryResultOk> for StartLiveStoryResult {
    fn from(val: crate::types::StartLiveStoryResultOk) -> Self {
        Self::Ok(Box::new(val))
    }
}

/// Converts a [`crate::types::StartLiveStoryResultFail`] into [`StartLiveStoryResult`].
impl From<crate::types::StartLiveStoryResultFail> for StartLiveStoryResult {
    fn from(val: crate::types::StartLiveStoryResultFail) -> Self {
        Self::Fail(Box::new(val))
    }
}

/// Describes privacy settings of a story
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryPrivacySettings {
    /// The story can be viewed by everyone
    #[serde(rename(serialize = "storyPrivacySettingsEveryone", deserialize = "storyPrivacySettingsEveryone"))]
    Everyone(Box<crate::types::StoryPrivacySettingsEveryone>),
    /// The story can be viewed by all contacts except chosen users
    #[serde(rename(serialize = "storyPrivacySettingsContacts", deserialize = "storyPrivacySettingsContacts"))]
    Contacts(Box<crate::types::StoryPrivacySettingsContacts>),
    /// The story can be viewed by all close friends
    #[serde(rename(serialize = "storyPrivacySettingsCloseFriends", deserialize = "storyPrivacySettingsCloseFriends"))]
    CloseFriends,
    /// The story can be viewed by certain specified users
    #[serde(rename(serialize = "storyPrivacySettingsSelectedUsers", deserialize = "storyPrivacySettingsSelectedUsers"))]
    SelectedUsers(Box<crate::types::StoryPrivacySettingsSelectedUsers>),
}

impl StoryPrivacySettings {
    /// Convenience constructor to create a [`StoryPrivacySettings::Everyone`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn everyone(val: crate::types::StoryPrivacySettingsEveryone) -> Self {
        Self::Everyone(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryPrivacySettings::Contacts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contacts(val: crate::types::StoryPrivacySettingsContacts) -> Self {
        Self::Contacts(Box::new(val))
    }

    /// Convenience constructor to create a [`StoryPrivacySettings::SelectedUsers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn selected_users(val: crate::types::StoryPrivacySettingsSelectedUsers) -> Self {
        Self::SelectedUsers(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryPrivacySettingsEveryone`] into [`StoryPrivacySettings`].
impl From<crate::types::StoryPrivacySettingsEveryone> for StoryPrivacySettings {
    fn from(val: crate::types::StoryPrivacySettingsEveryone) -> Self {
        Self::Everyone(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryPrivacySettingsContacts`] into [`StoryPrivacySettings`].
impl From<crate::types::StoryPrivacySettingsContacts> for StoryPrivacySettings {
    fn from(val: crate::types::StoryPrivacySettingsContacts) -> Self {
        Self::Contacts(Box::new(val))
    }
}

/// Converts a [`crate::types::StoryPrivacySettingsSelectedUsers`] into [`StoryPrivacySettings`].
impl From<crate::types::StoryPrivacySettingsSelectedUsers> for StoryPrivacySettings {
    fn from(val: crate::types::StoryPrivacySettingsSelectedUsers) -> Self {
        Self::SelectedUsers(Box::new(val))
    }
}

/// Describes result of story report
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportStoryResult {
    /// The story was reported successfully
    #[serde(rename(serialize = "reportStoryResultOk", deserialize = "reportStoryResultOk"))]
    Ok,
    /// The user must choose an option to report the story and repeat request with the chosen option
    #[serde(rename(serialize = "reportStoryResultOptionRequired", deserialize = "reportStoryResultOptionRequired"))]
    OptionRequired(Box<crate::types::ReportStoryResultOptionRequired>),
    /// The user must add additional text details to the report
    #[serde(rename(serialize = "reportStoryResultTextRequired", deserialize = "reportStoryResultTextRequired"))]
    TextRequired(Box<crate::types::ReportStoryResultTextRequired>),
}

impl ReportStoryResult {
    /// Convenience constructor to create a [`ReportStoryResult::OptionRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn option_required(val: crate::types::ReportStoryResultOptionRequired) -> Self {
        Self::OptionRequired(Box::new(val))
    }

    /// Convenience constructor to create a [`ReportStoryResult::TextRequired`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn text_required(val: crate::types::ReportStoryResultTextRequired) -> Self {
        Self::TextRequired(Box::new(val))
    }

}

/// Converts a [`crate::types::ReportStoryResultOptionRequired`] into [`ReportStoryResult`].
impl From<crate::types::ReportStoryResultOptionRequired> for ReportStoryResult {
    fn from(val: crate::types::ReportStoryResultOptionRequired) -> Self {
        Self::OptionRequired(Box::new(val))
    }
}

/// Converts a [`crate::types::ReportStoryResultTextRequired`] into [`ReportStoryResult`].
impl From<crate::types::ReportStoryResultTextRequired> for ReportStoryResult {
    fn from(val: crate::types::ReportStoryResultTextRequired) -> Self {
        Self::TextRequired(Box::new(val))
    }
}

/// TDLib `StoryStatistics` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryStatistics {
    /// A detailed statistics about a story
    #[serde(rename(serialize = "storyStatistics", deserialize = "storyStatistics"))]
    StoryStatistics(Box<crate::types::StoryStatistics>),
}

impl StoryStatistics {
    /// Convenience constructor to create a [`StoryStatistics::StoryStatistics`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn story_statistics(val: crate::types::StoryStatistics) -> Self {
        Self::StoryStatistics(Box::new(val))
    }

}

/// Converts a [`crate::types::StoryStatistics`] into [`StoryStatistics`].
impl From<crate::types::StoryStatistics> for StoryStatistics {
    fn from(val: crate::types::StoryStatistics) -> Self {
        Self::StoryStatistics(Box::new(val))
    }
}

