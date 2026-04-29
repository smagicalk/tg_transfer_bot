#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TestBytes {
    /// A simple object containing a sequence of bytes; for testing only
    #[serde(rename(serialize = "testBytes", deserialize = "testBytes"))]
    TestBytes(crate::types::TestBytes),
}
