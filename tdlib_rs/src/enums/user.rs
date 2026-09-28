//!
//! TDLib `user` domain enums.
//!
//! Types, enums, and functions for user accounts, privacy settings, and contacts.
//!

#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

/// TDLib `Contact` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Contact {
    /// Describes a contact of a user
    #[serde(rename(serialize = "contact", deserialize = "contact"))]
    Contact(Box<crate::types::Contact>),
}

impl Contact {
    /// Convenience constructor to create a [`Contact::Contact`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn contact(val: crate::types::Contact) -> Self {
        Self::Contact(Box::new(val))
    }

}

/// Converts a [`crate::types::Contact`] into [`Contact`].
impl From<crate::types::Contact> for Contact {
    fn from(val: crate::types::Contact) -> Self {
        Self::Contact(Box::new(val))
    }
}

/// Represents the type of user. The following types are possible: regular users, deleted users and bots
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserType {
    /// A regular user
    #[serde(rename(serialize = "userTypeRegular", deserialize = "userTypeRegular"))]
    Regular,
    /// A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user
    #[serde(rename(serialize = "userTypeDeleted", deserialize = "userTypeDeleted"))]
    Deleted,
    /// A bot (see https:core.telegram.org/bots)
    #[serde(rename(serialize = "userTypeBot", deserialize = "userTypeBot"))]
    Bot(Box<crate::types::UserTypeBot>),
    /// No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type
    #[serde(rename(serialize = "userTypeUnknown", deserialize = "userTypeUnknown"))]
    Unknown,
}

impl UserType {
    /// Convenience constructor to create a [`UserType::Bot`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn bot(val: crate::types::UserTypeBot) -> Self {
        Self::Bot(Box::new(val))
    }

}

/// Converts a [`crate::types::UserTypeBot`] into [`UserType`].
impl From<crate::types::UserTypeBot> for UserType {
    fn from(val: crate::types::UserTypeBot) -> Self {
        Self::Bot(Box::new(val))
    }
}

/// TDLib `CloseBirthdayUser` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CloseBirthdayUser {
    /// Describes a user who had or will have a birthday soon
    #[serde(rename(serialize = "closeBirthdayUser", deserialize = "closeBirthdayUser"))]
    CloseBirthdayUser(Box<crate::types::CloseBirthdayUser>),
}

impl CloseBirthdayUser {
    /// Convenience constructor to create a [`CloseBirthdayUser::CloseBirthdayUser`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn close_birthday_user(val: crate::types::CloseBirthdayUser) -> Self {
        Self::CloseBirthdayUser(Box::new(val))
    }

}

/// Converts a [`crate::types::CloseBirthdayUser`] into [`CloseBirthdayUser`].
impl From<crate::types::CloseBirthdayUser> for CloseBirthdayUser {
    fn from(val: crate::types::CloseBirthdayUser) -> Self {
        Self::CloseBirthdayUser(Box::new(val))
    }
}

/// TDLib `UserAuctionBid` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserAuctionBid {
    /// Describes a bid of the current user in an auction
    #[serde(rename(serialize = "userAuctionBid", deserialize = "userAuctionBid"))]
    UserAuctionBid(Box<crate::types::UserAuctionBid>),
}

impl UserAuctionBid {
    /// Convenience constructor to create a [`UserAuctionBid::UserAuctionBid`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_auction_bid(val: crate::types::UserAuctionBid) -> Self {
        Self::UserAuctionBid(Box::new(val))
    }

}

/// Converts a [`crate::types::UserAuctionBid`] into [`UserAuctionBid`].
impl From<crate::types::UserAuctionBid> for UserAuctionBid {
    fn from(val: crate::types::UserAuctionBid) -> Self {
        Self::UserAuctionBid(Box::new(val))
    }
}

/// TDLib `UserRating` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserRating {
    /// Contains description of user rating
    #[serde(rename(serialize = "userRating", deserialize = "userRating"))]
    UserRating(Box<crate::types::UserRating>),
}

impl UserRating {
    /// Convenience constructor to create a [`UserRating::UserRating`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_rating(val: crate::types::UserRating) -> Self {
        Self::UserRating(Box::new(val))
    }

}

