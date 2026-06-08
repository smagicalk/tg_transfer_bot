#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AdvertisementSponsor {
    /// Information about the sponsor of an advertisement
    #[serde(rename(
        serialize = "advertisementSponsor",
        deserialize = "advertisementSponsor"
    ))]
    AdvertisementSponsor(crate::types::AdvertisementSponsor),
}
