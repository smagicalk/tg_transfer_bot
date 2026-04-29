#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Information about the OAuth authorization
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OauthLinkInfo {
    /// Identifier of the user for which the link was generated; may be 0 if unknown. The corresponding user may be unknown.
    /// If the user is logged in the app, then they must be chosen for authorization by default
    pub user_id: i64,
    /// An HTTP URL where the user authorizes
    pub url: String,
    /// A domain of the URL
    pub domain: String,
    /// User identifier of a bot linked with the website
    pub bot_user_id: i64,
    /// True, if the user must be asked for the permission to the bot to send them messages
    pub request_write_access: bool,
    /// True, if the user must be asked for the permission to share their phone number
    pub request_phone_number_access: bool,
    /// The version of a browser used for the authorization
    pub browser: String,
    /// Operating system the browser is running on
    pub platform: String,
    /// IP address from which the authorization is performed, in human-readable format
    pub ip_address: String,
    /// Human-readable description of a country and a region from which the authorization is performed, based on the IP address
    pub location: String,
    /// True, if code matching dialog must be shown first and checkOauthRequestMatchCode must be called before acceptOauthRequest. Otherwise, checkOauthRequestMatchCode must not be called
    pub match_code_first: bool,
    /// The list of codes to match; may be empty if irrelevant
    pub match_codes: Vec<String>,
}
