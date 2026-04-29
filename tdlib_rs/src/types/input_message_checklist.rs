#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A message with a checklist. Checklists can't be sent to secret chats, channel chats and channel direct messages chats; for Telegram Premium users only
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InputMessageChecklist {
    /// The checklist to send
    pub checklist: crate::types::InputChecklist,
}
