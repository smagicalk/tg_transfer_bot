#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AccountTtl {
    /// Contains information about the period of inactivity after which the current user's account will automatically be deleted
    #[serde(rename(serialize = "accountTtl", deserialize = "accountTtl"))]
    AccountTtl(crate::types::AccountTtl),
}
