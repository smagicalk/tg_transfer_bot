#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A simple object containing a vector of objects that hold a number; for testing only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct TestVectorIntObject {
    /// Vector of objects
    pub value: Vec<crate::types::TestInt>,
}
