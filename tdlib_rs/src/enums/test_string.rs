#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestString {
    /// A simple object containing a string; for testing only
    #[serde(rename(serialize = "testString", deserialize = "testString"))]
    TestString(crate::types::TestString),
}
