#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AgeVerificationParameters {
    /// Describes parameters for age verification of the current user
    #[serde(rename(
        serialize = "ageVerificationParameters",
        deserialize = "ageVerificationParameters"
    ))]
    AgeVerificationParameters(crate::types::AgeVerificationParameters),
}
