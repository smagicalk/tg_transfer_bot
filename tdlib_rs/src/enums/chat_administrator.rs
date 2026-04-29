#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAdministrator {
    /// Contains information about a chat administrator
    #[serde(rename(serialize = "chatAdministrator", deserialize = "chatAdministrator"))]
    ChatAdministrator(crate::types::ChatAdministrator),
}
