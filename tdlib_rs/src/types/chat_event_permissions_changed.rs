#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The chat permissions were changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventPermissionsChanged {
    /// Previous chat permissions
    pub old_permissions: crate::types::ChatPermissions,
    /// New chat permissions
    pub new_permissions: crate::types::ChatPermissions,
}
