#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The payment form is for a regular payment
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PaymentFormTypeRegular {
    /// Full information about the invoice
    pub invoice: crate::types::Invoice,
    /// User identifier of the payment provider bot
    pub payment_provider_user_id: i64,
    /// Information about the payment provider
    pub payment_provider: crate::enums::PaymentProvider,
    /// The list of additional payment options
    pub additional_payment_options: Vec<crate::types::PaymentOption>,
    /// Saved server-side order information; may be null
    pub saved_order_info: Option<crate::types::OrderInfo>,
    /// The list of saved payment credentials
    pub saved_credentials: Vec<crate::types::SavedCredentials>,
    /// True, if the user can choose to save credentials
    pub can_save_credentials: bool,
    /// True, if the user will be able to save credentials, if sets up a 2-step verification password
    pub need_password: bool,
}
