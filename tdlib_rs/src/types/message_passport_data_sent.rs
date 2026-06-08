#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Telegram Passport data has been sent to a bot
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct MessagePassportDataSent {
    /// List of Telegram Passport element types sent
    pub types: Vec<crate::enums::PassportElementType>,
}
