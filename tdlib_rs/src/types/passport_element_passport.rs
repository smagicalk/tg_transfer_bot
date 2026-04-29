#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// A Telegram Passport element containing the user's passport
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PassportElementPassport {
    /// Passport
    pub passport: crate::types::IdentityDocument,
}
