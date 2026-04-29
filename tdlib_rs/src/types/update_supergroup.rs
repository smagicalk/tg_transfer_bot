#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateSupergroup {
    /// New data about the supergroup
    pub supergroup: crate::types::Supergroup,
}
