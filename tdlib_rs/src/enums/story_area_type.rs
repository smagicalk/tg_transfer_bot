#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StoryAreaType {
    /// An area pointing to a location
    #[serde(rename(
        serialize = "storyAreaTypeLocation",
        deserialize = "storyAreaTypeLocation"
    ))]
    Location(crate::types::StoryAreaTypeLocation),
    /// An area pointing to a venue
    #[serde(rename(serialize = "storyAreaTypeVenue", deserialize = "storyAreaTypeVenue"))]
    Venue(crate::types::StoryAreaTypeVenue),
    /// An area pointing to a suggested reaction. App needs to show a clickable reaction on the area and call setStoryReaction when the are is clicked
    #[serde(rename(
        serialize = "storyAreaTypeSuggestedReaction",
        deserialize = "storyAreaTypeSuggestedReaction"
    ))]
    SuggestedReaction(crate::types::StoryAreaTypeSuggestedReaction),
    /// An area pointing to a message
    #[serde(rename(
        serialize = "storyAreaTypeMessage",
        deserialize = "storyAreaTypeMessage"
    ))]
    Message(crate::types::StoryAreaTypeMessage),
    /// An area pointing to a HTTP or tg: link
    #[serde(rename(serialize = "storyAreaTypeLink", deserialize = "storyAreaTypeLink"))]
    Link(crate::types::StoryAreaTypeLink),
    /// An area with information about weather
    #[serde(rename(
        serialize = "storyAreaTypeWeather",
        deserialize = "storyAreaTypeWeather"
    ))]
    Weather(crate::types::StoryAreaTypeWeather),
    /// An area with an upgraded gift
    #[serde(rename(
        serialize = "storyAreaTypeUpgradedGift",
        deserialize = "storyAreaTypeUpgradedGift"
    ))]
    UpgradedGift(crate::types::StoryAreaTypeUpgradedGift),
}
