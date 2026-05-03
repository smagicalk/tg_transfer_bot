#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AccountInfo {
    /// Contains basic information about another user who started a chat with the current user
    #[serde(rename(serialize = "accountInfo", deserialize = "accountInfo"))]
    AccountInfo(crate::types::AccountInfo),
}
