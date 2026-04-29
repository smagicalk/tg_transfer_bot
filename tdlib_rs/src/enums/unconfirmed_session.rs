#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UnconfirmedSession {
    /// Contains information about an unconfirmed session
    #[serde(rename(serialize = "unconfirmedSession", deserialize = "unconfirmedSession"))]
    UnconfirmedSession(crate::types::UnconfirmedSession),
}
