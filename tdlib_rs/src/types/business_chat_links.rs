#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of business chat links created by the user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BusinessChatLinks {
    /// List of links
    pub links: Vec<crate::types::BusinessChatLink>,
}
