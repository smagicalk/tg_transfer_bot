#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a temporary identifier of validated order information, which is stored for one hour, and the available shipping options
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ValidatedOrderInfo {
    /// Temporary identifier of the order information
    pub order_info_id: String,
    /// Available shipping options
    pub shipping_options: Vec<crate::types::ShippingOption>,
}
