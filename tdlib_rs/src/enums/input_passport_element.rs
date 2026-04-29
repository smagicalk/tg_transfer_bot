#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPassportElement {
    /// A Telegram Passport element to be saved containing the user's personal details
    #[serde(rename(serialize = "inputPassportElementPersonalDetails", deserialize = "inputPassportElementPersonalDetails"))]
    PersonalDetails(crate::types::InputPassportElementPersonalDetails),
    /// A Telegram Passport element to be saved containing the user's passport
    #[serde(rename(serialize = "inputPassportElementPassport", deserialize = "inputPassportElementPassport"))]
    Passport(crate::types::InputPassportElementPassport),
    /// A Telegram Passport element to be saved containing the user's driver license
    #[serde(rename(serialize = "inputPassportElementDriverLicense", deserialize = "inputPassportElementDriverLicense"))]
    DriverLicense(crate::types::InputPassportElementDriverLicense),
    /// A Telegram Passport element to be saved containing the user's identity card
    #[serde(rename(serialize = "inputPassportElementIdentityCard", deserialize = "inputPassportElementIdentityCard"))]
    IdentityCard(crate::types::InputPassportElementIdentityCard),
    /// A Telegram Passport element to be saved containing the user's internal passport
    #[serde(rename(serialize = "inputPassportElementInternalPassport", deserialize = "inputPassportElementInternalPassport"))]
    InternalPassport(crate::types::InputPassportElementInternalPassport),
    /// A Telegram Passport element to be saved containing the user's address
    #[serde(rename(serialize = "inputPassportElementAddress", deserialize = "inputPassportElementAddress"))]
    Address(crate::types::InputPassportElementAddress),
    /// A Telegram Passport element to be saved containing the user's utility bill
    #[serde(rename(serialize = "inputPassportElementUtilityBill", deserialize = "inputPassportElementUtilityBill"))]
    UtilityBill(crate::types::InputPassportElementUtilityBill),
    /// A Telegram Passport element to be saved containing the user's bank statement
    #[serde(rename(serialize = "inputPassportElementBankStatement", deserialize = "inputPassportElementBankStatement"))]
    BankStatement(crate::types::InputPassportElementBankStatement),
    /// A Telegram Passport element to be saved containing the user's rental agreement
    #[serde(rename(serialize = "inputPassportElementRentalAgreement", deserialize = "inputPassportElementRentalAgreement"))]
    RentalAgreement(crate::types::InputPassportElementRentalAgreement),
    /// A Telegram Passport element to be saved containing the user's passport registration
    #[serde(rename(serialize = "inputPassportElementPassportRegistration", deserialize = "inputPassportElementPassportRegistration"))]
    PassportRegistration(crate::types::InputPassportElementPassportRegistration),
    /// A Telegram Passport element to be saved containing the user's temporary registration
    #[serde(rename(serialize = "inputPassportElementTemporaryRegistration", deserialize = "inputPassportElementTemporaryRegistration"))]
    TemporaryRegistration(crate::types::InputPassportElementTemporaryRegistration),
    /// A Telegram Passport element to be saved containing the user's phone number
    #[serde(rename(serialize = "inputPassportElementPhoneNumber", deserialize = "inputPassportElementPhoneNumber"))]
    PhoneNumber(crate::types::InputPassportElementPhoneNumber),
    /// A Telegram Passport element to be saved containing the user's email address
    #[serde(rename(serialize = "inputPassportElementEmailAddress", deserialize = "inputPassportElementEmailAddress"))]
    EmailAddress(crate::types::InputPassportElementEmailAddress),
}
