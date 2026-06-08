#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputStoryAreaType {
    /// An area pointing to a location
    #[serde(rename(
        serialize = "inputStoryAreaTypeLocation",
        deserialize = "inputStoryAreaTypeLocation"
    ))]
    Location(crate::types::InputStoryAreaTypeLocation),
    /// An area pointing to a venue found by the bot getOption("venue_search_bot_username")
    #[serde(rename(
        serialize = "inputStoryAreaTypeFoundVenue",
        deserialize = "inputStoryAreaTypeFoundVenue"
    ))]
    FoundVenue(crate::types::InputStoryAreaTypeFoundVenue),
    /// An area pointing to a venue already added to the story
    #[serde(rename(
        serialize = "inputStoryAreaTypePreviousVenue",
        deserialize = "inputStoryAreaTypePreviousVenue"
    ))]
    PreviousVenue(crate::types::InputStoryAreaTypePreviousVenue),
    /// An area pointing to a suggested reaction
    #[serde(rename(
        serialize = "inputStoryAreaTypeSuggestedReaction",
        deserialize = "inputStoryAreaTypeSuggestedReaction"
    ))]
    SuggestedReaction(crate::types::InputStoryAreaTypeSuggestedReaction),
    /// An area pointing to a message
    #[serde(rename(
        serialize = "inputStoryAreaTypeMessage",
        deserialize = "inputStoryAreaTypeMessage"
    ))]
    Message(crate::types::InputStoryAreaTypeMessage),
    /// An area pointing to a HTTP or tg: link
    #[serde(rename(
        serialize = "inputStoryAreaTypeLink",
        deserialize = "inputStoryAreaTypeLink"
    ))]
    Link(crate::types::InputStoryAreaTypeLink),
    /// An area with information about weather
    #[serde(rename(
        serialize = "inputStoryAreaTypeWeather",
        deserialize = "inputStoryAreaTypeWeather"
    ))]
    Weather(crate::types::InputStoryAreaTypeWeather),
    /// An area with an upgraded gift
    #[serde(rename(
        serialize = "inputStoryAreaTypeUpgradedGift",
        deserialize = "inputStoryAreaTypeUpgradedGift"
    ))]
    UpgradedGift(crate::types::InputStoryAreaTypeUpgradedGift),
}
