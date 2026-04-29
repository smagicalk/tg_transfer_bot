#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TmeUrlType {
    /// A URL linking to a user
    #[serde(rename(serialize = "tMeUrlTypeUser", deserialize = "tMeUrlTypeUser"))]
    User(crate::types::TmeUrlTypeUser),
    /// A URL linking to a public supergroup or channel
    #[serde(rename(serialize = "tMeUrlTypeSupergroup", deserialize = "tMeUrlTypeSupergroup"))]
    Supergroup(crate::types::TmeUrlTypeSupergroup),
    /// A chat invite link
    #[serde(rename(serialize = "tMeUrlTypeChatInvite", deserialize = "tMeUrlTypeChatInvite"))]
    ChatInvite(crate::types::TmeUrlTypeChatInvite),
    /// A URL linking to a sticker set
    #[serde(rename(serialize = "tMeUrlTypeStickerSet", deserialize = "tMeUrlTypeStickerSet"))]
    StickerSet(crate::types::TmeUrlTypeStickerSet),
}
