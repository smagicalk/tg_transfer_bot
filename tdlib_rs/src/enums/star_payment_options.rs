#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StarPaymentOptions {
    /// Contains a list of options for buying Telegram Stars
    #[serde(rename(serialize = "starPaymentOptions", deserialize = "starPaymentOptions"))]
    StarPaymentOptions(crate::types::StarPaymentOptions),
}
