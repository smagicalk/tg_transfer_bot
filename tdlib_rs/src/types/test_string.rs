#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A simple object containing a string; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestString {
    /// String
    pub value: String,
}
