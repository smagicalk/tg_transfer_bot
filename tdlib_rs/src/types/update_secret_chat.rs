#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateSecretChat {
    /// New data about the secret chat
    pub secret_chat: crate::types::SecretChat,
}
