#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ProfileAccentColors {
    /// Contains information about supported accent colors for user profile photo background in RGB format
    #[serde(rename(serialize = "profileAccentColors", deserialize = "profileAccentColors"))]
    ProfileAccentColors(crate::types::ProfileAccentColors),
}
