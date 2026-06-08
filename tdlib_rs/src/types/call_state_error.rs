#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The call has ended with an error
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallStateError {
    /// Error. An error with the code 4005000 will be returned if an outgoing call is missed because of an expired timeout
    pub error: crate::types::Error,
}
