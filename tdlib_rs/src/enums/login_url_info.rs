#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LoginUrlInfo {
    /// An HTTP URL needs to be open
    #[serde(rename(serialize = "loginUrlInfoOpen", deserialize = "loginUrlInfoOpen"))]
    Open(crate::types::LoginUrlInfoOpen),
    /// An authorization confirmation dialog needs to be shown to the user
    #[serde(rename(
        serialize = "loginUrlInfoRequestConfirmation",
        deserialize = "loginUrlInfoRequestConfirmation"
    ))]
    RequestConfirmation(crate::types::LoginUrlInfoRequestConfirmation),
}
