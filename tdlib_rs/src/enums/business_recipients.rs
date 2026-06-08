#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum BusinessRecipients {
    /// Describes private chats chosen for automatic interaction with a business
    #[serde(rename(serialize = "businessRecipients", deserialize = "businessRecipients"))]
    BusinessRecipients(crate::types::BusinessRecipients),
}
