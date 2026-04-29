#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The gift can't be sent now by the current user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CanSendGiftResultFail {
    /// Reason to be shown to the user
    pub reason: crate::types::FormattedText,
}
