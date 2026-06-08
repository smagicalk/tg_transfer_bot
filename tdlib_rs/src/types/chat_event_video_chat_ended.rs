#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// A video chat was ended
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatEventVideoChatEnded {
    /// Identifier of the video chat. The video chat can be received through the method getGroupCall
    pub group_call_id: i32,
}
