#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BotVerificationParameters {
    /// Describes parameters of verification that is provided by a bot
    #[serde(rename(serialize = "botVerificationParameters", deserialize = "botVerificationParameters"))]
    BotVerificationParameters(crate::types::BotVerificationParameters),
}
