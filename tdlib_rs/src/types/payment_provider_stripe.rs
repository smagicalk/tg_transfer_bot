#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Stripe payment provider
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PaymentProviderStripe {
    /// Stripe API publishable key
    pub publishable_key: String,
    /// True, if the user country must be provided
    pub need_country: bool,
    /// True, if the user ZIP/postal code must be provided
    pub need_postal_code: bool,
    /// True, if the cardholder name must be provided
    pub need_cardholder_name: bool,
}
