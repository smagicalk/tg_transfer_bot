#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessFeature {
    /// The ability to set location
    #[serde(rename(serialize = "businessFeatureLocation", deserialize = "businessFeatureLocation"))]
    Location,
    /// The ability to set opening hours
    #[serde(rename(serialize = "businessFeatureOpeningHours", deserialize = "businessFeatureOpeningHours"))]
    OpeningHours,
    /// The ability to use quick replies
    #[serde(rename(serialize = "businessFeatureQuickReplies", deserialize = "businessFeatureQuickReplies"))]
    QuickReplies,
    /// The ability to set up a greeting message
    #[serde(rename(serialize = "businessFeatureGreetingMessage", deserialize = "businessFeatureGreetingMessage"))]
    GreetingMessage,
    /// The ability to set up an away message
    #[serde(rename(serialize = "businessFeatureAwayMessage", deserialize = "businessFeatureAwayMessage"))]
    AwayMessage,
    /// The ability to create links to the business account with predefined message text
    #[serde(rename(serialize = "businessFeatureAccountLinks", deserialize = "businessFeatureAccountLinks"))]
    AccountLinks,
    /// The ability to customize start page
    #[serde(rename(serialize = "businessFeatureStartPage", deserialize = "businessFeatureStartPage"))]
    StartPage,
    /// The ability to connect a bot to the account
    #[serde(rename(serialize = "businessFeatureBots", deserialize = "businessFeatureBots"))]
    Bots,
    /// The ability to show an emoji status along with the business name
    #[serde(rename(serialize = "businessFeatureEmojiStatus", deserialize = "businessFeatureEmojiStatus"))]
    EmojiStatus,
    /// The ability to display folder names for each chat in the chat list
    #[serde(rename(serialize = "businessFeatureChatFolderTags", deserialize = "businessFeatureChatFolderTags"))]
    ChatFolderTags,
    /// Allowed to use many additional features for stories
    #[serde(rename(serialize = "businessFeatureUpgradedStories", deserialize = "businessFeatureUpgradedStories"))]
    UpgradedStories,
}
