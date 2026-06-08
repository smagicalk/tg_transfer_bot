#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A simple object containing a vector of numbers; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestVectorInt {
    /// Vector of numbers
    pub value: Vec<i32>,
}
