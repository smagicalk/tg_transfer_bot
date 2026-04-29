#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A just ended call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCallDiscarded {
    /// Identifier of the call
    pub call_id: i32,
}
