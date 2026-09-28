//!
//! TDLib `passport` domain enums.
//!
//! Types, enums, and functions for Telegram Passport and identity verification documents.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// Contains the type of Telegram Passport element
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementType {
    /// A Telegram Passport element containing the user's personal details
    #[serde(rename(serialize = "passportElementTypePersonalDetails", deserialize = "passportElementTypePersonalDetails"))]
    PersonalDetails,
    /// A Telegram Passport element containing the user's passport
    #[serde(rename(serialize = "passportElementTypePassport", deserialize = "passportElementTypePassport"))]
    Passport,
    /// A Telegram Passport element containing the user's driver license
    #[serde(rename(serialize = "passportElementTypeDriverLicense", deserialize = "passportElementTypeDriverLicense"))]
    DriverLicense,
    /// A Telegram Passport element containing the user's identity card
    #[serde(rename(serialize = "passportElementTypeIdentityCard", deserialize = "passportElementTypeIdentityCard"))]
    IdentityCard,
    /// A Telegram Passport element containing the user's internal passport
    #[serde(rename(serialize = "passportElementTypeInternalPassport", deserialize = "passportElementTypeInternalPassport"))]
    InternalPassport,
    /// A Telegram Passport element containing the user's address
    #[serde(rename(serialize = "passportElementTypeAddress", deserialize = "passportElementTypeAddress"))]
    Address,
    /// A Telegram Passport element containing the user's utility bill
    #[serde(rename(serialize = "passportElementTypeUtilityBill", deserialize = "passportElementTypeUtilityBill"))]
    UtilityBill,
    /// A Telegram Passport element containing the user's bank statement
    #[serde(rename(serialize = "passportElementTypeBankStatement", deserialize = "passportElementTypeBankStatement"))]
    BankStatement,
    /// A Telegram Passport element containing the user's rental agreement
    #[serde(rename(serialize = "passportElementTypeRentalAgreement", deserialize = "passportElementTypeRentalAgreement"))]
    RentalAgreement,
    /// A Telegram Passport element containing the registration page of the user's passport
    #[serde(rename(serialize = "passportElementTypePassportRegistration", deserialize = "passportElementTypePassportRegistration"))]
    PassportRegistration,
    /// A Telegram Passport element containing the user's temporary registration
    #[serde(rename(serialize = "passportElementTypeTemporaryRegistration", deserialize = "passportElementTypeTemporaryRegistration"))]
    TemporaryRegistration,
    /// A Telegram Passport element containing the user's phone number
    #[serde(rename(serialize = "passportElementTypePhoneNumber", deserialize = "passportElementTypePhoneNumber"))]
    PhoneNumber,
    /// A Telegram Passport element containing the user's email address
    #[serde(rename(serialize = "passportElementTypeEmailAddress", deserialize = "passportElementTypeEmailAddress"))]
    EmailAddress,
}

