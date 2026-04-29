#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The call was ended because it has been upgraded to a group call
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallDiscardReasonUpgradeToGroupCall {
    /// Invite link for the group call
    pub invite_link: String,
}
