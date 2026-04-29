#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The connection state has changed. This update must be used only to show a human-readable description of the connection state
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateConnectionState {
    /// The new connection state
    pub state: crate::enums::ConnectionState,
}
