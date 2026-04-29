#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAdministratorRights {
    /// Describes rights of the administrator
    #[serde(rename(serialize = "chatAdministratorRights", deserialize = "chatAdministratorRights"))]
    ChatAdministratorRights(crate::types::ChatAdministratorRights),
}
