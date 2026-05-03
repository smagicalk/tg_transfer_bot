#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// New call was created or information about a call was updated
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateCall {
    /// New data about a call
    pub call: crate::types::Call,
}
