#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The list of supported dice emojis has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateDiceEmojis {
    /// The new list of supported dice emojis
    pub emojis: Vec<String>,
}
