#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumSource {
    /// A limit was exceeded
    #[serde(rename(
        serialize = "premiumSourceLimitExceeded",
        deserialize = "premiumSourceLimitExceeded"
    ))]
    LimitExceeded(crate::types::PremiumSourceLimitExceeded),
    /// A user tried to use a Premium feature
    #[serde(rename(
        serialize = "premiumSourceFeature",
        deserialize = "premiumSourceFeature"
    ))]
    Feature(crate::types::PremiumSourceFeature),
    /// A user tried to use a Business feature
    #[serde(rename(
        serialize = "premiumSourceBusinessFeature",
        deserialize = "premiumSourceBusinessFeature"
    ))]
    BusinessFeature(crate::types::PremiumSourceBusinessFeature),
    /// A user tried to use a Premium story feature
    #[serde(rename(
        serialize = "premiumSourceStoryFeature",
        deserialize = "premiumSourceStoryFeature"
    ))]
    StoryFeature(crate::types::PremiumSourceStoryFeature),
    /// A user opened an internal link of the type internalLinkTypePremiumFeaturesPage
    #[serde(rename(serialize = "premiumSourceLink", deserialize = "premiumSourceLink"))]
    Link(crate::types::PremiumSourceLink),
    /// A user opened the Premium features screen from settings
    #[serde(rename(
        serialize = "premiumSourceSettings",
        deserialize = "premiumSourceSettings"
    ))]
    Settings,
}
