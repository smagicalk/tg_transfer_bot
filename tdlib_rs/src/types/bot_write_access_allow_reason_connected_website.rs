#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The user connected a website by logging in using Telegram Login Widget on it
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct BotWriteAccessAllowReasonConnectedWebsite {
    /// Domain name of the connected website
    pub domain_name: String,
}
