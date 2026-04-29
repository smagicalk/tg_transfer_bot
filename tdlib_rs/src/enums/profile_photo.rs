#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ProfilePhoto {
    /// Describes a user profile photo
    #[serde(rename(serialize = "profilePhoto", deserialize = "profilePhoto"))]
    ProfilePhoto(crate::types::ProfilePhoto),
}
