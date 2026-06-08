#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Describes a venue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Venue {
    /// Venue location; as defined by the sender
    pub location: crate::types::Location,
    /// Venue name; as defined by the sender
    pub title: String,
    /// Venue address; as defined by the sender
    pub address: String,
    /// Provider of the venue database; as defined by the sender. Currently, only "foursquare" and "gplaces" (Google Places) need to be supported
    pub provider: String,
    /// Identifier of the venue in the provider database; as defined by the sender
    pub id: String,
    /// Type of the venue in the provider database; as defined by the sender
    pub r#type: String,
}
