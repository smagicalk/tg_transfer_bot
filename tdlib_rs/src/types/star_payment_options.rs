#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains a list of options for buying Telegram Stars
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct StarPaymentOptions {
    /// The list of options
    pub options: Vec<crate::types::StarPaymentOption>,
}
