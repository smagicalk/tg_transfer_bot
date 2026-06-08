#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about a phone number
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PhoneNumberInfo {
    /// Information about the country to which the phone number belongs; may be null
    pub country: Option<crate::types::CountryInfo>,
    /// The part of the phone number denoting country calling code or its part
    pub country_calling_code: String,
    /// The phone number without country calling code formatted accordingly to local rules. Expected digits are returned as '-', but even more digits might be entered by the user
    pub formatted_phone_number: String,
    /// True, if the phone number was bought at https:fragment.com and isn't tied to a SIM card. Information about the phone number can be received using getCollectibleItemInfo
    pub is_anonymous: bool,
}
