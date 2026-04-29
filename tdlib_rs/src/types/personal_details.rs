#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Contains the user's personal details
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct PersonalDetails {
    /// First name of the user written in English; 1-255 characters
    pub first_name: String,
    /// Middle name of the user written in English; 0-255 characters
    pub middle_name: String,
    /// Last name of the user written in English; 1-255 characters
    pub last_name: String,
    /// Native first name of the user; 1-255 characters
    pub native_first_name: String,
    /// Native middle name of the user; 0-255 characters
    pub native_middle_name: String,
    /// Native last name of the user; 1-255 characters
    pub native_last_name: String,
    /// Birthdate of the user
    pub birthdate: crate::types::Date,
    /// Gender of the user, "male" or "female"
    pub gender: String,
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's country
    pub country_code: String,
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's residence country
    pub residence_country_code: String,
}
