#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of gifts that can be sent to another user or channel chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AvailableGifts {
    /// The list of gifts
    pub gifts: Vec<crate::types::AvailableGift>,
}
