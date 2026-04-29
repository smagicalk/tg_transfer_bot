#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// An OAuth authorization request was received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewOauthRequest {
    /// A domain of the URL where the user authorizes
    pub domain: String,
    /// Human-readable description of a country and a region from which the authorization is performed, based on the IP address
    pub location: String,
    /// The URL to pass to getOauthLinkInfo; the link is valid for 60 seconds
    pub url: String,
}
