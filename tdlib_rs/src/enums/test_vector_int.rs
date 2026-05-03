#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestVectorInt {
    /// A simple object containing a vector of numbers; for testing only
    #[serde(rename(serialize = "testVectorInt", deserialize = "testVectorInt"))]
    TestVectorInt(crate::types::TestVectorInt),
}
