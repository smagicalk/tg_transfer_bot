#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The list of bots added to attachment or side menu has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateAttachmentMenuBots {
    /// The new list of bots. The bots must not be shown on scheduled messages screen
    pub bots: Vec<crate::types::AttachmentMenuBot>,
}
