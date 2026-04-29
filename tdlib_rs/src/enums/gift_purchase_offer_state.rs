#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum GiftPurchaseOfferState {
    /// The offer must be accepted or rejected
    #[serde(rename(serialize = "giftPurchaseOfferStatePending", deserialize = "giftPurchaseOfferStatePending"))]
    Pending,
    /// The offer was accepted
    #[serde(rename(serialize = "giftPurchaseOfferStateAccepted", deserialize = "giftPurchaseOfferStateAccepted"))]
    Accepted,
    /// The offer was rejected
    #[serde(rename(serialize = "giftPurchaseOfferStateRejected", deserialize = "giftPurchaseOfferStateRejected"))]
    Rejected,
}
