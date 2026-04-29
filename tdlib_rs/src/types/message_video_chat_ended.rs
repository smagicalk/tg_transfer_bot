#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with information about an ended video chat
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageVideoChatEnded {
    /// Call duration, in seconds
    pub duration: i32,
}
