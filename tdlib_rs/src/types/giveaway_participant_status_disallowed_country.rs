#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// The user can't participate in the giveaway, because they phone number is from a disallowed country
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParticipantStatusDisallowedCountry {
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's country
    pub user_country_code: String,
}
