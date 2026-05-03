#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Chat has_protected_content setting was requested to be disabled
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageChatHasProtectedContentDisableRequested {
    /// True, if the request has expired
    pub is_expired: bool,
}
