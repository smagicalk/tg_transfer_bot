#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Regular gift
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SentGiftRegular {
    /// The gift
    pub gift: crate::types::Gift,
}