/// Contains information about a Telegram Passport element
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElement {
    /// A Telegram Passport element containing the user's personal details
    #[serde(rename(serialize = "passportElementPersonalDetails", deserialize = "passportElementPersonalDetails"))]
    PersonalDetails(Box<crate::types::PassportElementPersonalDetails>),
    /// A Telegram Passport element containing the user's passport
    #[serde(rename(serialize = "passportElementPassport", deserialize = "passportElementPassport"))]
    Passport(Box<crate::types::PassportElementPassport>),
    /// A Telegram Passport element containing the user's driver license
    #[serde(rename(serialize = "passportElementDriverLicense", deserialize = "passportElementDriverLicense"))]
    DriverLicense(Box<crate::types::PassportElementDriverLicense>),
    /// A Telegram Passport element containing the user's identity card
    #[serde(rename(serialize = "passportElementIdentityCard", deserialize = "passportElementIdentityCard"))]
    IdentityCard(Box<crate::types::PassportElementIdentityCard>),
    /// A Telegram Passport element containing the user's internal passport
    #[serde(rename(serialize = "passportElementInternalPassport", deserialize = "passportElementInternalPassport"))]
    InternalPassport(Box<crate::types::PassportElementInternalPassport>),
    /// A Telegram Passport element containing the user's address
    #[serde(rename(serialize = "passportElementAddress", deserialize = "passportElementAddress"))]
    Address(Box<crate::types::PassportElementAddress>),
    /// A Telegram Passport element containing the user's utility bill
    #[serde(rename(serialize = "passportElementUtilityBill", deserialize = "passportElementUtilityBill"))]
    UtilityBill(Box<crate::types::PassportElementUtilityBill>),
    /// A Telegram Passport element containing the user's bank statement
    #[serde(rename(serialize = "passportElementBankStatement", deserialize = "passportElementBankStatement"))]
    BankStatement(Box<crate::types::PassportElementBankStatement>),
    /// A Telegram Passport element containing the user's rental agreement
    #[serde(rename(serialize = "passportElementRentalAgreement", deserialize = "passportElementRentalAgreement"))]
    RentalAgreement(Box<crate::types::PassportElementRentalAgreement>),
    /// A Telegram Passport element containing the user's passport registration pages
    #[serde(rename(serialize = "passportElementPassportRegistration", deserialize = "passportElementPassportRegistration"))]
    PassportRegistration(Box<crate::types::PassportElementPassportRegistration>),
    /// A Telegram Passport element containing the user's temporary registration
    #[serde(rename(serialize = "passportElementTemporaryRegistration", deserialize = "passportElementTemporaryRegistration"))]
    TemporaryRegistration(Box<crate::types::PassportElementTemporaryRegistration>),
    /// A Telegram Passport element containing the user's phone number
    #[serde(rename(serialize = "passportElementPhoneNumber", deserialize = "passportElementPhoneNumber"))]
    PhoneNumber(Box<crate::types::PassportElementPhoneNumber>),
    /// A Telegram Passport element containing the user's email address
    #[serde(rename(serialize = "passportElementEmailAddress", deserialize = "passportElementEmailAddress"))]
    EmailAddress(Box<crate::types::PassportElementEmailAddress>),
}

