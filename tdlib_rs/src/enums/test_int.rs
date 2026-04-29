#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestInt {
    /// A simple object containing a number; for testing only
    #[serde(rename(serialize = "testInt", deserialize = "testInt"))]
    TestInt(crate::types::TestInt),
}
