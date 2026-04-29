#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A simple object containing a vector of strings; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestVectorString {
    /// Vector of strings
    pub value: Vec<String>,
}