/// Converts a [`crate::types::UserRating`] into [`UserRating`].
impl From<crate::types::UserRating> for UserRating {
    fn from(val: crate::types::UserRating) -> Self {
        Self::UserRating(Box::new(val))
    }
}

/// TDLib `Usernames` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Usernames {
    /// Describes usernames assigned to a user, a supergroup, or a channel
    #[serde(rename(serialize = "usernames", deserialize = "usernames"))]
    Usernames(Box<crate::types::Usernames>),
}

impl Usernames {
    /// Convenience constructor to create a [`Usernames::Usernames`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn usernames(val: crate::types::Usernames) -> Self {
        Self::Usernames(Box::new(val))
    }

}

/// Converts a [`crate::types::Usernames`] into [`Usernames`].
impl From<crate::types::Usernames> for Usernames {
    fn from(val: crate::types::Usernames) -> Self {
        Self::Usernames(Box::new(val))
    }
}

/// TDLib `User` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum User {
    /// Represents a user
    #[serde(rename(serialize = "user", deserialize = "user"))]
    User(Box<crate::types::User>),
}

impl User {
    /// Convenience constructor to create a [`User::User`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user(val: crate::types::User) -> Self {
        Self::User(Box::new(val))
    }

}

/// Converts a [`crate::types::User`] into [`User`].
impl From<crate::types::User> for User {
    fn from(val: crate::types::User) -> Self {
        Self::User(Box::new(val))
    }
}

/// TDLib `UserFullInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserFullInfo {
    /// Contains full information about a user
    #[serde(rename(serialize = "userFullInfo", deserialize = "userFullInfo"))]
    UserFullInfo(Box<crate::types::UserFullInfo>),
}

impl UserFullInfo {
    /// Convenience constructor to create a [`UserFullInfo::UserFullInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_full_info(val: crate::types::UserFullInfo) -> Self {
        Self::UserFullInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::UserFullInfo`] into [`UserFullInfo`].
impl From<crate::types::UserFullInfo> for UserFullInfo {
    fn from(val: crate::types::UserFullInfo) -> Self {
        Self::UserFullInfo(Box::new(val))
    }
}

/// TDLib `Users` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Users {
    /// Represents a list of users
    #[serde(rename(serialize = "users", deserialize = "users"))]
    Users(Box<crate::types::Users>),
}

impl Users {
    /// Convenience constructor to create a [`Users::Users`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn users(val: crate::types::Users) -> Self {
        Self::Users(Box::new(val))
    }

}

/// Converts a [`crate::types::Users`] into [`Users`].
impl From<crate::types::Users> for Users {
    fn from(val: crate::types::Users) -> Self {
        Self::Users(Box::new(val))
    }
}

/// TDLib `FoundUsers` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FoundUsers {
    /// Represents a list of found users
    #[serde(rename(serialize = "foundUsers", deserialize = "foundUsers"))]
    FoundUsers(Box<crate::types::FoundUsers>),
}

impl FoundUsers {
    /// Convenience constructor to create a [`FoundUsers::FoundUsers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn found_users(val: crate::types::FoundUsers) -> Self {
        Self::FoundUsers(Box::new(val))
    }

}

/// Converts a [`crate::types::FoundUsers`] into [`FoundUsers`].
impl From<crate::types::FoundUsers> for FoundUsers {
    fn from(val: crate::types::FoundUsers) -> Self {
        Self::FoundUsers(Box::new(val))
    }
}

/// TDLib `SharedUser` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SharedUser {
    /// Contains information about a user shared with a bot
    #[serde(rename(serialize = "sharedUser", deserialize = "sharedUser"))]
    SharedUser(Box<crate::types::SharedUser>),
}

impl SharedUser {
    /// Convenience constructor to create a [`SharedUser::SharedUser`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn shared_user(val: crate::types::SharedUser) -> Self {
        Self::SharedUser(Box::new(val))
    }

}

/// Converts a [`crate::types::SharedUser`] into [`SharedUser`].
impl From<crate::types::SharedUser> for SharedUser {
    fn from(val: crate::types::SharedUser) -> Self {
        Self::SharedUser(Box::new(val))
    }
}

