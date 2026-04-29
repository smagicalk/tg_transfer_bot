#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes a found affiliate program
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FoundAffiliateProgram {
    /// User identifier of the bot created the program
    pub bot_user_id: i64,
    /// Information about the affiliate program
    pub info: crate::types::AffiliateProgramInfo,
}