impl PassportElement {
    /// Convenience constructor to create a [`PassportElement::PersonalDetails`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn personal_details(val: crate::types::PassportElementPersonalDetails) -> Self {
        Self::PersonalDetails(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::Passport`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport(val: crate::types::PassportElementPassport) -> Self {
        Self::Passport(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::DriverLicense`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn driver_license(val: crate::types::PassportElementDriverLicense) -> Self {
        Self::DriverLicense(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::IdentityCard`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn identity_card(val: crate::types::PassportElementIdentityCard) -> Self {
        Self::IdentityCard(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::InternalPassport`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn internal_passport(val: crate::types::PassportElementInternalPassport) -> Self {
        Self::InternalPassport(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::Address`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn address(val: crate::types::PassportElementAddress) -> Self {
        Self::Address(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::UtilityBill`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn utility_bill(val: crate::types::PassportElementUtilityBill) -> Self {
        Self::UtilityBill(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::BankStatement`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bank_statement(val: crate::types::PassportElementBankStatement) -> Self {
        Self::BankStatement(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::RentalAgreement`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn rental_agreement(val: crate::types::PassportElementRentalAgreement) -> Self {
        Self::RentalAgreement(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::PassportRegistration`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_registration(val: crate::types::PassportElementPassportRegistration) -> Self {
        Self::PassportRegistration(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::TemporaryRegistration`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn temporary_registration(val: crate::types::PassportElementTemporaryRegistration) -> Self {
        Self::TemporaryRegistration(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::PhoneNumber`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn phone_number(val: crate::types::PassportElementPhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElement::EmailAddress`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn email_address(val: crate::types::PassportElementEmailAddress) -> Self {
        Self::EmailAddress(Box::new(val))
    }

}

/// Converts a [`crate::types::PassportElementPersonalDetails`] into [`PassportElement`].
impl From<crate::types::PassportElementPersonalDetails> for PassportElement {
    fn from(val: crate::types::PassportElementPersonalDetails) -> Self {
        Self::PersonalDetails(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementPassport`] into [`PassportElement`].
impl From<crate::types::PassportElementPassport> for PassportElement {
    fn from(val: crate::types::PassportElementPassport) -> Self {
        Self::Passport(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementDriverLicense`] into [`PassportElement`].
impl From<crate::types::PassportElementDriverLicense> for PassportElement {
    fn from(val: crate::types::PassportElementDriverLicense) -> Self {
        Self::DriverLicense(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementIdentityCard`] into [`PassportElement`].
impl From<crate::types::PassportElementIdentityCard> for PassportElement {
    fn from(val: crate::types::PassportElementIdentityCard) -> Self {
        Self::IdentityCard(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementInternalPassport`] into [`PassportElement`].
impl From<crate::types::PassportElementInternalPassport> for PassportElement {
    fn from(val: crate::types::PassportElementInternalPassport) -> Self {
        Self::InternalPassport(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementAddress`] into [`PassportElement`].
impl From<crate::types::PassportElementAddress> for PassportElement {
    fn from(val: crate::types::PassportElementAddress) -> Self {
        Self::Address(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementUtilityBill`] into [`PassportElement`].
impl From<crate::types::PassportElementUtilityBill> for PassportElement {
    fn from(val: crate::types::PassportElementUtilityBill) -> Self {
        Self::UtilityBill(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementBankStatement`] into [`PassportElement`].
impl From<crate::types::PassportElementBankStatement> for PassportElement {
    fn from(val: crate::types::PassportElementBankStatement) -> Self {
        Self::BankStatement(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementRentalAgreement`] into [`PassportElement`].
impl From<crate::types::PassportElementRentalAgreement> for PassportElement {
    fn from(val: crate::types::PassportElementRentalAgreement) -> Self {
        Self::RentalAgreement(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementPassportRegistration`] into [`PassportElement`].
impl From<crate::types::PassportElementPassportRegistration> for PassportElement {
    fn from(val: crate::types::PassportElementPassportRegistration) -> Self {
        Self::PassportRegistration(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementTemporaryRegistration`] into [`PassportElement`].
impl From<crate::types::PassportElementTemporaryRegistration> for PassportElement {
    fn from(val: crate::types::PassportElementTemporaryRegistration) -> Self {
        Self::TemporaryRegistration(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementPhoneNumber`] into [`PassportElement`].
impl From<crate::types::PassportElementPhoneNumber> for PassportElement {
    fn from(val: crate::types::PassportElementPhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementEmailAddress`] into [`PassportElement`].
impl From<crate::types::PassportElementEmailAddress> for PassportElement {
    fn from(val: crate::types::PassportElementEmailAddress) -> Self {
        Self::EmailAddress(Box::new(val))
    }
}

/// Contains information about a Telegram Passport element to be saved
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPassportElement {
    /// A Telegram Passport element to be saved containing the user's personal details
    #[serde(rename(serialize = "inputPassportElementPersonalDetails", deserialize = "inputPassportElementPersonalDetails"))]
    PersonalDetails(Box<crate::types::InputPassportElementPersonalDetails>),
    /// A Telegram Passport element to be saved containing the user's passport
    #[serde(rename(serialize = "inputPassportElementPassport", deserialize = "inputPassportElementPassport"))]
    Passport(Box<crate::types::InputPassportElementPassport>),
    /// A Telegram Passport element to be saved containing the user's driver license
    #[serde(rename(serialize = "inputPassportElementDriverLicense", deserialize = "inputPassportElementDriverLicense"))]
    DriverLicense(Box<crate::types::InputPassportElementDriverLicense>),
    /// A Telegram Passport element to be saved containing the user's identity card
    #[serde(rename(serialize = "inputPassportElementIdentityCard", deserialize = "inputPassportElementIdentityCard"))]
    IdentityCard(Box<crate::types::InputPassportElementIdentityCard>),
    /// A Telegram Passport element to be saved containing the user's internal passport
    #[serde(rename(serialize = "inputPassportElementInternalPassport", deserialize = "inputPassportElementInternalPassport"))]
    InternalPassport(Box<crate::types::InputPassportElementInternalPassport>),
    /// A Telegram Passport element to be saved containing the user's address
    #[serde(rename(serialize = "inputPassportElementAddress", deserialize = "inputPassportElementAddress"))]
    Address(Box<crate::types::InputPassportElementAddress>),
    /// A Telegram Passport element to be saved containing the user's utility bill
    #[serde(rename(serialize = "inputPassportElementUtilityBill", deserialize = "inputPassportElementUtilityBill"))]
    UtilityBill(Box<crate::types::InputPassportElementUtilityBill>),
    /// A Telegram Passport element to be saved containing the user's bank statement
    #[serde(rename(serialize = "inputPassportElementBankStatement", deserialize = "inputPassportElementBankStatement"))]
    BankStatement(Box<crate::types::InputPassportElementBankStatement>),
    /// A Telegram Passport element to be saved containing the user's rental agreement
    #[serde(rename(serialize = "inputPassportElementRentalAgreement", deserialize = "inputPassportElementRentalAgreement"))]
    RentalAgreement(Box<crate::types::InputPassportElementRentalAgreement>),
    /// A Telegram Passport element to be saved containing the user's passport registration
    #[serde(rename(serialize = "inputPassportElementPassportRegistration", deserialize = "inputPassportElementPassportRegistration"))]
    PassportRegistration(Box<crate::types::InputPassportElementPassportRegistration>),
    /// A Telegram Passport element to be saved containing the user's temporary registration
    #[serde(rename(serialize = "inputPassportElementTemporaryRegistration", deserialize = "inputPassportElementTemporaryRegistration"))]
    TemporaryRegistration(Box<crate::types::InputPassportElementTemporaryRegistration>),
    /// A Telegram Passport element to be saved containing the user's phone number
    #[serde(rename(serialize = "inputPassportElementPhoneNumber", deserialize = "inputPassportElementPhoneNumber"))]
    PhoneNumber(Box<crate::types::InputPassportElementPhoneNumber>),
    /// A Telegram Passport element to be saved containing the user's email address
    #[serde(rename(serialize = "inputPassportElementEmailAddress", deserialize = "inputPassportElementEmailAddress"))]
    EmailAddress(Box<crate::types::InputPassportElementEmailAddress>),
}

impl InputPassportElement {
    /// Convenience constructor to create a [`InputPassportElement::PersonalDetails`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn personal_details(val: crate::types::InputPassportElementPersonalDetails) -> Self {
        Self::PersonalDetails(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::Passport`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport(val: crate::types::InputPassportElementPassport) -> Self {
        Self::Passport(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::DriverLicense`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn driver_license(val: crate::types::InputPassportElementDriverLicense) -> Self {
        Self::DriverLicense(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::IdentityCard`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn identity_card(val: crate::types::InputPassportElementIdentityCard) -> Self {
        Self::IdentityCard(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::InternalPassport`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn internal_passport(val: crate::types::InputPassportElementInternalPassport) -> Self {
        Self::InternalPassport(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::Address`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn address(val: crate::types::InputPassportElementAddress) -> Self {
        Self::Address(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::UtilityBill`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn utility_bill(val: crate::types::InputPassportElementUtilityBill) -> Self {
        Self::UtilityBill(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::BankStatement`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bank_statement(val: crate::types::InputPassportElementBankStatement) -> Self {
        Self::BankStatement(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::RentalAgreement`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn rental_agreement(val: crate::types::InputPassportElementRentalAgreement) -> Self {
        Self::RentalAgreement(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::PassportRegistration`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_registration(val: crate::types::InputPassportElementPassportRegistration) -> Self {
        Self::PassportRegistration(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::TemporaryRegistration`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn temporary_registration(val: crate::types::InputPassportElementTemporaryRegistration) -> Self {
        Self::TemporaryRegistration(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::PhoneNumber`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn phone_number(val: crate::types::InputPassportElementPhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElement::EmailAddress`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn email_address(val: crate::types::InputPassportElementEmailAddress) -> Self {
        Self::EmailAddress(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPassportElementPersonalDetails`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementPersonalDetails> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementPersonalDetails) -> Self {
        Self::PersonalDetails(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementPassport`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementPassport> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementPassport) -> Self {
        Self::Passport(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementDriverLicense`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementDriverLicense> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementDriverLicense) -> Self {
        Self::DriverLicense(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementIdentityCard`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementIdentityCard> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementIdentityCard) -> Self {
        Self::IdentityCard(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementInternalPassport`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementInternalPassport> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementInternalPassport) -> Self {
        Self::InternalPassport(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementAddress`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementAddress> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementAddress) -> Self {
        Self::Address(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementUtilityBill`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementUtilityBill> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementUtilityBill) -> Self {
        Self::UtilityBill(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementBankStatement`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementBankStatement> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementBankStatement) -> Self {
        Self::BankStatement(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementRentalAgreement`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementRentalAgreement> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementRentalAgreement) -> Self {
        Self::RentalAgreement(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementPassportRegistration`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementPassportRegistration> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementPassportRegistration) -> Self {
        Self::PassportRegistration(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementTemporaryRegistration`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementTemporaryRegistration> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementTemporaryRegistration) -> Self {
        Self::TemporaryRegistration(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementPhoneNumber`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementPhoneNumber> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementPhoneNumber) -> Self {
        Self::PhoneNumber(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementEmailAddress`] into [`InputPassportElement`].
impl From<crate::types::InputPassportElementEmailAddress> for InputPassportElement {
    fn from(val: crate::types::InputPassportElementEmailAddress) -> Self {
        Self::EmailAddress(Box::new(val))
    }
}

/// TDLib `PassportElements` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElements {
    /// Contains information about saved Telegram Passport elements
    #[serde(rename(serialize = "passportElements", deserialize = "passportElements"))]
    PassportElements(Box<crate::types::PassportElements>),
}

impl PassportElements {
    /// Convenience constructor to create a [`PassportElements::PassportElements`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_elements(val: crate::types::PassportElements) -> Self {
        Self::PassportElements(Box::new(val))
    }

}

/// Converts a [`crate::types::PassportElements`] into [`PassportElements`].
impl From<crate::types::PassportElements> for PassportElements {
    fn from(val: crate::types::PassportElements) -> Self {
        Self::PassportElements(Box::new(val))
    }
}

/// Contains the description of an error in a Telegram Passport element
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementErrorSource {
    /// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
    #[serde(rename(serialize = "passportElementErrorSourceUnspecified", deserialize = "passportElementErrorSourceUnspecified"))]
    Unspecified,
    /// One of the data fields contains an error. The error will be considered resolved when the value of the field changes
    #[serde(rename(serialize = "passportElementErrorSourceDataField", deserialize = "passportElementErrorSourceDataField"))]
    DataField(Box<crate::types::PassportElementErrorSourceDataField>),
    /// The front side of the document contains an error. The error will be considered resolved when the file with the front side changes
    #[serde(rename(serialize = "passportElementErrorSourceFrontSide", deserialize = "passportElementErrorSourceFrontSide"))]
    FrontSide,
    /// The reverse side of the document contains an error. The error will be considered resolved when the file with the reverse side changes
    #[serde(rename(serialize = "passportElementErrorSourceReverseSide", deserialize = "passportElementErrorSourceReverseSide"))]
    ReverseSide,
    /// The selfie with the document contains an error. The error will be considered resolved when the file with the selfie changes
    #[serde(rename(serialize = "passportElementErrorSourceSelfie", deserialize = "passportElementErrorSourceSelfie"))]
    Selfie,
    /// One of files with the translation of the document contains an error. The error will be considered resolved when the file changes
    #[serde(rename(serialize = "passportElementErrorSourceTranslationFile", deserialize = "passportElementErrorSourceTranslationFile"))]
    TranslationFile(Box<crate::types::PassportElementErrorSourceTranslationFile>),
    /// The translation of the document contains an error. The error will be considered resolved when the list of translation files changes
    #[serde(rename(serialize = "passportElementErrorSourceTranslationFiles", deserialize = "passportElementErrorSourceTranslationFiles"))]
    TranslationFiles,
    /// The file contains an error. The error will be considered resolved when the file changes
    #[serde(rename(serialize = "passportElementErrorSourceFile", deserialize = "passportElementErrorSourceFile"))]
    File(Box<crate::types::PassportElementErrorSourceFile>),
    /// The list of attached files contains an error. The error will be considered resolved when the list of files changes
    #[serde(rename(serialize = "passportElementErrorSourceFiles", deserialize = "passportElementErrorSourceFiles"))]
    Files,
}

impl PassportElementErrorSource {
    /// Convenience constructor to create a [`PassportElementErrorSource::DataField`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn data_field(val: crate::types::PassportElementErrorSourceDataField) -> Self {
        Self::DataField(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElementErrorSource::TranslationFile`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn translation_file(val: crate::types::PassportElementErrorSourceTranslationFile) -> Self {
        Self::TranslationFile(Box::new(val))
    }

    /// Convenience constructor to create a [`PassportElementErrorSource::File`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file(val: crate::types::PassportElementErrorSourceFile) -> Self {
        Self::File(Box::new(val))
    }

}

/// Converts a [`crate::types::PassportElementErrorSourceDataField`] into [`PassportElementErrorSource`].
impl From<crate::types::PassportElementErrorSourceDataField> for PassportElementErrorSource {
    fn from(val: crate::types::PassportElementErrorSourceDataField) -> Self {
        Self::DataField(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementErrorSourceTranslationFile`] into [`PassportElementErrorSource`].
impl From<crate::types::PassportElementErrorSourceTranslationFile> for PassportElementErrorSource {
    fn from(val: crate::types::PassportElementErrorSourceTranslationFile) -> Self {
        Self::TranslationFile(Box::new(val))
    }
}

/// Converts a [`crate::types::PassportElementErrorSourceFile`] into [`PassportElementErrorSource`].
impl From<crate::types::PassportElementErrorSourceFile> for PassportElementErrorSource {
    fn from(val: crate::types::PassportElementErrorSourceFile) -> Self {
        Self::File(Box::new(val))
    }
}

/// TDLib `PassportElementError` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementError {
    /// Contains the description of an error in a Telegram Passport element
    #[serde(rename(serialize = "passportElementError", deserialize = "passportElementError"))]
    PassportElementError(Box<crate::types::PassportElementError>),
}

impl PassportElementError {
    /// Convenience constructor to create a [`PassportElementError::PassportElementError`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_element_error(val: crate::types::PassportElementError) -> Self {
        Self::PassportElementError(Box::new(val))
    }

}

/// Converts a [`crate::types::PassportElementError`] into [`PassportElementError`].
impl From<crate::types::PassportElementError> for PassportElementError {
    fn from(val: crate::types::PassportElementError) -> Self {
        Self::PassportElementError(Box::new(val))
    }
}

/// TDLib `PassportSuitableElement` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportSuitableElement {
    /// Contains information about a Telegram Passport element that was requested by a service
    #[serde(rename(serialize = "passportSuitableElement", deserialize = "passportSuitableElement"))]
    PassportSuitableElement(Box<crate::types::PassportSuitableElement>),
}

impl PassportSuitableElement {
    /// Convenience constructor to create a [`PassportSuitableElement::PassportSuitableElement`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_suitable_element(val: crate::types::PassportSuitableElement) -> Self {
        Self::PassportSuitableElement(Box::new(val))
    }

}

/// Converts a [`crate::types::PassportSuitableElement`] into [`PassportSuitableElement`].
impl From<crate::types::PassportSuitableElement> for PassportSuitableElement {
    fn from(val: crate::types::PassportSuitableElement) -> Self {
        Self::PassportSuitableElement(Box::new(val))
    }
}

/// TDLib `PassportRequiredElement` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportRequiredElement {
    /// Contains a description of the required Telegram Passport element that was requested by a service
    #[serde(rename(serialize = "passportRequiredElement", deserialize = "passportRequiredElement"))]
    PassportRequiredElement(Box<crate::types::PassportRequiredElement>),
}

impl PassportRequiredElement {
    /// Convenience constructor to create a [`PassportRequiredElement::PassportRequiredElement`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_required_element(val: crate::types::PassportRequiredElement) -> Self {
        Self::PassportRequiredElement(Box::new(val))
    }

}

/// Converts a [`crate::types::PassportRequiredElement`] into [`PassportRequiredElement`].
impl From<crate::types::PassportRequiredElement> for PassportRequiredElement {
    fn from(val: crate::types::PassportRequiredElement) -> Self {
        Self::PassportRequiredElement(Box::new(val))
    }
}

/// TDLib `PassportAuthorizationForm` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportAuthorizationForm {
    /// Contains information about a Telegram Passport authorization form that was requested
    #[serde(rename(serialize = "passportAuthorizationForm", deserialize = "passportAuthorizationForm"))]
    PassportAuthorizationForm(Box<crate::types::PassportAuthorizationForm>),
}

impl PassportAuthorizationForm {
    /// Convenience constructor to create a [`PassportAuthorizationForm::PassportAuthorizationForm`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_authorization_form(val: crate::types::PassportAuthorizationForm) -> Self {
        Self::PassportAuthorizationForm(Box::new(val))
    }

}

/// Converts a [`crate::types::PassportAuthorizationForm`] into [`PassportAuthorizationForm`].
impl From<crate::types::PassportAuthorizationForm> for PassportAuthorizationForm {
    fn from(val: crate::types::PassportAuthorizationForm) -> Self {
        Self::PassportAuthorizationForm(Box::new(val))
    }
}

/// TDLib `PassportElementsWithErrors` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementsWithErrors {
    /// Contains information about a Telegram Passport elements and corresponding errors
    #[serde(rename(serialize = "passportElementsWithErrors", deserialize = "passportElementsWithErrors"))]
    PassportElementsWithErrors(Box<crate::types::PassportElementsWithErrors>),
}

impl PassportElementsWithErrors {
    /// Convenience constructor to create a [`PassportElementsWithErrors::PassportElementsWithErrors`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn passport_elements_with_errors(val: crate::types::PassportElementsWithErrors) -> Self {
        Self::PassportElementsWithErrors(Box::new(val))
    }

}

/// Converts a [`crate::types::PassportElementsWithErrors`] into [`PassportElementsWithErrors`].
impl From<crate::types::PassportElementsWithErrors> for PassportElementsWithErrors {
    fn from(val: crate::types::PassportElementsWithErrors) -> Self {
        Self::PassportElementsWithErrors(Box::new(val))
    }
}

/// TDLib `EncryptedPassportElement` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum EncryptedPassportElement {
    /// Contains information about an encrypted Telegram Passport element; for bots only
    #[serde(rename(serialize = "encryptedPassportElement", deserialize = "encryptedPassportElement"))]
    EncryptedPassportElement(Box<crate::types::EncryptedPassportElement>),
}

impl EncryptedPassportElement {
    /// Convenience constructor to create a [`EncryptedPassportElement::EncryptedPassportElement`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn encrypted_passport_element(val: crate::types::EncryptedPassportElement) -> Self {
        Self::EncryptedPassportElement(Box::new(val))
    }

}

/// Converts a [`crate::types::EncryptedPassportElement`] into [`EncryptedPassportElement`].
impl From<crate::types::EncryptedPassportElement> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElement) -> Self {
        Self::EncryptedPassportElement(Box::new(val))
    }
}

/// Contains the description of an error in a Telegram Passport element; for bots only
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPassportElementErrorSource {
    /// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
    #[serde(rename(serialize = "inputPassportElementErrorSourceUnspecified", deserialize = "inputPassportElementErrorSourceUnspecified"))]
    Unspecified(Box<crate::types::InputPassportElementErrorSourceUnspecified>),
    /// A data field contains an error. The error is considered resolved when the field's value changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceDataField", deserialize = "inputPassportElementErrorSourceDataField"))]
    DataField(Box<crate::types::InputPassportElementErrorSourceDataField>),
    /// The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceFrontSide", deserialize = "inputPassportElementErrorSourceFrontSide"))]
    FrontSide(Box<crate::types::InputPassportElementErrorSourceFrontSide>),
    /// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceReverseSide", deserialize = "inputPassportElementErrorSourceReverseSide"))]
    ReverseSide(Box<crate::types::InputPassportElementErrorSourceReverseSide>),
    /// The selfie contains an error. The error is considered resolved when the file with the selfie changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceSelfie", deserialize = "inputPassportElementErrorSourceSelfie"))]
    Selfie(Box<crate::types::InputPassportElementErrorSourceSelfie>),
    /// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceTranslationFile", deserialize = "inputPassportElementErrorSourceTranslationFile"))]
    TranslationFile(Box<crate::types::InputPassportElementErrorSourceTranslationFile>),
    /// The translation of the document contains an error. The error is considered resolved when the list of files changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceTranslationFiles", deserialize = "inputPassportElementErrorSourceTranslationFiles"))]
    TranslationFiles(Box<crate::types::InputPassportElementErrorSourceTranslationFiles>),
    /// The file contains an error. The error is considered resolved when the file changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceFile", deserialize = "inputPassportElementErrorSourceFile"))]
    File(Box<crate::types::InputPassportElementErrorSourceFile>),
    /// The list of attached files contains an error. The error is considered resolved when the file list changes
    #[serde(rename(serialize = "inputPassportElementErrorSourceFiles", deserialize = "inputPassportElementErrorSourceFiles"))]
    Files(Box<crate::types::InputPassportElementErrorSourceFiles>),
}

impl InputPassportElementErrorSource {
    /// Convenience constructor to create a [`InputPassportElementErrorSource::Unspecified`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn unspecified(val: crate::types::InputPassportElementErrorSourceUnspecified) -> Self {
        Self::Unspecified(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElementErrorSource::DataField`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn data_field(val: crate::types::InputPassportElementErrorSourceDataField) -> Self {
        Self::DataField(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElementErrorSource::FrontSide`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn front_side(val: crate::types::InputPassportElementErrorSourceFrontSide) -> Self {
        Self::FrontSide(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElementErrorSource::ReverseSide`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn reverse_side(val: crate::types::InputPassportElementErrorSourceReverseSide) -> Self {
        Self::ReverseSide(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElementErrorSource::Selfie`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn selfie(val: crate::types::InputPassportElementErrorSourceSelfie) -> Self {
        Self::Selfie(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElementErrorSource::TranslationFile`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn translation_file(val: crate::types::InputPassportElementErrorSourceTranslationFile) -> Self {
        Self::TranslationFile(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElementErrorSource::TranslationFiles`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn translation_files(val: crate::types::InputPassportElementErrorSourceTranslationFiles) -> Self {
        Self::TranslationFiles(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElementErrorSource::File`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn file(val: crate::types::InputPassportElementErrorSourceFile) -> Self {
        Self::File(Box::new(val))
    }

    /// Convenience constructor to create a [`InputPassportElementErrorSource::Files`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn files(val: crate::types::InputPassportElementErrorSourceFiles) -> Self {
        Self::Files(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPassportElementErrorSourceUnspecified`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceUnspecified> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceUnspecified) -> Self {
        Self::Unspecified(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementErrorSourceDataField`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceDataField> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceDataField) -> Self {
        Self::DataField(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementErrorSourceFrontSide`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceFrontSide> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceFrontSide) -> Self {
        Self::FrontSide(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementErrorSourceReverseSide`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceReverseSide> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceReverseSide) -> Self {
        Self::ReverseSide(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementErrorSourceSelfie`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceSelfie> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceSelfie) -> Self {
        Self::Selfie(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementErrorSourceTranslationFile`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceTranslationFile> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceTranslationFile) -> Self {
        Self::TranslationFile(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementErrorSourceTranslationFiles`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceTranslationFiles> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceTranslationFiles) -> Self {
        Self::TranslationFiles(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementErrorSourceFile`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceFile> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceFile) -> Self {
        Self::File(Box::new(val))
    }
}

/// Converts a [`crate::types::InputPassportElementErrorSourceFiles`] into [`InputPassportElementErrorSource`].
impl From<crate::types::InputPassportElementErrorSourceFiles> for InputPassportElementErrorSource {
    fn from(val: crate::types::InputPassportElementErrorSourceFiles) -> Self {
        Self::Files(Box::new(val))
    }
}

/// TDLib `InputPassportElementError` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPassportElementError {
    /// Contains the description of an error in a Telegram Passport element; for bots only
    #[serde(rename(serialize = "inputPassportElementError", deserialize = "inputPassportElementError"))]
    InputPassportElementError(Box<crate::types::InputPassportElementError>),
}

impl InputPassportElementError {
    /// Convenience constructor to create a [`InputPassportElementError::InputPassportElementError`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn input_passport_element_error(val: crate::types::InputPassportElementError) -> Self {
        Self::InputPassportElementError(Box::new(val))
    }

}

/// Converts a [`crate::types::InputPassportElementError`] into [`InputPassportElementError`].
impl From<crate::types::InputPassportElementError> for InputPassportElementError {
    fn from(val: crate::types::InputPassportElementError) -> Self {
        Self::InputPassportElementError(Box::new(val))
    }
}

