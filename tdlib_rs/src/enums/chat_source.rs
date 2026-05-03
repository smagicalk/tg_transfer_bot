#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatSource {
    /// The chat is sponsored by the user's MTProxy server
    #[serde(rename(
        serialize = "chatSourceMtprotoProxy",
        deserialize = "chatSourceMtprotoProxy"
    ))]
    MtprotoProxy,
    /// The chat contains a public service announcement
    #[serde(rename(
        serialize = "chatSourcePublicServiceAnnouncement",
        deserialize = "chatSourcePublicServiceAnnouncement"
    ))]
    PublicServiceAnnouncement(crate::types::ChatSourcePublicServiceAnnouncement),
}
