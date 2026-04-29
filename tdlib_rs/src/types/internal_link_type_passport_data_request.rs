#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application; otherwise, ignore it
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypePassportDataRequest {
    /// User identifier of the service's bot; the corresponding user may be unknown yet
    pub bot_user_id: i64,
    /// Telegram Passport element types requested by the service
    pub scope: String,
    /// Service's public key
    pub public_key: String,
    /// Unique request identifier provided by the service
    pub nonce: String,
    /// An HTTP URL to open once the request is finished, canceled, or failed with the parameters tg_passport=success, tg_passport=cancel, or tg_passport=error&error=... respectively.
    /// If empty, then onActivityResult method must be used to return response on Android, or the link tgbot{bot_user_id}:passport/success or tgbot{bot_user_id}:passport/cancel must be opened otherwise
    pub callback_url: String,
}
