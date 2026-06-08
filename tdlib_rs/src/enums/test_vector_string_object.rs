#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestVectorStringObject {
    /// A simple object containing a vector of objects that hold a string; for testing only
    #[serde(rename(
        serialize = "testVectorStringObject",
        deserialize = "testVectorStringObject"
    ))]
    TestVectorStringObject(crate::types::TestVectorStringObject),
}