/// Describes the last time the user was online
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserStatus {
    /// The user's status has never been changed
    #[serde(rename(serialize = "userStatusEmpty", deserialize = "userStatusEmpty"))]
    Empty,
    /// The user is online
    #[serde(rename(serialize = "userStatusOnline", deserialize = "userStatusOnline"))]
    Online(Box<crate::types::UserStatusOnline>),
    /// The user is offline
    #[serde(rename(serialize = "userStatusOffline", deserialize = "userStatusOffline"))]
    Offline(Box<crate::types::UserStatusOffline>),
    /// The user was online recently
    #[serde(rename(serialize = "userStatusRecently", deserialize = "userStatusRecently"))]
    Recently(Box<crate::types::UserStatusRecently>),
    /// The user is offline, but was online last week
    #[serde(rename(serialize = "userStatusLastWeek", deserialize = "userStatusLastWeek"))]
    LastWeek(Box<crate::types::UserStatusLastWeek>),
    /// The user is offline, but was online last month
    #[serde(rename(serialize = "userStatusLastMonth", deserialize = "userStatusLastMonth"))]
    LastMonth(Box<crate::types::UserStatusLastMonth>),
}

impl UserStatus {
    /// Convenience constructor to create a [`UserStatus::Online`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn online(val: crate::types::UserStatusOnline) -> Self {
        Self::Online(Box::new(val))
    }

    /// Convenience constructor to create a [`UserStatus::Offline`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn offline(val: crate::types::UserStatusOffline) -> Self {
        Self::Offline(Box::new(val))
    }

    /// Convenience constructor to create a [`UserStatus::Recently`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn recently(val: crate::types::UserStatusRecently) -> Self {
        Self::Recently(Box::new(val))
    }

    /// Convenience constructor to create a [`UserStatus::LastWeek`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn last_week(val: crate::types::UserStatusLastWeek) -> Self {
        Self::LastWeek(Box::new(val))
    }

    /// Convenience constructor to create a [`UserStatus::LastMonth`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn last_month(val: crate::types::UserStatusLastMonth) -> Self {
        Self::LastMonth(Box::new(val))
    }

}

/// Converts a [`crate::types::UserStatusOnline`] into [`UserStatus`].
impl From<crate::types::UserStatusOnline> for UserStatus {
    fn from(val: crate::types::UserStatusOnline) -> Self {
        Self::Online(Box::new(val))
    }
}

/// Converts a [`crate::types::UserStatusOffline`] into [`UserStatus`].
impl From<crate::types::UserStatusOffline> for UserStatus {
    fn from(val: crate::types::UserStatusOffline) -> Self {
        Self::Offline(Box::new(val))
    }
}

/// Converts a [`crate::types::UserStatusRecently`] into [`UserStatus`].
impl From<crate::types::UserStatusRecently> for UserStatus {
    fn from(val: crate::types::UserStatusRecently) -> Self {
        Self::Recently(Box::new(val))
    }
}

/// Converts a [`crate::types::UserStatusLastWeek`] into [`UserStatus`].
impl From<crate::types::UserStatusLastWeek> for UserStatus {
    fn from(val: crate::types::UserStatusLastWeek) -> Self {
        Self::LastWeek(Box::new(val))
    }
}

/// Converts a [`crate::types::UserStatusLastMonth`] into [`UserStatus`].
impl From<crate::types::UserStatusLastMonth> for UserStatus {
    fn from(val: crate::types::UserStatusLastMonth) -> Self {
        Self::LastMonth(Box::new(val))
    }
}

/// TDLib `ImportedContact` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ImportedContact {
    /// Describes a contact to import
    #[serde(rename(serialize = "importedContact", deserialize = "importedContact"))]
    ImportedContact(Box<crate::types::ImportedContact>),
}

impl ImportedContact {
    /// Convenience constructor to create a [`ImportedContact::ImportedContact`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn imported_contact(val: crate::types::ImportedContact) -> Self {
        Self::ImportedContact(Box::new(val))
    }

}

/// Converts a [`crate::types::ImportedContact`] into [`ImportedContact`].
impl From<crate::types::ImportedContact> for ImportedContact {
    fn from(val: crate::types::ImportedContact) -> Self {
        Self::ImportedContact(Box::new(val))
    }
}

