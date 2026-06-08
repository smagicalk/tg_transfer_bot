#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElement {
    /// A Telegram Passport element containing the user's personal details
    #[serde(rename(
        serialize = "passportElementPersonalDetails",
        deserialize = "passportElementPersonalDetails"
    ))]
    PersonalDetails(crate::types::PassportElementPersonalDetails),
    /// A Telegram Passport element containing the user's passport
    #[serde(rename(
        serialize = "passportElementPassport",
        deserialize = "passportElementPassport"
    ))]
    Passport(crate::types::PassportElementPassport),
    /// A Telegram Passport element containing the user's driver license
    #[serde(rename(
        serialize = "passportElementDriverLicense",
        deserialize = "passportElementDriverLicense"
    ))]
    DriverLicense(crate::types::PassportElementDriverLicense),
    /// A Telegram Passport element containing the user's identity card
    #[serde(rename(
        serialize = "passportElementIdentityCard",
        deserialize = "passportElementIdentityCard"
    ))]
    IdentityCard(crate::types::PassportElementIdentityCard),
    /// A Telegram Passport element containing the user's internal passport
    #[serde(rename(
        serialize = "passportElementInternalPassport",
        deserialize = "passportElementInternalPassport"
    ))]
    InternalPassport(crate::types::PassportElementInternalPassport),
    /// A Telegram Passport element containing the user's address
    #[serde(rename(
        serialize = "passportElementAddress",
        deserialize = "passportElementAddress"
    ))]
    Address(crate::types::PassportElementAddress),
    /// A Telegram Passport element containing the user's utility bill
    #[serde(rename(
        serialize = "passportElementUtilityBill",
        deserialize = "passportElementUtilityBill"
    ))]
    UtilityBill(crate::types::PassportElementUtilityBill),
    /// A Telegram Passport element containing the user's bank statement
    #[serde(rename(
        serialize = "passportElementBankStatement",
        deserialize = "passportElementBankStatement"
    ))]
    BankStatement(crate::types::PassportElementBankStatement),
    /// A Telegram Passport element containing the user's rental agreement
    #[serde(rename(
        serialize = "passportElementRentalAgreement",
        deserialize = "passportElementRentalAgreement"
    ))]
    RentalAgreement(crate::types::PassportElementRentalAgreement),
    /// A Telegram Passport element containing the user's passport registration pages
    #[serde(rename(
        serialize = "passportElementPassportRegistration",
        deserialize = "passportElementPassportRegistration"
    ))]
    PassportRegistration(crate::types::PassportElementPassportRegistration),
    /// A Telegram Passport element containing the user's temporary registration
    #[serde(rename(
        serialize = "passportElementTemporaryRegistration",
        deserialize = "passportElementTemporaryRegistration"
    ))]
    TemporaryRegistration(crate::types::PassportElementTemporaryRegistration),
    /// A Telegram Passport element containing the user's phone number
    #[serde(rename(
        serialize = "passportElementPhoneNumber",
        deserialize = "passportElementPhoneNumber"
    ))]
    PhoneNumber(crate::types::PassportElementPhoneNumber),
    /// A Telegram Passport element containing the user's email address
    #[serde(rename(
        serialize = "passportElementEmailAddress",
        deserialize = "passportElementEmailAddress"
    ))]
    EmailAddress(crate::types::PassportElementEmailAddress),
}
