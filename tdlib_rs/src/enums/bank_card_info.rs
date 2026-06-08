#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BankCardInfo {
    /// Information about a bank card
    #[serde(rename(serialize = "bankCardInfo", deserialize = "bankCardInfo"))]
    BankCardInfo(crate::types::BankCardInfo),
}
