#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains information about one website the current user is logged in with Telegram
#[serde_as]
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ConnectedWebsite {
    /// Website identifier
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// The domain name of the website
    pub domain_name: String,
    /// User identifier of a bot linked with the website
    pub bot_user_id: i64,
    /// The version of a browser used to log in
    pub browser: String,
    /// Operating system the browser is running on
    pub platform: String,
    /// Point in time (Unix timestamp) when the user was logged in
    pub log_in_date: i32,
    /// Point in time (Unix timestamp) when obtained authorization was last used
    pub last_active_date: i32,
    /// IP address from which the user was logged in, in human-readable format
    pub ip_address: String,
    /// Human-readable description of a country and a region from which the user was logged in, based on the IP address
    pub location: String,
}