/// TDLib `ImportedContacts` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ImportedContacts {
    /// Represents the result of an importContacts request
    #[serde(rename(serialize = "importedContacts", deserialize = "importedContacts"))]
    ImportedContacts(Box<crate::types::ImportedContacts>),
}

impl ImportedContacts {
    /// Convenience constructor to create a [`ImportedContacts::ImportedContacts`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn imported_contacts(val: crate::types::ImportedContacts) -> Self {
        Self::ImportedContacts(Box::new(val))
    }

}

/// Converts a [`crate::types::ImportedContacts`] into [`ImportedContacts`].
impl From<crate::types::ImportedContacts> for ImportedContacts {
    fn from(val: crate::types::ImportedContacts) -> Self {
        Self::ImportedContacts(Box::new(val))
    }
}

/// TDLib `UserLink` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserLink {
    /// Contains an HTTPS URL, which can be used to get information about a user
    #[serde(rename(serialize = "userLink", deserialize = "userLink"))]
    UserLink(Box<crate::types::UserLink>),
}

impl UserLink {
    /// Convenience constructor to create a [`UserLink::UserLink`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_link(val: crate::types::UserLink) -> Self {
        Self::UserLink(Box::new(val))
    }

}

/// Converts a [`crate::types::UserLink`] into [`UserLink`].
impl From<crate::types::UserLink> for UserLink {
    fn from(val: crate::types::UserLink) -> Self {
        Self::UserLink(Box::new(val))
    }
}

/// Represents a single rule for managing user privacy settings
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserPrivacySettingRule {
    /// A rule to allow all users to do something
    #[serde(rename(serialize = "userPrivacySettingRuleAllowAll", deserialize = "userPrivacySettingRuleAllowAll"))]
    AllowAll,
    /// A rule to allow all contacts of the user to do something
    #[serde(rename(serialize = "userPrivacySettingRuleAllowContacts", deserialize = "userPrivacySettingRuleAllowContacts"))]
    AllowContacts,
    /// A rule to allow all bots to do something
    #[serde(rename(serialize = "userPrivacySettingRuleAllowBots", deserialize = "userPrivacySettingRuleAllowBots"))]
    AllowBots,
    /// A rule to allow all Premium Users to do something; currently, allowed only for userPrivacySettingAllowChatInvites
    #[serde(rename(serialize = "userPrivacySettingRuleAllowPremiumUsers", deserialize = "userPrivacySettingRuleAllowPremiumUsers"))]
    AllowPremiumUsers,
    /// A rule to allow certain specified users to do something
    #[serde(rename(serialize = "userPrivacySettingRuleAllowUsers", deserialize = "userPrivacySettingRuleAllowUsers"))]
    AllowUsers(Box<crate::types::UserPrivacySettingRuleAllowUsers>),
    /// A rule to allow all members of certain specified basic groups and supergroups to doing something
    #[serde(rename(serialize = "userPrivacySettingRuleAllowChatMembers", deserialize = "userPrivacySettingRuleAllowChatMembers"))]
    AllowChatMembers(Box<crate::types::UserPrivacySettingRuleAllowChatMembers>),
    /// A rule to restrict all users from doing something
    #[serde(rename(serialize = "userPrivacySettingRuleRestrictAll", deserialize = "userPrivacySettingRuleRestrictAll"))]
    RestrictAll,
    /// A rule to restrict all contacts of the user from doing something
    #[serde(rename(serialize = "userPrivacySettingRuleRestrictContacts", deserialize = "userPrivacySettingRuleRestrictContacts"))]
    RestrictContacts,
    /// A rule to restrict all bots from doing something
    #[serde(rename(serialize = "userPrivacySettingRuleRestrictBots", deserialize = "userPrivacySettingRuleRestrictBots"))]
    RestrictBots,
    /// A rule to restrict all specified users from doing something
    #[serde(rename(serialize = "userPrivacySettingRuleRestrictUsers", deserialize = "userPrivacySettingRuleRestrictUsers"))]
    RestrictUsers(Box<crate::types::UserPrivacySettingRuleRestrictUsers>),
    /// A rule to restrict all members of specified basic groups and supergroups from doing something
    #[serde(rename(serialize = "userPrivacySettingRuleRestrictChatMembers", deserialize = "userPrivacySettingRuleRestrictChatMembers"))]
    RestrictChatMembers(Box<crate::types::UserPrivacySettingRuleRestrictChatMembers>),
}

