#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user authorization state has changed
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateAuthorizationState {
    /// New authorization state
    pub authorization_state: crate::enums::AuthorizationState,
}
