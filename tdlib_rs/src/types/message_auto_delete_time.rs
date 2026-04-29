#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains default auto-delete timer setting for new chats
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessageAutoDeleteTime {
    /// Message auto-delete time, in seconds. If 0, then messages aren't deleted automatically
    pub time: i32,
}
