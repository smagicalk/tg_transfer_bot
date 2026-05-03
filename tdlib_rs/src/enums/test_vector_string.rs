#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestVectorString {
    /// A simple object containing a vector of strings; for testing only
    #[serde(rename(serialize = "testVectorString", deserialize = "testVectorString"))]
    TestVectorString(crate::types::TestVectorString),
}
