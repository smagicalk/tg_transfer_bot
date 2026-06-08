#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpdateTermsOfService {
    /// Identifier of the terms of service
    pub terms_of_service_id: String,
    /// The new terms of service
    pub terms_of_service: crate::types::TermsOfService,
}
