#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The payload for a callback button requiring password
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallbackQueryPayloadDataWithPassword {
    /// The 2-step verification password for the current user
    pub password: String,
    /// Data that was attached to the callback button
    pub data: String,
}