impl UserPrivacySettingRule {
    /// Convenience constructor to create a [`UserPrivacySettingRule::AllowUsers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn allow_users(val: crate::types::UserPrivacySettingRuleAllowUsers) -> Self {
        Self::AllowUsers(Box::new(val))
    }

    /// Convenience constructor to create a [`UserPrivacySettingRule::AllowChatMembers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn allow_chat_members(val: crate::types::UserPrivacySettingRuleAllowChatMembers) -> Self {
        Self::AllowChatMembers(Box::new(val))
    }

    /// Convenience constructor to create a [`UserPrivacySettingRule::RestrictUsers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn restrict_users(val: crate::types::UserPrivacySettingRuleRestrictUsers) -> Self {
        Self::RestrictUsers(Box::new(val))
    }

    /// Convenience constructor to create a [`UserPrivacySettingRule::RestrictChatMembers`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn restrict_chat_members(val: crate::types::UserPrivacySettingRuleRestrictChatMembers) -> Self {
        Self::RestrictChatMembers(Box::new(val))
    }

}

/// Converts a [`crate::types::UserPrivacySettingRuleAllowUsers`] into [`UserPrivacySettingRule`].
impl From<crate::types::UserPrivacySettingRuleAllowUsers> for UserPrivacySettingRule {
    fn from(val: crate::types::UserPrivacySettingRuleAllowUsers) -> Self {
        Self::AllowUsers(Box::new(val))
    }
}

/// Converts a [`crate::types::UserPrivacySettingRuleAllowChatMembers`] into [`UserPrivacySettingRule`].
impl From<crate::types::UserPrivacySettingRuleAllowChatMembers> for UserPrivacySettingRule {
    fn from(val: crate::types::UserPrivacySettingRuleAllowChatMembers) -> Self {
        Self::AllowChatMembers(Box::new(val))
    }
}

/// Converts a [`crate::types::UserPrivacySettingRuleRestrictUsers`] into [`UserPrivacySettingRule`].
impl From<crate::types::UserPrivacySettingRuleRestrictUsers> for UserPrivacySettingRule {
    fn from(val: crate::types::UserPrivacySettingRuleRestrictUsers) -> Self {
        Self::RestrictUsers(Box::new(val))
    }
}

/// Converts a [`crate::types::UserPrivacySettingRuleRestrictChatMembers`] into [`UserPrivacySettingRule`].
impl From<crate::types::UserPrivacySettingRuleRestrictChatMembers> for UserPrivacySettingRule {
    fn from(val: crate::types::UserPrivacySettingRuleRestrictChatMembers) -> Self {
        Self::RestrictChatMembers(Box::new(val))
    }
}

/// TDLib `UserPrivacySettingRules` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserPrivacySettingRules {
    /// A list of privacy rules. Rules are matched in the specified order. The first matched rule defines the privacy setting for a given user. If no rule matches, the action is not allowed
    #[serde(rename(serialize = "userPrivacySettingRules", deserialize = "userPrivacySettingRules"))]
    UserPrivacySettingRules(Box<crate::types::UserPrivacySettingRules>),
}

impl UserPrivacySettingRules {
    /// Convenience constructor to create a [`UserPrivacySettingRules::UserPrivacySettingRules`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_privacy_setting_rules(val: crate::types::UserPrivacySettingRules) -> Self {
        Self::UserPrivacySettingRules(Box::new(val))
    }

}

/// Converts a [`crate::types::UserPrivacySettingRules`] into [`UserPrivacySettingRules`].
impl From<crate::types::UserPrivacySettingRules> for UserPrivacySettingRules {
    fn from(val: crate::types::UserPrivacySettingRules) -> Self {
        Self::UserPrivacySettingRules(Box::new(val))
    }
}

