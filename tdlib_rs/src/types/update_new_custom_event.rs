#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A new incoming event; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCustomEvent {
    /// A JSON-serialized event
    pub event: String,
}
