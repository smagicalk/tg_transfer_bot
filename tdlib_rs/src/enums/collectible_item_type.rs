#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CollectibleItemType {
    /// A username
    #[serde(rename(serialize = "collectibleItemTypeUsername", deserialize = "collectibleItemTypeUsername"))]
    Username(crate::types::CollectibleItemTypeUsername),
    /// A phone number
    #[serde(rename(serialize = "collectibleItemTypePhoneNumber", deserialize = "collectibleItemTypePhoneNumber"))]
    PhoneNumber(crate::types::CollectibleItemTypePhoneNumber),
}
