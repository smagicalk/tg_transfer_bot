#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PhoneNumberInfo {
    /// Contains information about a phone number
    #[serde(rename(serialize = "phoneNumberInfo", deserialize = "phoneNumberInfo"))]
    PhoneNumberInfo(crate::types::PhoneNumberInfo),
}
