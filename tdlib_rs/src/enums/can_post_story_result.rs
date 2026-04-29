#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CanPostStoryResult {
    /// A story can be sent
    #[serde(rename(serialize = "canPostStoryResultOk", deserialize = "canPostStoryResultOk"))]
    Ok(crate::types::CanPostStoryResultOk),
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
    WeeklyLimitExceeded(crate::types::CanPostStoryResultWeeklyLimitExceeded),
    /// The monthly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
    #[serde(rename(serialize = "canPostStoryResultMonthlyLimitExceeded", deserialize = "canPostStoryResultMonthlyLimitExceeded"))]
    MonthlyLimitExceeded(crate::types::CanPostStoryResultMonthlyLimitExceeded),
    /// The user or the chat has an active live story. The live story must be deleted first
    #[serde(rename(serialize = "canPostStoryResultLiveStoryIsActive", deserialize = "canPostStoryResultLiveStoryIsActive"))]
    LiveStoryIsActive(crate::types::CanPostStoryResultLiveStoryIsActive),
}
