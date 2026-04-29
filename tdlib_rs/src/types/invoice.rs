#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Product invoice
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Invoice {
    /// ISO 4217 currency code
    pub currency: String,
    /// A list of objects used to calculate the total price of the product
    pub price_parts: Vec<crate::types::LabeledPricePart>,
    /// The number of seconds between consecutive Telegram Star debiting for subscription invoices; 0 if the invoice doesn't create subscription
    pub subscription_period: i32,
    /// The maximum allowed amount of tip in the smallest units of the currency
    pub max_tip_amount: i64,
    /// Suggested amounts of tip in the smallest units of the currency
    pub suggested_tip_amounts: Vec<i64>,
    /// An HTTP URL with terms of service for recurring payments. If non-empty, the invoice payment will result in recurring payments and the user must accept the terms of service before allowed to pay
    pub recurring_payment_terms_of_service_url: String,
    /// An HTTP URL with terms of service for non-recurring payments. If non-empty, then the user must accept the terms of service before allowed to pay
    pub terms_of_service_url: String,
    /// True, if the payment is a test payment
    pub is_test: bool,
    /// True, if the user's name is needed for payment
    pub need_name: bool,
    /// True, if the user's phone number is needed for payment
    pub need_phone_number: bool,
    /// True, if the user's email address is needed for payment
    pub need_email_address: bool,
    /// True, if the user's shipping address is needed for payment
    pub need_shipping_address: bool,
    /// True, if the user's phone number will be sent to the provider
    pub send_phone_number_to_provider: bool,
    /// True, if the user's email address will be sent to the provider
    pub send_email_address_to_provider: bool,
    /// True, if the total price depends on the shipping method
    pub is_flexible: bool,
}
