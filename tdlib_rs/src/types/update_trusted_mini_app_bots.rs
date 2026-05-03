#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Lists of bots which Mini Apps must be allowed to read text from clipboard and must be opened without a warning
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateTrustedMiniAppBots {
    /// List of user identifiers of the bots; the corresponding users may not be sent using updateUser updates and may not be accessible
    pub bot_user_ids: Vec<i64>,
}
