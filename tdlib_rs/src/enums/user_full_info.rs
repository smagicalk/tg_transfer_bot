#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserFullInfo {
    /// Contains full information about a user
    #[serde(rename(serialize = "userFullInfo", deserialize = "userFullInfo"))]
    UserFullInfo(crate::types::UserFullInfo),
}
