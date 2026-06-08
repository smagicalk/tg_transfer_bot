#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// An object of this type can be returned on every function call, in case of an error
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Error {
    /// Error code; subject to future changes. If the error code is 406, the error message must not be processed in any way and must not be displayed to the user
    pub code: i32,
    /// Error message; subject to future changes
    pub message: String,
}
