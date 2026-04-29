#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BankCardActionOpenUrl {
    /// Describes an action associated with a bank card number
    #[serde(rename(serialize = "bankCardActionOpenUrl", deserialize = "bankCardActionOpenUrl"))]
    BankCardActionOpenUrl(crate::types::BankCardActionOpenUrl),
}
