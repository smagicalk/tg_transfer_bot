#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// The link is a link to a gift collection. Call searchPublicChat with the given username, then call getReceivedGifts with the received gift owner identifier
/// and the given collection identifier, then show the collection if received
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct InternalLinkTypeGiftCollection {
    /// Username of the owner of the gift collection
    pub gift_owner_username: String,
    /// Gift collection identifier
    pub collection_id: i32,
}
