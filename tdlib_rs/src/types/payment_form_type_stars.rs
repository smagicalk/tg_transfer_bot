#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The payment form is for a payment in Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentFormTypeStars {
    /// Number of Telegram Stars that will be paid
    pub star_count: i64,
}
