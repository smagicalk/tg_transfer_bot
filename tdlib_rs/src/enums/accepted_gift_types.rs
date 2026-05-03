#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AcceptedGiftTypes {
    /// Describes gift types that are accepted by a user
    #[serde(rename(serialize = "acceptedGiftTypes", deserialize = "acceptedGiftTypes"))]
    AcceptedGiftTypes(crate::types::AcceptedGiftTypes),
}
