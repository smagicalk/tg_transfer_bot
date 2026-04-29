#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Order information
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct OrderInfo {
    /// Name of the user
    pub name: String,
    /// Phone number of the user
    pub phone_number: String,
    /// Email address of the user
    pub email_address: String,
    /// Shipping address for this order; may be null
    pub shipping_address: Option<crate::types::Address>,
}
