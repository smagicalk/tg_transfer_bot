#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ReportSponsoredResult {
    /// The message was reported successfully
    #[serde(rename(
        serialize = "reportSponsoredResultOk",
        deserialize = "reportSponsoredResultOk"
    ))]
    Ok,
    /// The sponsored message is too old or not found
    #[serde(rename(
        serialize = "reportSponsoredResultFailed",
        deserialize = "reportSponsoredResultFailed"
    ))]
    Failed,
    /// The user must choose an option to report the message and repeat request with the chosen option
    #[serde(rename(
        serialize = "reportSponsoredResultOptionRequired",
        deserialize = "reportSponsoredResultOptionRequired"
    ))]
    OptionRequired(crate::types::ReportSponsoredResultOptionRequired),
    /// Sponsored messages were hidden for the user in all chats
    #[serde(rename(
        serialize = "reportSponsoredResultAdsHidden",
        deserialize = "reportSponsoredResultAdsHidden"
    ))]
    AdsHidden,
    /// The user asked to hide sponsored messages, but Telegram Premium is required for this
    #[serde(rename(
        serialize = "reportSponsoredResultPremiumRequired",
        deserialize = "reportSponsoredResultPremiumRequired"
    ))]
    PremiumRequired,
}
