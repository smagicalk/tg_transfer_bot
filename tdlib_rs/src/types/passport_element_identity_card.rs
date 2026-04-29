#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram Passport element containing the user's identity card
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementIdentityCard {
    /// Identity card
    pub identity_card: crate::types::IdentityDocument,
}
