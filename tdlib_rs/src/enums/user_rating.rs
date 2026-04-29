#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserRating {
    /// Contains description of user rating
    #[serde(rename(serialize = "userRating", deserialize = "userRating"))]
    UserRating(crate::types::UserRating),
}
