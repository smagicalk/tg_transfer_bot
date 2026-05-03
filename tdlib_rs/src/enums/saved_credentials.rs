#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SavedCredentials {
    /// Contains information about saved payment credentials
    #[serde(rename(serialize = "savedCredentials", deserialize = "savedCredentials"))]
    SavedCredentials(crate::types::SavedCredentials),
}
