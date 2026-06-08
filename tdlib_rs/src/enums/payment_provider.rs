#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PaymentProvider {
    /// Smart Glocal payment provider
    #[serde(rename(
        serialize = "paymentProviderSmartGlocal",
        deserialize = "paymentProviderSmartGlocal"
    ))]
    SmartGlocal(crate::types::PaymentProviderSmartGlocal),
    /// Stripe payment provider
    #[serde(rename(
        serialize = "paymentProviderStripe",
        deserialize = "paymentProviderStripe"
    ))]
    Stripe(crate::types::PaymentProviderStripe),
    /// Some other payment provider, for which a web payment form must be shown
    #[serde(rename(
        serialize = "paymentProviderOther",
        deserialize = "paymentProviderOther"
    ))]
    Other(crate::types::PaymentProviderOther),
}
