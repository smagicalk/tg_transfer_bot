#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserSupportInfo {
    /// Contains custom information about the user
    #[serde(rename(serialize = "userSupportInfo", deserialize = "userSupportInfo"))]
    UserSupportInfo(crate::types::UserSupportInfo),
}
