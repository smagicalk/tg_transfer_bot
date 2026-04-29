#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The call has ended successfully
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CallStateDiscarded {
    /// The reason why the call has ended
    pub reason: crate::enums::CallDiscardReason,
    /// True, if the call rating must be sent to the server
    pub need_rating: bool,
    /// True, if the call debug information must be sent to the server
    pub need_debug_information: bool,
    /// True, if the call log must be sent to the server
    pub need_log: bool,
}
