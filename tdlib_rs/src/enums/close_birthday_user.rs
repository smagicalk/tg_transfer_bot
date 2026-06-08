#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CloseBirthdayUser {
    /// Describes a user who had or will have a birthday soon
    #[serde(rename(serialize = "closeBirthdayUser", deserialize = "closeBirthdayUser"))]
    CloseBirthdayUser(crate::types::CloseBirthdayUser),
}
