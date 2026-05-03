#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CollectibleItemTypePhoneNumber {
    /// The phone number
    pub phone_number: String,
}
