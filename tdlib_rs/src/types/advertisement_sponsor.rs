#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Information about the sponsor of an advertisement
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AdvertisementSponsor {
    /// URL of the sponsor to be opened when the advertisement is clicked
    pub url: String,
    /// Photo of the sponsor; may be null if must not be shown
    pub photo: Option<crate::types::Photo>,
    /// Additional optional information about the sponsor to be shown along with the advertisement
    pub info: String,
}
