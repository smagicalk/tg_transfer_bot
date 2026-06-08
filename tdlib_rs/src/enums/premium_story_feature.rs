#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumStoryFeature {
    /// Stories of the current user are displayed before stories of non-Premium contacts, supergroups, and channels
    #[serde(rename(
        serialize = "premiumStoryFeaturePriorityOrder",
        deserialize = "premiumStoryFeaturePriorityOrder"
    ))]
    PriorityOrder,
    /// The ability to hide the fact that the user viewed other's stories
    #[serde(rename(
        serialize = "premiumStoryFeatureStealthMode",
        deserialize = "premiumStoryFeatureStealthMode"
    ))]
    StealthMode,
    /// The ability to check who opened the current user's stories after they expire
    #[serde(rename(
        serialize = "premiumStoryFeaturePermanentViewsHistory",
        deserialize = "premiumStoryFeaturePermanentViewsHistory"
    ))]
    PermanentViewsHistory,
    /// The ability to set custom expiration duration for stories
    #[serde(rename(
        serialize = "premiumStoryFeatureCustomExpirationDuration",
        deserialize = "premiumStoryFeatureCustomExpirationDuration"
    ))]
    CustomExpirationDuration,
    /// The ability to save other's unprotected stories
    #[serde(rename(
        serialize = "premiumStoryFeatureSaveStories",
        deserialize = "premiumStoryFeatureSaveStories"
    ))]
    SaveStories,
    /// The ability to use links and formatting in story caption, and use inputStoryAreaTypeLink areas
    #[serde(rename(
        serialize = "premiumStoryFeatureLinksAndFormatting",
        deserialize = "premiumStoryFeatureLinksAndFormatting"
    ))]
    LinksAndFormatting,
    /// The ability to choose better quality for viewed stories
    #[serde(rename(
        serialize = "premiumStoryFeatureVideoQuality",
        deserialize = "premiumStoryFeatureVideoQuality"
    ))]
    VideoQuality,
}
