#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes an affiliate program that was connected to an affiliate
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedAffiliateProgram {
    /// The link that can be used to refer users if the program is still active
    pub url: String,
    /// User identifier of the bot created the program
    pub bot_user_id: i64,
    /// The parameters of the affiliate program
    pub parameters: crate::types::AffiliateProgramParameters,
    /// Point in time (Unix timestamp) when the affiliate program was connected
    pub connection_date: i32,
    /// True, if the program was canceled by the bot, or disconnected by the chat owner and isn't available anymore
    pub is_disconnected: bool,
    /// The number of users that used the affiliate program
    #[serde_as(as = "DisplayFromStr")]
    pub user_count: i64,
    /// The number of Telegram Stars that were earned by the affiliate program
    pub revenue_star_count: i64,
}
