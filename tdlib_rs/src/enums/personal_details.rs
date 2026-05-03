#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PersonalDetails {
    /// Contains the user's personal details
    #[serde(rename(serialize = "personalDetails", deserialize = "personalDetails"))]
    PersonalDetails(crate::types::PersonalDetails),
}
