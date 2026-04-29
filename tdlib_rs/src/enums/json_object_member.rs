#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum JsonObjectMember {
    /// Represents one member of a JSON object
    #[serde(rename(serialize = "jsonObjectMember", deserialize = "jsonObjectMember"))]
    JsonObjectMember(crate::types::JsonObjectMember),
}
