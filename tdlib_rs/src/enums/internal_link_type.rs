#[allow(clippy::all)]
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InternalLinkType {
    /// The link is a link to an attachment menu bot to be opened in the specified or a chosen chat. Process given target_chat to open the chat.
    /// Then, call searchPublicChat with the given bot username, check that the user is a bot and can be added to attachment menu. Then, use getAttachmentMenuBot to receive information about the bot.
    /// If the bot isn't added to attachment menu, then show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu.
    /// If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot.
    /// If the attachment menu bot can't be used in the opened chat, show an error to the user. If the bot is added to attachment menu and can be used in the chat, then use openWebApp with the given URL
    #[serde(rename(serialize = "internalLinkTypeAttachmentMenuBot", deserialize = "internalLinkTypeAttachmentMenuBot"))]
    AttachmentMenuBot(Box<crate::types::InternalLinkTypeAttachmentMenuBot>),
    /// The link contains an authentication code. Call checkAuthenticationCode with the code if the current authorization state is authorizationStateWaitCode
    #[serde(rename(serialize = "internalLinkTypeAuthenticationCode", deserialize = "internalLinkTypeAuthenticationCode"))]
    AuthenticationCode(crate::types::InternalLinkTypeAuthenticationCode),
    /// The link is a link to a background. Call searchBackground with the given background name to process the link.
    /// If background is found and the user wants to apply it, then call setDefaultBackground
    #[serde(rename(serialize = "internalLinkTypeBackground", deserialize = "internalLinkTypeBackground"))]
    Background(crate::types::InternalLinkTypeBackground),
    /// The link is a link to a Telegram bot, which is expected to be added to a channel chat as an administrator. Call searchPublicChat with the given bot username and check that the user is a bot,
    /// ask the current user to select a channel chat to add the bot to as an administrator. Then, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator,
    /// check that the current user can edit its administrator rights and combine received rights with the requested administrator rights. Then, show confirmation box to the user, and call setChatMemberStatus with the chosen chat and confirmed rights
    #[serde(rename(serialize = "internalLinkTypeBotAddToChannel", deserialize = "internalLinkTypeBotAddToChannel"))]
    BotAddToChannel(crate::types::InternalLinkTypeBotAddToChannel),
    /// The link is a link to a chat with a Telegram bot. Call searchPublicChat with the given bot username, check that the user is a bot, show START button in the chat with the bot,
    /// and then call sendBotStartMessage with the given start parameter after the button is pressed
    #[serde(rename(serialize = "internalLinkTypeBotStart", deserialize = "internalLinkTypeBotStart"))]
    BotStart(crate::types::InternalLinkTypeBotStart),
    /// The link is a link to a Telegram bot, which is expected to be added to a group chat. Call searchPublicChat with the given bot username, check that the user is a bot and can be added to groups,
    /// ask the current user to select a basic group or a supergroup chat to add the bot to, taking into account that bots can be added to a public supergroup only by administrators of the supergroup.
    /// If administrator rights are provided by the link, call getChatMember to receive the current bot rights in the chat and if the bot already is an administrator,
    /// check that the current user can edit its administrator rights, combine received rights with the requested administrator rights, show confirmation box to the user,
    /// and call setChatMemberStatus with the chosen chat and confirmed administrator rights. Before call to setChatMemberStatus it may be required to upgrade the chosen basic group chat to a supergroup chat.
    /// Then, if start_parameter isn't empty, call sendBotStartMessage with the given start parameter and the chosen chat; otherwise, just send /start message with bot's username added to the chat
    #[serde(rename(serialize = "internalLinkTypeBotStartInGroup", deserialize = "internalLinkTypeBotStartInGroup"))]
    BotStartInGroup(crate::types::InternalLinkTypeBotStartInGroup),
    /// The link is a link to a business chat. Use getBusinessChatLinkInfo with the provided link name to get information about the link,
    /// then open received private chat and replace chat draft with the provided text
    #[serde(rename(serialize = "internalLinkTypeBusinessChat", deserialize = "internalLinkTypeBusinessChat"))]
    BusinessChat(crate::types::InternalLinkTypeBusinessChat),
    /// The link is a link to the Call tab or page
    #[serde(rename(serialize = "internalLinkTypeCallsPage", deserialize = "internalLinkTypeCallsPage"))]
    CallsPage(crate::types::InternalLinkTypeCallsPage),
    /// The link is an affiliate program link. Call searchChatAffiliateProgram with the given username and referrer to process the link
    #[serde(rename(serialize = "internalLinkTypeChatAffiliateProgram", deserialize = "internalLinkTypeChatAffiliateProgram"))]
    ChatAffiliateProgram(crate::types::InternalLinkTypeChatAffiliateProgram),
    /// The link is a link to boost a Telegram chat. Call getChatBoostLinkInfo with the given URL to process the link.
    /// If the chat is found, then call getChatBoostStatus and getAvailableChatBoostSlots to get the current boost status and check whether the chat can be boosted.
    /// If the user wants to boost the chat and the chat can be boosted, then call boostChat
    #[serde(rename(serialize = "internalLinkTypeChatBoost", deserialize = "internalLinkTypeChatBoost"))]
    ChatBoost(crate::types::InternalLinkTypeChatBoost),
    /// The link is an invite link to a chat folder. Call checkChatFolderInviteLink with the given invite link to process the link.
    /// If the link is valid and the user wants to join the chat folder, then call addChatFolderByInviteLink
    #[serde(rename(serialize = "internalLinkTypeChatFolderInvite", deserialize = "internalLinkTypeChatFolderInvite"))]
    ChatFolderInvite(crate::types::InternalLinkTypeChatFolderInvite),
    /// The link is a chat invite link. Call checkChatInviteLink with the given invite link to process the link.
    /// If the link is valid and the user wants to join the chat, then call joinChatByInviteLink
    #[serde(rename(serialize = "internalLinkTypeChatInvite", deserialize = "internalLinkTypeChatInvite"))]
    ChatInvite(crate::types::InternalLinkTypeChatInvite),
    /// The link is a link that allows to select some chats
    #[serde(rename(serialize = "internalLinkTypeChatSelection", deserialize = "internalLinkTypeChatSelection"))]
    ChatSelection,
    /// The link is a link to the Contacts tab or page
    #[serde(rename(serialize = "internalLinkTypeContactsPage", deserialize = "internalLinkTypeContactsPage"))]
    ContactsPage(crate::types::InternalLinkTypeContactsPage),
    /// The link is a link to a channel direct messages chat by username of the channel. Call searchPublicChat with the given chat username to process the link.
    /// If the chat is found and is channel, open the direct messages chat of the channel
    #[serde(rename(serialize = "internalLinkTypeDirectMessagesChat", deserialize = "internalLinkTypeDirectMessagesChat"))]
    DirectMessagesChat(crate::types::InternalLinkTypeDirectMessagesChat),
    /// The link is a link to a game. Call searchPublicChat with the given bot username, check that the user is a bot,
    /// ask the current user to select a chat to send the game, and then call sendMessage with inputMessageGame
    #[serde(rename(serialize = "internalLinkTypeGame", deserialize = "internalLinkTypeGame"))]
    Game(crate::types::InternalLinkTypeGame),
    /// The link is a link to a gift auction. Call getGiftAuctionState with the given auction identifier to process the link
    #[serde(rename(serialize = "internalLinkTypeGiftAuction", deserialize = "internalLinkTypeGiftAuction"))]
    GiftAuction(crate::types::InternalLinkTypeGiftAuction),
    /// The link is a link to a gift collection. Call searchPublicChat with the given username, then call getReceivedGifts with the received gift owner identifier
    /// and the given collection identifier, then show the collection if received
    #[serde(rename(serialize = "internalLinkTypeGiftCollection", deserialize = "internalLinkTypeGiftCollection"))]
    GiftCollection(crate::types::InternalLinkTypeGiftCollection),
    /// The link is a link to a group call that isn't bound to a chat. Use getGroupCallParticipants to get the list of group call participants and show them on the join group call screen.
    /// Call joinGroupCall with the given invite_link to join the call
    #[serde(rename(serialize = "internalLinkTypeGroupCall", deserialize = "internalLinkTypeGroupCall"))]
    GroupCall(crate::types::InternalLinkTypeGroupCall),
    /// The link must be opened in an Instant View. Call getWebPageInstantView with the given URL to process the link.
    /// If Instant View is found, then show it, otherwise, open the fallback URL in an external browser
    #[serde(rename(serialize = "internalLinkTypeInstantView", deserialize = "internalLinkTypeInstantView"))]
    InstantView(crate::types::InternalLinkTypeInstantView),
    /// The link is a link to an invoice. Call getPaymentForm with the given invoice name to process the link
    #[serde(rename(serialize = "internalLinkTypeInvoice", deserialize = "internalLinkTypeInvoice"))]
    Invoice(crate::types::InternalLinkTypeInvoice),
    /// The link is a link to a language pack. Call getLanguagePackInfo with the given language pack identifier to process the link.
    /// If the language pack is found and the user wants to apply it, then call setOption for the option "language_pack_id"
    #[serde(rename(serialize = "internalLinkTypeLanguagePack", deserialize = "internalLinkTypeLanguagePack"))]
    LanguagePack(crate::types::InternalLinkTypeLanguagePack),
    /// The link is a link to a live story. Call searchPublicChat with the given chat username, then getChatActiveStories to get active stories in the chat,
    /// then find a live story among active stories of the chat, and then joinLiveStory to join the live story
    #[serde(rename(serialize = "internalLinkTypeLiveStory", deserialize = "internalLinkTypeLiveStory"))]
    LiveStory(crate::types::InternalLinkTypeLiveStory),
    /// The link is a link to the main Web App of a bot. Call searchPublicChat with the given bot username, check that the user is a bot and has the main Web App.
    /// If the bot can be added to attachment menu, then use getAttachmentMenuBot to receive information about the bot, then if the bot isn't added to side menu,
    /// show a disclaimer about Mini Apps being third-party applications, ask the user to accept their Terms of service and confirm adding the bot to side and attachment menu,
    /// then if the user accepts the terms and confirms adding, use toggleBotIsAddedToAttachmentMenu to add the bot.
    /// Then, use getMainWebApp with the given start parameter and mode and open the returned URL as a Web App
    #[serde(rename(serialize = "internalLinkTypeMainWebApp", deserialize = "internalLinkTypeMainWebApp"))]
    MainWebApp(crate::types::InternalLinkTypeMainWebApp),
    /// The link is a link to a Telegram message or a forum topic. Call getMessageLinkInfo with the given URL to process the link,
    /// and then open received forum topic or chat and show the message there
    #[serde(rename(serialize = "internalLinkTypeMessage", deserialize = "internalLinkTypeMessage"))]
    Message(crate::types::InternalLinkTypeMessage),
    /// The link contains a message draft text. A share screen needs to be shown to the user, then the chosen chat must be opened and the text is added to the input field
    #[serde(rename(serialize = "internalLinkTypeMessageDraft", deserialize = "internalLinkTypeMessageDraft"))]
    MessageDraft(crate::types::InternalLinkTypeMessageDraft),
    /// The link is a link to the My Profile application page
    #[serde(rename(serialize = "internalLinkTypeMyProfilePage", deserialize = "internalLinkTypeMyProfilePage"))]
    MyProfilePage(crate::types::InternalLinkTypeMyProfilePage),
    /// The link is a link to the screen for creating a new channel chat
    #[serde(rename(serialize = "internalLinkTypeNewChannelChat", deserialize = "internalLinkTypeNewChannelChat"))]
    NewChannelChat,
    /// The link is a link to the screen for creating a new group chat
    #[serde(rename(serialize = "internalLinkTypeNewGroupChat", deserialize = "internalLinkTypeNewGroupChat"))]
    NewGroupChat,
    /// The link is a link to the screen for creating a new private chat with a contact
    #[serde(rename(serialize = "internalLinkTypeNewPrivateChat", deserialize = "internalLinkTypeNewPrivateChat"))]
    NewPrivateChat,
    /// The link is a link to open the story posting interface
    #[serde(rename(serialize = "internalLinkTypeNewStory", deserialize = "internalLinkTypeNewStory"))]
    NewStory(crate::types::InternalLinkTypeNewStory),
    /// The link is an OAuth link. Call getOauthLinkInfo with the given URL to process the link if the link was received from outside of the application; otherwise, ignore it.
    /// After getOauthLinkInfo, show the user confirmation dialog and process it with checkOauthRequestMatchCode, acceptOauthRequest or declineOauthRequest
    #[serde(rename(serialize = "internalLinkTypeOauth", deserialize = "internalLinkTypeOauth"))]
    Oauth(crate::types::InternalLinkTypeOauth),
    /// The link contains a request of Telegram passport data. Call getPassportAuthorizationForm with the given parameters to process the link if the link was received from outside of the application; otherwise, ignore it
    #[serde(rename(serialize = "internalLinkTypePassportDataRequest", deserialize = "internalLinkTypePassportDataRequest"))]
    PassportDataRequest(crate::types::InternalLinkTypePassportDataRequest),
    /// The link can be used to confirm ownership of a phone number to prevent account deletion. Call sendPhoneNumberCode with the given phone number and with phoneNumberCodeTypeConfirmOwnership with the given hash to process the link.
    /// If succeeded, call checkPhoneNumberCode to check entered by the user code, or resendPhoneNumberCode to resend it
    #[serde(rename(serialize = "internalLinkTypePhoneNumberConfirmation", deserialize = "internalLinkTypePhoneNumberConfirmation"))]
    PhoneNumberConfirmation(crate::types::InternalLinkTypePhoneNumberConfirmation),
    /// The link is a link to the Premium features screen of the application from which the user can subscribe to Telegram Premium. Call getPremiumFeatures with the given referrer to process the link
    #[serde(rename(serialize = "internalLinkTypePremiumFeaturesPage", deserialize = "internalLinkTypePremiumFeaturesPage"))]
    PremiumFeaturesPage(crate::types::InternalLinkTypePremiumFeaturesPage),
    /// The link is a link with a Telegram Premium gift code. Call checkPremiumGiftCode with the given code to process the link.
    /// If the code is valid and the user wants to apply it, then call applyPremiumGiftCode
    #[serde(rename(serialize = "internalLinkTypePremiumGiftCode", deserialize = "internalLinkTypePremiumGiftCode"))]
    PremiumGiftCode(crate::types::InternalLinkTypePremiumGiftCode),
    /// The link is a link to the screen for gifting Telegram Premium subscriptions to friends via inputInvoiceTelegram with telegramPaymentPurposePremiumGift payments or in-store purchases
    #[serde(rename(serialize = "internalLinkTypePremiumGiftPurchase", deserialize = "internalLinkTypePremiumGiftPurchase"))]
    PremiumGiftPurchase(crate::types::InternalLinkTypePremiumGiftPurchase),
    /// The link is a link to a proxy. Call addProxy with the given parameters to process the link and add the proxy
    #[serde(rename(serialize = "internalLinkTypeProxy", deserialize = "internalLinkTypeProxy"))]
    Proxy(crate::types::InternalLinkTypeProxy),
    /// The link is a link to a chat by its username. Call searchPublicChat with the given chat username to process the link.
    /// If the chat is found, open its profile information screen or the chat itself.
    /// If draft text isn't empty and the chat is a private chat with a regular user, then put the draft text in the input field
    #[serde(rename(serialize = "internalLinkTypePublicChat", deserialize = "internalLinkTypePublicChat"))]
    PublicChat(crate::types::InternalLinkTypePublicChat),
    /// The link can be used to login the current user on another device, but it must be scanned from QR-code using in-app camera. An alert similar to
    /// "This code can be used to allow someone to log in to your Telegram account. To confirm Telegram login, please go to Settings > Devices > Scan QR and scan the code" needs to be shown
    #[serde(rename(serialize = "internalLinkTypeQrCodeAuthentication", deserialize = "internalLinkTypeQrCodeAuthentication"))]
    QrCodeAuthentication,
    /// The link forces restore of App Store purchases when opened. For official iOS application only
    #[serde(rename(serialize = "internalLinkTypeRestorePurchases", deserialize = "internalLinkTypeRestorePurchases"))]
    RestorePurchases,
    /// The link is a link to the Saved Messages chat. Call createPrivateChat with getOption("my_id") and open the chat
    #[serde(rename(serialize = "internalLinkTypeSavedMessages", deserialize = "internalLinkTypeSavedMessages"))]
    SavedMessages,
    /// The link is a link to the global chat and messages search field
    #[serde(rename(serialize = "internalLinkTypeSearch", deserialize = "internalLinkTypeSearch"))]
    Search,
    /// The link is a link to application settings
    #[serde(rename(serialize = "internalLinkTypeSettings", deserialize = "internalLinkTypeSettings"))]
    Settings(crate::types::InternalLinkTypeSettings),
    /// The link is a link to the Telegram Star purchase section of the application
    #[serde(rename(serialize = "internalLinkTypeStarPurchase", deserialize = "internalLinkTypeStarPurchase"))]
    StarPurchase(crate::types::InternalLinkTypeStarPurchase),
    /// The link is a link to a sticker set. Call searchStickerSet with the given sticker set name to process the link and show the sticker set.
    /// If the sticker set is found and the user wants to add it, then call changeStickerSet
    #[serde(rename(serialize = "internalLinkTypeStickerSet", deserialize = "internalLinkTypeStickerSet"))]
    StickerSet(crate::types::InternalLinkTypeStickerSet),
    /// The link is a link to a story. Call searchPublicChat with the given poster username, then call getStory with the received chat identifier and the given story identifier, then show the story if received
    #[serde(rename(serialize = "internalLinkTypeStory", deserialize = "internalLinkTypeStory"))]
    Story(crate::types::InternalLinkTypeStory),
    /// The link is a link to an album of stories. Call searchPublicChat with the given username, then call getStoryAlbumStories with the received chat identifier
    /// and the given story album identifier, then show the story album if received
    #[serde(rename(serialize = "internalLinkTypeStoryAlbum", deserialize = "internalLinkTypeStoryAlbum"))]
    StoryAlbum(crate::types::InternalLinkTypeStoryAlbum),
    /// The link is a link to a cloud theme. TDLib has no theme support yet
    #[serde(rename(serialize = "internalLinkTypeTheme", deserialize = "internalLinkTypeTheme"))]
    Theme(crate::types::InternalLinkTypeTheme),
    /// The link is an unknown tg: link. Call getDeepLinkInfo to process the link
    #[serde(rename(serialize = "internalLinkTypeUnknownDeepLink", deserialize = "internalLinkTypeUnknownDeepLink"))]
    UnknownDeepLink(crate::types::InternalLinkTypeUnknownDeepLink),
    /// The link is a link to an upgraded gift. Call getUpgradedGift with the given name to process the link
    #[serde(rename(serialize = "internalLinkTypeUpgradedGift", deserialize = "internalLinkTypeUpgradedGift"))]
    UpgradedGift(crate::types::InternalLinkTypeUpgradedGift),
    /// The link is a link to a user by its phone number. Call searchUserByPhoneNumber with the given phone number to process the link.
    /// If the user is found, then call createPrivateChat and open user's profile information screen or the chat itself. If draft text isn't empty, then put the draft text in the input field
    #[serde(rename(serialize = "internalLinkTypeUserPhoneNumber", deserialize = "internalLinkTypeUserPhoneNumber"))]
    UserPhoneNumber(crate::types::InternalLinkTypeUserPhoneNumber),
    /// The link is a link to a user by a temporary token. Call searchUserByToken with the given token to process the link.
    /// If the user is found, then call createPrivateChat and open the chat
    #[serde(rename(serialize = "internalLinkTypeUserToken", deserialize = "internalLinkTypeUserToken"))]
    UserToken(crate::types::InternalLinkTypeUserToken),
    /// The link is a link to a video chat. Call searchPublicChat with the given chat username, and then joinVideoChat with the given invite hash to process the link
    #[serde(rename(serialize = "internalLinkTypeVideoChat", deserialize = "internalLinkTypeVideoChat"))]
    VideoChat(crate::types::InternalLinkTypeVideoChat),
    /// The link is a link to a Web App. Call searchPublicChat with the given bot username, check that the user is a bot. If the bot is restricted for the current user, then show an error message.
    /// Otherwise, call searchWebApp with the received bot and the given web_app_short_name. Process received foundWebApp by showing a confirmation dialog if needed.
    /// If the bot can be added to attachment or side menu, but isn't added yet, then show a disclaimer about Mini Apps being third-party applications instead of the dialog
    /// and ask the user to accept their Terms of service. If the user accept the terms and confirms adding, then use toggleBotIsAddedToAttachmentMenu to add the bot.
    /// Then, call getWebAppLinkUrl and open the returned URL as a Web App
    #[serde(rename(serialize = "internalLinkTypeWebApp", deserialize = "internalLinkTypeWebApp"))]
    WebApp(crate::types::InternalLinkTypeWebApp),
}
