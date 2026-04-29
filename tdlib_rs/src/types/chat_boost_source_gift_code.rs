#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The chat created a Telegram Premium gift code for a user
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ChatBoostSourceGiftCode {
    /// Identifier of a user, for which the gift code was created
    pub user_id: i64,
    /// The created Telegram Premium gift code, which is known only if this is a gift code for the current user, or it has already been claimed
    pub gift_code: String,
}
