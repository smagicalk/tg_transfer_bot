#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SuggestedPostRefundReason {
    /// The post was refunded, because it was deleted by channel administrators in less than getOption("suggested_post_lifetime_min") seconds
    #[serde(rename(serialize = "suggestedPostRefundReasonPostDeleted", deserialize = "suggestedPostRefundReasonPostDeleted"))]
    PostDeleted,
    /// The post was refunded, because the payment for the post was refunded
    #[serde(rename(serialize = "suggestedPostRefundReasonPaymentRefunded", deserialize = "suggestedPostRefundReasonPaymentRefunded"))]
    PaymentRefunded,
}
