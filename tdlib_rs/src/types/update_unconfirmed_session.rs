#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The first unconfirmed session has changed
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateUnconfirmedSession {
    /// The unconfirmed session; may be null if none
    pub session: Option<crate::types::UnconfirmedSession>,
}
