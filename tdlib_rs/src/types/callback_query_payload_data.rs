#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The payload for a general callback button
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallbackQueryPayloadData {
    /// Data that was attached to the callback button
    pub data: String,
}
