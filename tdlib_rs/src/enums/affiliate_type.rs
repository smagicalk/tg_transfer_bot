#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateType {
    /// The affiliate is the current user
    #[serde(rename(
        serialize = "affiliateTypeCurrentUser",
        deserialize = "affiliateTypeCurrentUser"
    ))]
    CurrentUser,
    /// The affiliate is a bot owned by the current user
    #[serde(rename(serialize = "affiliateTypeBot", deserialize = "affiliateTypeBot"))]
    Bot(crate::types::AffiliateTypeBot),
    /// The affiliate is a channel chat where the current user has can_post_messages administrator right
    #[serde(rename(
        serialize = "affiliateTypeChannel",
        deserialize = "affiliateTypeChannel"
    ))]
    Channel(crate::types::AffiliateTypeChannel),
}
