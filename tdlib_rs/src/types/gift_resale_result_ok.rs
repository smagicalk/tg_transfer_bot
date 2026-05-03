#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Operation was successfully completed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiftResaleResultOk {
    /// Unique identifier of the received gift; only for the gifts sent to the current user
    pub received_gift_id: String,
}
