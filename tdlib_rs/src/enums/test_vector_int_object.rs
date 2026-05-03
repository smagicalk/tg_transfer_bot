#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestVectorIntObject {
    /// A simple object containing a vector of objects that hold a number; for testing only
    #[serde(rename(serialize = "testVectorIntObject", deserialize = "testVectorIntObject"))]
    TestVectorIntObject(crate::types::TestVectorIntObject),
}
