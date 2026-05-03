#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementType {
    /// A Telegram Passport element containing the user's personal details
    #[serde(rename(
        serialize = "passportElementTypePersonalDetails",
        deserialize = "passportElementTypePersonalDetails"
    ))]
    PersonalDetails,
    /// A Telegram Passport element containing the user's passport
    #[serde(rename(
        serialize = "passportElementTypePassport",
        deserialize = "passportElementTypePassport"
    ))]
    Passport,
    /// A Telegram Passport element containing the user's driver license
    #[serde(rename(
        serialize = "passportElementTypeDriverLicense",
        deserialize = "passportElementTypeDriverLicense"
    ))]
    DriverLicense,
    /// A Telegram Passport element containing the user's identity card
    #[serde(rename(
        serialize = "passportElementTypeIdentityCard",
        deserialize = "passportElementTypeIdentityCard"
    ))]
    IdentityCard,
    /// A Telegram Passport element containing the user's internal passport
    #[serde(rename(
        serialize = "passportElementTypeInternalPassport",
        deserialize = "passportElementTypeInternalPassport"
    ))]
    InternalPassport,
    /// A Telegram Passport element containing the user's address
    #[serde(rename(
        serialize = "passportElementTypeAddress",
        deserialize = "passportElementTypeAddress"
    ))]
    Address,
    /// A Telegram Passport element containing the user's utility bill
    #[serde(rename(
        serialize = "passportElementTypeUtilityBill",
        deserialize = "passportElementTypeUtilityBill"
    ))]
    UtilityBill,
    /// A Telegram Passport element containing the user's bank statement
    #[serde(rename(
        serialize = "passportElementTypeBankStatement",
        deserialize = "passportElementTypeBankStatement"
    ))]
    BankStatement,
    /// A Telegram Passport element containing the user's rental agreement
    #[serde(rename(
        serialize = "passportElementTypeRentalAgreement",
        deserialize = "passportElementTypeRentalAgreement"
    ))]
    RentalAgreement,
    /// A Telegram Passport element containing the registration page of the user's passport
    #[serde(rename(
        serialize = "passportElementTypePassportRegistration",
        deserialize = "passportElementTypePassportRegistration"
    ))]
    PassportRegistration,
    /// A Telegram Passport element containing the user's temporary registration
    #[serde(rename(
        serialize = "passportElementTypeTemporaryRegistration",
        deserialize = "passportElementTypeTemporaryRegistration"
    ))]
    TemporaryRegistration,
    /// A Telegram Passport element containing the user's phone number
    #[serde(rename(
        serialize = "passportElementTypePhoneNumber",
        deserialize = "passportElementTypePhoneNumber"
    ))]
    PhoneNumber,
    /// A Telegram Passport element containing the user's email address
    #[serde(rename(
        serialize = "passportElementTypeEmailAddress",
        deserialize = "passportElementTypeEmailAddress"
    ))]
    EmailAddress,
}
