#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The parameters for age verification of the current user's account has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAgeVerificationParameters {
    /// Parameters for the age verification; may be null if age verification isn't needed
    pub parameters: Option<crate::types::AgeVerificationParameters>,
}
