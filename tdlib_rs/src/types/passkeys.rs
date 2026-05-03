#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of passkeys
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Passkeys {
    /// List of passkeys
    pub passkeys: Vec<crate::types::Passkey>,
}
