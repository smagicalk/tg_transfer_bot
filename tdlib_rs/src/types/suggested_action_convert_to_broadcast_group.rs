#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Suggests the user to convert specified supergroup to a broadcast group
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SuggestedActionConvertToBroadcastGroup {
    /// Supergroup identifier
    pub supergroup_id: i64,
}
