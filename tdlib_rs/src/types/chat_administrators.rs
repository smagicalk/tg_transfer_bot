#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Represents a list of chat administrators
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatAdministrators {
    /// A list of chat administrators
    pub administrators: Vec<crate::types::ChatAdministrator>,
}
