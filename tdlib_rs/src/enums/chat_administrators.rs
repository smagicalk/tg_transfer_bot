#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAdministrators {
    /// Represents a list of chat administrators
    #[serde(rename(serialize = "chatAdministrators", deserialize = "chatAdministrators"))]
    ChatAdministrators(crate::types::ChatAdministrators),
}
