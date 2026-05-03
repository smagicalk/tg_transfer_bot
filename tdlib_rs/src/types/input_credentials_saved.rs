#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputCredentialsSaved {
    /// Identifier of the saved credentials
    pub saved_credentials_id: String,
}
