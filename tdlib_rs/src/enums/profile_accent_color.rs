#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ProfileAccentColor {
    /// Contains information about supported accent color for user profile photo background
    #[serde(rename(serialize = "profileAccentColor", deserialize = "profileAccentColor"))]
    ProfileAccentColor(crate::types::ProfileAccentColor),
}