/// Describes available user privacy settings
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserPrivacySetting {
    /// A privacy setting for managing whether the user's online status is visible
    #[serde(rename(serialize = "userPrivacySettingShowStatus", deserialize = "userPrivacySettingShowStatus"))]
    ShowStatus,
    /// A privacy setting for managing whether the user's profile photo is visible
    #[serde(rename(serialize = "userPrivacySettingShowProfilePhoto", deserialize = "userPrivacySettingShowProfilePhoto"))]
    ShowProfilePhoto,
    /// A privacy setting for managing whether a link to the user's account is included in forwarded messages
    #[serde(rename(serialize = "userPrivacySettingShowLinkInForwardedMessages", deserialize = "userPrivacySettingShowLinkInForwardedMessages"))]
    ShowLinkInForwardedMessages,
    /// A privacy setting for managing whether the user's phone number is visible
    #[serde(rename(serialize = "userPrivacySettingShowPhoneNumber", deserialize = "userPrivacySettingShowPhoneNumber"))]
    ShowPhoneNumber,
    /// A privacy setting for managing whether the user's bio is visible
    #[serde(rename(serialize = "userPrivacySettingShowBio", deserialize = "userPrivacySettingShowBio"))]
    ShowBio,
    /// A privacy setting for managing whether the user's birthdate is visible
    #[serde(rename(serialize = "userPrivacySettingShowBirthdate", deserialize = "userPrivacySettingShowBirthdate"))]
    ShowBirthdate,
    /// A privacy setting for managing whether the user's profile audio files are visible
    #[serde(rename(serialize = "userPrivacySettingShowProfileAudio", deserialize = "userPrivacySettingShowProfileAudio"))]
    ShowProfileAudio,
    /// A privacy setting for managing whether the user can be invited to chats
    #[serde(rename(serialize = "userPrivacySettingAllowChatInvites", deserialize = "userPrivacySettingAllowChatInvites"))]
    AllowChatInvites,
    /// A privacy setting for managing whether the user can be called
    #[serde(rename(serialize = "userPrivacySettingAllowCalls", deserialize = "userPrivacySettingAllowCalls"))]
    AllowCalls,
    /// A privacy setting for managing whether peer-to-peer connections can be used for calls
    #[serde(rename(serialize = "userPrivacySettingAllowPeerToPeerCalls", deserialize = "userPrivacySettingAllowPeerToPeerCalls"))]
    AllowPeerToPeerCalls,
    /// A privacy setting for managing whether the user can be found by their phone number. Checked only if the phone number is not known to the other user. Can be set only to "Allow contacts" or "Allow all"
    #[serde(rename(serialize = "userPrivacySettingAllowFindingByPhoneNumber", deserialize = "userPrivacySettingAllowFindingByPhoneNumber"))]
    AllowFindingByPhoneNumber,
    /// A privacy setting for managing whether the user can receive voice and video messages in private chats; for Telegram Premium users only
    #[serde(rename(serialize = "userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages", deserialize = "userPrivacySettingAllowPrivateVoiceAndVideoNoteMessages"))]
    AllowPrivateVoiceAndVideoNoteMessages,
    /// A privacy setting for managing whether received gifts are automatically shown on the user's profile page
    #[serde(rename(serialize = "userPrivacySettingAutosaveGifts", deserialize = "userPrivacySettingAutosaveGifts"))]
    AutosaveGifts,
    /// A privacy setting for managing whether the user can receive messages without additional payment
    #[serde(rename(serialize = "userPrivacySettingAllowUnpaidMessages", deserialize = "userPrivacySettingAllowUnpaidMessages"))]
    AllowUnpaidMessages,
}

/// TDLib `UserSupportInfo` union enum.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserSupportInfo {
    /// Contains custom information about the user
    #[serde(rename(serialize = "userSupportInfo", deserialize = "userSupportInfo"))]
    UserSupportInfo(Box<crate::types::UserSupportInfo>),
}

impl UserSupportInfo {
    /// Convenience constructor to create a [`UserSupportInfo::UserSupportInfo`] variant.
    ///
    /// Automatically wraps the payload in `Box`.
    pub fn user_support_info(val: crate::types::UserSupportInfo) -> Self {
        Self::UserSupportInfo(Box::new(val))
    }

}

/// Converts a [`crate::types::UserSupportInfo`] into [`UserSupportInfo`].
impl From<crate::types::UserSupportInfo> for UserSupportInfo {
    fn from(val: crate::types::UserSupportInfo) -> Self {
        Self::UserSupportInfo(Box::new(val))
    }
}

