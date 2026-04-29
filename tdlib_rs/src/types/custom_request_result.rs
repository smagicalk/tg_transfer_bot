#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains the result of a custom request
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomRequestResult {
    /// A JSON-serialized result
    pub result: String,
}
