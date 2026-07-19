#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A business connection has changed; for bots only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateBusinessConnection {
    /// New data about the connection
    pub connection: crate::types::BusinessConnection,
}
