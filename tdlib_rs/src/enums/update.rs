#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum Update {
    /// The user authorization state has changed
    #[serde(rename(
        serialize = "updateAuthorizationState",
        deserialize = "updateAuthorizationState"
    ))]
    AuthorizationState(crate::types::UpdateAuthorizationState),
    /// A new message was received; can also be an outgoing message
    #[serde(rename(serialize = "updateNewMessage", deserialize = "updateNewMessage"))]
    NewMessage(crate::types::UpdateNewMessage),
    /// A new incoming callback query to a bot was received.
    ///
    /// 当前生成的 `tdlib_rs` 缺少这个 update，所以在这里补一个最小定义，
    /// 供机器人处理 inline keyboard 分页按钮使用。
    #[serde(rename(
        serialize = "updateNewCallbackQuery",
        deserialize = "updateNewCallbackQuery"
    ))]
    NewCallbackQuery(UpdateNewCallbackQuery),
    /// A request to send a message has reached the Telegram server. This doesn't mean that the message will be sent successfully.
    /// This update is sent only if the option "use_quick_ack" is set to true. This update may be sent multiple times for the same message
    #[serde(rename(
        serialize = "updateMessageSendAcknowledged",
        deserialize = "updateMessageSendAcknowledged"
    ))]
    MessageSendAcknowledged(crate::types::UpdateMessageSendAcknowledged),
    /// A message has been successfully sent
    #[serde(rename(
        serialize = "updateMessageSendSucceeded",
        deserialize = "updateMessageSendSucceeded"
    ))]
    MessageSendSucceeded(crate::types::UpdateMessageSendSucceeded),
    /// A message failed to send. Be aware that some messages being sent can be irrecoverably deleted, in which case updateDeleteMessages will be received instead of this update
    #[serde(rename(
        serialize = "updateMessageSendFailed",
        deserialize = "updateMessageSendFailed"
    ))]
    MessageSendFailed(crate::types::UpdateMessageSendFailed),
    /// The message content has changed
    #[serde(rename(
        serialize = "updateMessageContent",
        deserialize = "updateMessageContent"
    ))]
    MessageContent(crate::types::UpdateMessageContent),
    /// A message was edited. Changes in the message content will come in a separate updateMessageContent
    #[serde(rename(serialize = "updateMessageEdited", deserialize = "updateMessageEdited"))]
    MessageEdited(crate::types::UpdateMessageEdited),
    /// The message pinned state was changed
    #[serde(rename(
        serialize = "updateMessageIsPinned",
        deserialize = "updateMessageIsPinned"
    ))]
    MessageIsPinned(crate::types::UpdateMessageIsPinned),
    /// The information about interactions with a message has changed
    #[serde(rename(
        serialize = "updateMessageInteractionInfo",
        deserialize = "updateMessageInteractionInfo"
    ))]
    MessageInteractionInfo(crate::types::UpdateMessageInteractionInfo),
    /// The message content was opened. Updates voice note messages to "listened", video note messages to "viewed" and starts the self-destruct timer
    #[serde(rename(
        serialize = "updateMessageContentOpened",
        deserialize = "updateMessageContentOpened"
    ))]
    MessageContentOpened(crate::types::UpdateMessageContentOpened),
    /// A message with an unread mention was read
    #[serde(rename(
        serialize = "updateMessageMentionRead",
        deserialize = "updateMessageMentionRead"
    ))]
    MessageMentionRead(crate::types::UpdateMessageMentionRead),
    /// The list of unread reactions added to a message was changed
    #[serde(rename(
        serialize = "updateMessageUnreadReactions",
        deserialize = "updateMessageUnreadReactions"
    ))]
    MessageUnreadReactions(crate::types::UpdateMessageUnreadReactions),
    /// A fact-check added to a message was changed
    #[serde(rename(
        serialize = "updateMessageFactCheck",
        deserialize = "updateMessageFactCheck"
    ))]
    MessageFactCheck(crate::types::UpdateMessageFactCheck),
    /// Information about suggested post of a message was changed
    #[serde(rename(
        serialize = "updateMessageSuggestedPostInfo",
        deserialize = "updateMessageSuggestedPostInfo"
    ))]
    MessageSuggestedPostInfo(crate::types::UpdateMessageSuggestedPostInfo),
    /// A message with a live location was viewed. When the update is received, the application is expected to update the live location
    #[serde(rename(
        serialize = "updateMessageLiveLocationViewed",
        deserialize = "updateMessageLiveLocationViewed"
    ))]
    MessageLiveLocationViewed(crate::types::UpdateMessageLiveLocationViewed),
    /// An automatically scheduled message with video has been successfully sent after conversion
    #[serde(rename(
        serialize = "updateVideoPublished",
        deserialize = "updateVideoPublished"
    ))]
    VideoPublished(crate::types::UpdateVideoPublished),
    /// A new chat has been loaded/created. This update is guaranteed to come before the chat identifier is returned to the application. The chat field changes will be reported through separate updates
    #[serde(rename(serialize = "updateNewChat", deserialize = "updateNewChat"))]
    NewChat(crate::types::UpdateNewChat),
    /// The title of a chat was changed
    #[serde(rename(serialize = "updateChatTitle", deserialize = "updateChatTitle"))]
    ChatTitle(crate::types::UpdateChatTitle),
    /// A chat photo was changed
    #[serde(rename(serialize = "updateChatPhoto", deserialize = "updateChatPhoto"))]
    ChatPhoto(crate::types::UpdateChatPhoto),
    /// Chat accent colors have changed
    #[serde(rename(
        serialize = "updateChatAccentColors",
        deserialize = "updateChatAccentColors"
    ))]
    ChatAccentColors(crate::types::UpdateChatAccentColors),
    /// Chat permissions were changed
    #[serde(rename(
        serialize = "updateChatPermissions",
        deserialize = "updateChatPermissions"
    ))]
    ChatPermissions(crate::types::UpdateChatPermissions),
    /// The last message of a chat was changed
    #[serde(rename(
        serialize = "updateChatLastMessage",
        deserialize = "updateChatLastMessage"
    ))]
    ChatLastMessage(crate::types::UpdateChatLastMessage),
    /// The position of a chat in a chat list has changed. An updateChatLastMessage or updateChatDraftMessage update might be sent instead of the update
    #[serde(rename(serialize = "updateChatPosition", deserialize = "updateChatPosition"))]
    ChatPosition(crate::types::UpdateChatPosition),
    /// A chat was added to a chat list
    #[serde(rename(
        serialize = "updateChatAddedToList",
        deserialize = "updateChatAddedToList"
    ))]
    ChatAddedToList(crate::types::UpdateChatAddedToList),
    /// A chat was removed from a chat list
    #[serde(rename(
        serialize = "updateChatRemovedFromList",
        deserialize = "updateChatRemovedFromList"
    ))]
    ChatRemovedFromList(crate::types::UpdateChatRemovedFromList),
    /// Incoming messages were read or the number of unread messages has been changed
    #[serde(rename(serialize = "updateChatReadInbox", deserialize = "updateChatReadInbox"))]
    ChatReadInbox(crate::types::UpdateChatReadInbox),
    /// Outgoing messages were read
    #[serde(rename(
        serialize = "updateChatReadOutbox",
        deserialize = "updateChatReadOutbox"
    ))]
    ChatReadOutbox(crate::types::UpdateChatReadOutbox),
    /// The chat action bar was changed
    #[serde(rename(serialize = "updateChatActionBar", deserialize = "updateChatActionBar"))]
    ChatActionBar(crate::types::UpdateChatActionBar),
    /// The bar for managing business bot was changed in a chat
    #[serde(rename(
        serialize = "updateChatBusinessBotManageBar",
        deserialize = "updateChatBusinessBotManageBar"
    ))]
    ChatBusinessBotManageBar(crate::types::UpdateChatBusinessBotManageBar),
    /// The chat available reactions were changed
    #[serde(rename(
        serialize = "updateChatAvailableReactions",
        deserialize = "updateChatAvailableReactions"
    ))]
    ChatAvailableReactions(crate::types::UpdateChatAvailableReactions),
    /// A chat draft has changed. Be aware that the update may come in the currently opened chat but with old content of the draft. If the user has changed the content of the draft, this update mustn't be applied
    #[serde(rename(
        serialize = "updateChatDraftMessage",
        deserialize = "updateChatDraftMessage"
    ))]
    ChatDraftMessage(crate::types::UpdateChatDraftMessage),
    /// Chat emoji status has changed
    #[serde(rename(
        serialize = "updateChatEmojiStatus",
        deserialize = "updateChatEmojiStatus"
    ))]
    ChatEmojiStatus(crate::types::UpdateChatEmojiStatus),
    /// The message sender that is selected to send messages in a chat has changed
    #[serde(rename(
        serialize = "updateChatMessageSender",
        deserialize = "updateChatMessageSender"
    ))]
    ChatMessageSender(crate::types::UpdateChatMessageSender),
    /// The message auto-delete or self-destruct timer setting for a chat was changed
    #[serde(rename(
        serialize = "updateChatMessageAutoDeleteTime",
        deserialize = "updateChatMessageAutoDeleteTime"
    ))]
    ChatMessageAutoDeleteTime(crate::types::UpdateChatMessageAutoDeleteTime),
    /// Notification settings for a chat were changed
    #[serde(rename(
        serialize = "updateChatNotificationSettings",
        deserialize = "updateChatNotificationSettings"
    ))]
    ChatNotificationSettings(crate::types::UpdateChatNotificationSettings),
    /// The chat pending join requests were changed
    #[serde(rename(
        serialize = "updateChatPendingJoinRequests",
        deserialize = "updateChatPendingJoinRequests"
    ))]
    ChatPendingJoinRequests(crate::types::UpdateChatPendingJoinRequests),
    /// The chat reply markup was changed
    #[serde(rename(
        serialize = "updateChatReplyMarkup",
        deserialize = "updateChatReplyMarkup"
    ))]
    ChatReplyMarkup(crate::types::UpdateChatReplyMarkup),
    /// The chat background was changed
    #[serde(rename(
        serialize = "updateChatBackground",
        deserialize = "updateChatBackground"
    ))]
    ChatBackground(crate::types::UpdateChatBackground),
    /// The chat theme was changed
    #[serde(rename(serialize = "updateChatTheme", deserialize = "updateChatTheme"))]
    ChatTheme(crate::types::UpdateChatTheme),
    /// The chat unread_mention_count has changed
    #[serde(rename(
        serialize = "updateChatUnreadMentionCount",
        deserialize = "updateChatUnreadMentionCount"
    ))]
    ChatUnreadMentionCount(crate::types::UpdateChatUnreadMentionCount),
    /// The chat unread_reaction_count has changed
    #[serde(rename(
        serialize = "updateChatUnreadReactionCount",
        deserialize = "updateChatUnreadReactionCount"
    ))]
    ChatUnreadReactionCount(crate::types::UpdateChatUnreadReactionCount),
    /// A chat video chat state has changed
    #[serde(rename(serialize = "updateChatVideoChat", deserialize = "updateChatVideoChat"))]
    ChatVideoChat(crate::types::UpdateChatVideoChat),
    /// The value of the default disable_notification parameter, used when a message is sent to the chat, was changed
    #[serde(rename(
        serialize = "updateChatDefaultDisableNotification",
        deserialize = "updateChatDefaultDisableNotification"
    ))]
    ChatDefaultDisableNotification(crate::types::UpdateChatDefaultDisableNotification),
    /// A chat content was allowed or restricted for saving
    #[serde(rename(
        serialize = "updateChatHasProtectedContent",
        deserialize = "updateChatHasProtectedContent"
    ))]
    ChatHasProtectedContent(crate::types::UpdateChatHasProtectedContent),
    /// Translation of chat messages was enabled or disabled
    #[serde(rename(
        serialize = "updateChatIsTranslatable",
        deserialize = "updateChatIsTranslatable"
    ))]
    ChatIsTranslatable(crate::types::UpdateChatIsTranslatable),
    /// A chat was marked as unread or was read
    #[serde(rename(
        serialize = "updateChatIsMarkedAsUnread",
        deserialize = "updateChatIsMarkedAsUnread"
    ))]
    ChatIsMarkedAsUnread(crate::types::UpdateChatIsMarkedAsUnread),
    /// A chat default appearance has changed
    #[serde(rename(
        serialize = "updateChatViewAsTopics",
        deserialize = "updateChatViewAsTopics"
    ))]
    ChatViewAsTopics(crate::types::UpdateChatViewAsTopics),
    /// A chat was blocked or unblocked
    #[serde(rename(serialize = "updateChatBlockList", deserialize = "updateChatBlockList"))]
    ChatBlockList(crate::types::UpdateChatBlockList),
    /// A chat's has_scheduled_messages field has changed
    #[serde(rename(
        serialize = "updateChatHasScheduledMessages",
        deserialize = "updateChatHasScheduledMessages"
    ))]
    ChatHasScheduledMessages(crate::types::UpdateChatHasScheduledMessages),
    /// The list of chat folders or a chat folder has changed
    #[serde(rename(serialize = "updateChatFolders", deserialize = "updateChatFolders"))]
    ChatFolders(crate::types::UpdateChatFolders),
    /// The number of online group members has changed. This update with non-zero number of online group members is sent only for currently opened chats.
    /// There is no guarantee that it is sent just after the number of online users has changed
    #[serde(rename(
        serialize = "updateChatOnlineMemberCount",
        deserialize = "updateChatOnlineMemberCount"
    ))]
    ChatOnlineMemberCount(crate::types::UpdateChatOnlineMemberCount),
    /// Basic information about a Saved Messages topic has changed. This update is guaranteed to come before the topic identifier is returned to the application
    #[serde(rename(
        serialize = "updateSavedMessagesTopic",
        deserialize = "updateSavedMessagesTopic"
    ))]
    SavedMessagesTopic(crate::types::UpdateSavedMessagesTopic),
    /// Number of Saved Messages topics has changed
    #[serde(rename(
        serialize = "updateSavedMessagesTopicCount",
        deserialize = "updateSavedMessagesTopicCount"
    ))]
    SavedMessagesTopicCount(crate::types::UpdateSavedMessagesTopicCount),
    /// Basic information about a topic in a channel direct messages chat administered by the current user has changed. This update is guaranteed to come before the topic identifier is returned to the application
    #[serde(rename(
        serialize = "updateDirectMessagesChatTopic",
        deserialize = "updateDirectMessagesChatTopic"
    ))]
    DirectMessagesChatTopic(crate::types::UpdateDirectMessagesChatTopic),
    /// Number of messages in a topic has changed; for Saved Messages and channel direct messages chat topics only
    #[serde(rename(
        serialize = "updateTopicMessageCount",
        deserialize = "updateTopicMessageCount"
    ))]
    TopicMessageCount(crate::types::UpdateTopicMessageCount),
    /// Basic information about a quick reply shortcut has changed. This update is guaranteed to come before the quick shortcut name is returned to the application
    #[serde(rename(
        serialize = "updateQuickReplyShortcut",
        deserialize = "updateQuickReplyShortcut"
    ))]
    QuickReplyShortcut(crate::types::UpdateQuickReplyShortcut),
    /// A quick reply shortcut and all its messages were deleted
    #[serde(rename(
        serialize = "updateQuickReplyShortcutDeleted",
        deserialize = "updateQuickReplyShortcutDeleted"
    ))]
    QuickReplyShortcutDeleted(crate::types::UpdateQuickReplyShortcutDeleted),
    /// The list of quick reply shortcuts has changed
    #[serde(rename(
        serialize = "updateQuickReplyShortcuts",
        deserialize = "updateQuickReplyShortcuts"
    ))]
    QuickReplyShortcuts(crate::types::UpdateQuickReplyShortcuts),
    /// The list of quick reply shortcut messages has changed
    #[serde(rename(
        serialize = "updateQuickReplyShortcutMessages",
        deserialize = "updateQuickReplyShortcutMessages"
    ))]
    QuickReplyShortcutMessages(crate::types::UpdateQuickReplyShortcutMessages),
    /// Basic information about a topic in a forum chat was changed
    #[serde(rename(
        serialize = "updateForumTopicInfo",
        deserialize = "updateForumTopicInfo"
    ))]
    ForumTopicInfo(crate::types::UpdateForumTopicInfo),
    /// Information about a topic in a forum chat was changed
    #[serde(rename(serialize = "updateForumTopic", deserialize = "updateForumTopic"))]
    ForumTopic(crate::types::UpdateForumTopic),
    /// Notification settings for some type of chats were updated
    #[serde(rename(
        serialize = "updateScopeNotificationSettings",
        deserialize = "updateScopeNotificationSettings"
    ))]
    ScopeNotificationSettings(crate::types::UpdateScopeNotificationSettings),
    /// Notification settings for reactions were updated
    #[serde(rename(
        serialize = "updateReactionNotificationSettings",
        deserialize = "updateReactionNotificationSettings"
    ))]
    ReactionNotificationSettings(crate::types::UpdateReactionNotificationSettings),
    /// A notification was changed
    #[serde(rename(serialize = "updateNotification", deserialize = "updateNotification"))]
    Notification(crate::types::UpdateNotification),
    /// A list of active notifications in a notification group has changed
    #[serde(rename(
        serialize = "updateNotificationGroup",
        deserialize = "updateNotificationGroup"
    ))]
    NotificationGroup(crate::types::UpdateNotificationGroup),
    /// Contains active notifications that were shown on previous application launches. This update is sent only if the message database is used. In that case it comes once before any updateNotification and updateNotificationGroup update
    #[serde(rename(
        serialize = "updateActiveNotifications",
        deserialize = "updateActiveNotifications"
    ))]
    ActiveNotifications(crate::types::UpdateActiveNotifications),
    /// Describes whether there are some pending notification updates. Can be used to prevent application from killing, while there are some pending notifications
    #[serde(rename(
        serialize = "updateHavePendingNotifications",
        deserialize = "updateHavePendingNotifications"
    ))]
    HavePendingNotifications(crate::types::UpdateHavePendingNotifications),
    /// Some messages were deleted
    #[serde(rename(
        serialize = "updateDeleteMessages",
        deserialize = "updateDeleteMessages"
    ))]
    DeleteMessages(crate::types::UpdateDeleteMessages),
    /// A message sender activity in the chat has changed
    #[serde(rename(serialize = "updateChatAction", deserialize = "updateChatAction"))]
    ChatAction(crate::types::UpdateChatAction),
    /// A new pending text message was received in a chat with a bot. The message must be shown in the chat for at most getOption("pending_text_message_period") seconds,
    /// replace any other pending message with the same draft_id, and be deleted whenever any incoming message from the bot in the message thread is received
    #[serde(rename(
        serialize = "updatePendingTextMessage",
        deserialize = "updatePendingTextMessage"
    ))]
    PendingTextMessage(crate::types::UpdatePendingTextMessage),
    /// The user went online or offline
    #[serde(rename(serialize = "updateUserStatus", deserialize = "updateUserStatus"))]
    UserStatus(crate::types::UpdateUserStatus),
    /// Some data of a user has changed. This update is guaranteed to come before the user identifier is returned to the application
    #[serde(rename(serialize = "updateUser", deserialize = "updateUser"))]
    User(crate::types::UpdateUser),
    /// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the application
    #[serde(rename(serialize = "updateBasicGroup", deserialize = "updateBasicGroup"))]
    BasicGroup(crate::types::UpdateBasicGroup),
    /// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the application
    #[serde(rename(serialize = "updateSupergroup", deserialize = "updateSupergroup"))]
    Supergroup(crate::types::UpdateSupergroup),
    /// Some data of a secret chat has changed. This update is guaranteed to come before the secret chat identifier is returned to the application
    #[serde(rename(serialize = "updateSecretChat", deserialize = "updateSecretChat"))]
    SecretChat(crate::types::UpdateSecretChat),
    /// Some data in userFullInfo has been changed
    #[serde(rename(serialize = "updateUserFullInfo", deserialize = "updateUserFullInfo"))]
    UserFullInfo(crate::types::UpdateUserFullInfo),
    /// Some data in basicGroupFullInfo has been changed
    #[serde(rename(
        serialize = "updateBasicGroupFullInfo",
        deserialize = "updateBasicGroupFullInfo"
    ))]
    BasicGroupFullInfo(crate::types::UpdateBasicGroupFullInfo),
    /// Some data in supergroupFullInfo has been changed
    #[serde(rename(
        serialize = "updateSupergroupFullInfo",
        deserialize = "updateSupergroupFullInfo"
    ))]
    SupergroupFullInfo(crate::types::UpdateSupergroupFullInfo),
    /// A service notification from the server was received. Upon receiving this the application must show a popup with the content of the notification
    #[serde(rename(
        serialize = "updateServiceNotification",
        deserialize = "updateServiceNotification"
    ))]
    ServiceNotification(crate::types::UpdateServiceNotification),
    /// An OAuth authorization request was received
    #[serde(rename(
        serialize = "updateNewOauthRequest",
        deserialize = "updateNewOauthRequest"
    ))]
    NewOauthRequest(crate::types::UpdateNewOauthRequest),
    /// Information about a file was updated
    #[serde(rename(serialize = "updateFile", deserialize = "updateFile"))]
    File(crate::types::UpdateFile),
    /// The file generation process needs to be started by the application. Use setFileGenerationProgress and finishFileGeneration to generate the file
    #[serde(rename(
        serialize = "updateFileGenerationStart",
        deserialize = "updateFileGenerationStart"
    ))]
    FileGenerationStart(crate::types::UpdateFileGenerationStart),
    /// File generation is no longer needed
    #[serde(rename(
        serialize = "updateFileGenerationStop",
        deserialize = "updateFileGenerationStop"
    ))]
    FileGenerationStop(crate::types::UpdateFileGenerationStop),
    /// The state of the file download list has changed
    #[serde(rename(serialize = "updateFileDownloads", deserialize = "updateFileDownloads"))]
    FileDownloads(crate::types::UpdateFileDownloads),
    /// A file was added to the file download list. This update is sent only after file download list is loaded for the first time
    #[serde(rename(
        serialize = "updateFileAddedToDownloads",
        deserialize = "updateFileAddedToDownloads"
    ))]
    FileAddedToDownloads(crate::types::UpdateFileAddedToDownloads),
    /// A file download was changed. This update is sent only after file download list is loaded for the first time
    #[serde(rename(serialize = "updateFileDownload", deserialize = "updateFileDownload"))]
    FileDownload(crate::types::UpdateFileDownload),
    /// A file was removed from the file download list. This update is sent only after file download list is loaded for the first time
    #[serde(rename(
        serialize = "updateFileRemovedFromDownloads",
        deserialize = "updateFileRemovedFromDownloads"
    ))]
    FileRemovedFromDownloads(crate::types::UpdateFileRemovedFromDownloads),
    /// A request can't be completed unless application verification is performed; for official mobile applications only.
    /// The method setApplicationVerificationToken must be called once the verification is completed or failed
    #[serde(rename(
        serialize = "updateApplicationVerificationRequired",
        deserialize = "updateApplicationVerificationRequired"
    ))]
    ApplicationVerificationRequired(crate::types::UpdateApplicationVerificationRequired),
    /// A request can't be completed unless reCAPTCHA verification is performed; for official mobile applications only.
    /// The method setApplicationVerificationToken must be called once the verification is completed or failed
    #[serde(rename(
        serialize = "updateApplicationRecaptchaVerificationRequired",
        deserialize = "updateApplicationRecaptchaVerificationRequired"
    ))]
    ApplicationRecaptchaVerificationRequired(
        crate::types::UpdateApplicationRecaptchaVerificationRequired,
    ),
    /// New call was created or information about a call was updated
    #[serde(rename(serialize = "updateCall", deserialize = "updateCall"))]
    Call(crate::types::UpdateCall),
    /// Information about a group call was updated
    #[serde(rename(serialize = "updateGroupCall", deserialize = "updateGroupCall"))]
    GroupCall(crate::types::UpdateGroupCall),
    /// Information about a group call participant was changed. The updates are sent only after the group call is received through getGroupCall and only if the call is joined or being joined
    #[serde(rename(
        serialize = "updateGroupCallParticipant",
        deserialize = "updateGroupCallParticipant"
    ))]
    GroupCallParticipant(crate::types::UpdateGroupCallParticipant),
    /// The list of group call participants that can send and receive encrypted call data has changed; for group calls not bound to a chat only
    #[serde(rename(
        serialize = "updateGroupCallParticipants",
        deserialize = "updateGroupCallParticipants"
    ))]
    GroupCallParticipants(crate::types::UpdateGroupCallParticipants),
    /// The verification state of an encrypted group call has changed; for group calls not bound to a chat only
    #[serde(rename(
        serialize = "updateGroupCallVerificationState",
        deserialize = "updateGroupCallVerificationState"
    ))]
    GroupCallVerificationState(crate::types::UpdateGroupCallVerificationState),
    /// A new message was received in a group call
    #[serde(rename(
        serialize = "updateNewGroupCallMessage",
        deserialize = "updateNewGroupCallMessage"
    ))]
    NewGroupCallMessage(crate::types::UpdateNewGroupCallMessage),
    /// A new paid reaction was received in a live story group call
    #[serde(rename(
        serialize = "updateNewGroupCallPaidReaction",
        deserialize = "updateNewGroupCallPaidReaction"
    ))]
    NewGroupCallPaidReaction(crate::types::UpdateNewGroupCallPaidReaction),
    /// A group call message failed to send
    #[serde(rename(
        serialize = "updateGroupCallMessageSendFailed",
        deserialize = "updateGroupCallMessageSendFailed"
    ))]
    GroupCallMessageSendFailed(crate::types::UpdateGroupCallMessageSendFailed),
    /// Some group call messages were deleted
    #[serde(rename(
        serialize = "updateGroupCallMessagesDeleted",
        deserialize = "updateGroupCallMessagesDeleted"
    ))]
    GroupCallMessagesDeleted(crate::types::UpdateGroupCallMessagesDeleted),
    /// The list of top donors in live story group call has changed
    #[serde(rename(
        serialize = "updateLiveStoryTopDonors",
        deserialize = "updateLiveStoryTopDonors"
    ))]
    LiveStoryTopDonors(crate::types::UpdateLiveStoryTopDonors),
    /// New call signaling data arrived
    #[serde(rename(
        serialize = "updateNewCallSignalingData",
        deserialize = "updateNewCallSignalingData"
    ))]
    NewCallSignalingData(crate::types::UpdateNewCallSignalingData),
    /// State of a gift auction was updated
    #[serde(rename(
        serialize = "updateGiftAuctionState",
        deserialize = "updateGiftAuctionState"
    ))]
    GiftAuctionState(crate::types::UpdateGiftAuctionState),
    /// The list of auctions in which participate the current user has changed
    #[serde(rename(
        serialize = "updateActiveGiftAuctions",
        deserialize = "updateActiveGiftAuctions"
    ))]
    ActiveGiftAuctions(crate::types::UpdateActiveGiftAuctions),
    /// Some privacy setting rules have been changed
    #[serde(rename(
        serialize = "updateUserPrivacySettingRules",
        deserialize = "updateUserPrivacySettingRules"
    ))]
    UserPrivacySettingRules(crate::types::UpdateUserPrivacySettingRules),
    /// Number of unread messages in a chat list has changed. This update is sent only if the message database is used
    #[serde(rename(
        serialize = "updateUnreadMessageCount",
        deserialize = "updateUnreadMessageCount"
    ))]
    UnreadMessageCount(crate::types::UpdateUnreadMessageCount),
    /// Number of unread chats, i.e. with unread messages or marked as unread, has changed. This update is sent only if the message database is used
    #[serde(rename(
        serialize = "updateUnreadChatCount",
        deserialize = "updateUnreadChatCount"
    ))]
    UnreadChatCount(crate::types::UpdateUnreadChatCount),
    /// A story was changed
    #[serde(rename(serialize = "updateStory", deserialize = "updateStory"))]
    Story(crate::types::UpdateStory),
    /// A story became inaccessible
    #[serde(rename(serialize = "updateStoryDeleted", deserialize = "updateStoryDeleted"))]
    StoryDeleted(crate::types::UpdateStoryDeleted),
    /// A story has been successfully posted
    #[serde(rename(
        serialize = "updateStoryPostSucceeded",
        deserialize = "updateStoryPostSucceeded"
    ))]
    StoryPostSucceeded(crate::types::UpdateStoryPostSucceeded),
    /// A story failed to post. If the story posting is canceled, then updateStoryDeleted will be received instead of this update
    #[serde(rename(
        serialize = "updateStoryPostFailed",
        deserialize = "updateStoryPostFailed"
    ))]
    StoryPostFailed(crate::types::UpdateStoryPostFailed),
    /// The list of active stories posted by a specific chat has changed
    #[serde(rename(
        serialize = "updateChatActiveStories",
        deserialize = "updateChatActiveStories"
    ))]
    ChatActiveStories(crate::types::UpdateChatActiveStories),
    /// Number of chats in a story list has changed
    #[serde(rename(
        serialize = "updateStoryListChatCount",
        deserialize = "updateStoryListChatCount"
    ))]
    StoryListChatCount(crate::types::UpdateStoryListChatCount),
    /// Story stealth mode settings have changed
    #[serde(rename(
        serialize = "updateStoryStealthMode",
        deserialize = "updateStoryStealthMode"
    ))]
    StoryStealthMode(crate::types::UpdateStoryStealthMode),
    /// Lists of bots which Mini Apps must be allowed to read text from clipboard and must be opened without a warning
    #[serde(rename(
        serialize = "updateTrustedMiniAppBots",
        deserialize = "updateTrustedMiniAppBots"
    ))]
    TrustedMiniAppBots(crate::types::UpdateTrustedMiniAppBots),
    /// An option changed its value
    #[serde(rename(serialize = "updateOption", deserialize = "updateOption"))]
    Option(crate::types::UpdateOption),
    /// A sticker set has changed
    #[serde(rename(serialize = "updateStickerSet", deserialize = "updateStickerSet"))]
    StickerSet(crate::types::UpdateStickerSet),
    /// The list of installed sticker sets was updated
    #[serde(rename(
        serialize = "updateInstalledStickerSets",
        deserialize = "updateInstalledStickerSets"
    ))]
    InstalledStickerSets(crate::types::UpdateInstalledStickerSets),
    /// The list of trending sticker sets was updated or some of them were viewed
    #[serde(rename(
        serialize = "updateTrendingStickerSets",
        deserialize = "updateTrendingStickerSets"
    ))]
    TrendingStickerSets(crate::types::UpdateTrendingStickerSets),
    /// The list of recently used stickers was updated
    #[serde(rename(
        serialize = "updateRecentStickers",
        deserialize = "updateRecentStickers"
    ))]
    RecentStickers(crate::types::UpdateRecentStickers),
    /// The list of favorite stickers was updated
    #[serde(rename(
        serialize = "updateFavoriteStickers",
        deserialize = "updateFavoriteStickers"
    ))]
    FavoriteStickers(crate::types::UpdateFavoriteStickers),
    /// The list of saved animations was updated
    #[serde(rename(
        serialize = "updateSavedAnimations",
        deserialize = "updateSavedAnimations"
    ))]
    SavedAnimations(crate::types::UpdateSavedAnimations),
    /// The list of saved notification sounds was updated. This update may not be sent until information about a notification sound was requested for the first time
    #[serde(rename(
        serialize = "updateSavedNotificationSounds",
        deserialize = "updateSavedNotificationSounds"
    ))]
    SavedNotificationSounds(crate::types::UpdateSavedNotificationSounds),
    /// The default background has changed
    #[serde(rename(
        serialize = "updateDefaultBackground",
        deserialize = "updateDefaultBackground"
    ))]
    DefaultBackground(crate::types::UpdateDefaultBackground),
    /// The list of available emoji chat themes has changed
    #[serde(rename(
        serialize = "updateEmojiChatThemes",
        deserialize = "updateEmojiChatThemes"
    ))]
    EmojiChatThemes(crate::types::UpdateEmojiChatThemes),
    /// The list of supported accent colors has changed
    #[serde(rename(serialize = "updateAccentColors", deserialize = "updateAccentColors"))]
    AccentColors(crate::types::UpdateAccentColors),
    /// The list of supported accent colors for user profiles has changed
    #[serde(rename(
        serialize = "updateProfileAccentColors",
        deserialize = "updateProfileAccentColors"
    ))]
    ProfileAccentColors(crate::types::UpdateProfileAccentColors),
    /// Some language pack strings have been updated
    #[serde(rename(
        serialize = "updateLanguagePackStrings",
        deserialize = "updateLanguagePackStrings"
    ))]
    LanguagePackStrings(crate::types::UpdateLanguagePackStrings),
    /// The connection state has changed. This update must be used only to show a human-readable description of the connection state
    #[serde(rename(
        serialize = "updateConnectionState",
        deserialize = "updateConnectionState"
    ))]
    ConnectionState(crate::types::UpdateConnectionState),
    /// The freeze state of the current user's account has changed
    #[serde(rename(serialize = "updateFreezeState", deserialize = "updateFreezeState"))]
    FreezeState(crate::types::UpdateFreezeState),
    /// The parameters for age verification of the current user's account has changed
    #[serde(rename(
        serialize = "updateAgeVerificationParameters",
        deserialize = "updateAgeVerificationParameters"
    ))]
    AgeVerificationParameters(crate::types::UpdateAgeVerificationParameters),
    /// New terms of service must be accepted by the user. If the terms of service are declined, then the deleteAccount method must be called with the reason "Decline ToS update"
    #[serde(rename(
        serialize = "updateTermsOfService",
        deserialize = "updateTermsOfService"
    ))]
    TermsOfService(crate::types::UpdateTermsOfService),
    /// The first unconfirmed session has changed
    #[serde(rename(
        serialize = "updateUnconfirmedSession",
        deserialize = "updateUnconfirmedSession"
    ))]
    UnconfirmedSession(crate::types::UpdateUnconfirmedSession),
    /// The list of bots added to attachment or side menu has changed
    #[serde(rename(
        serialize = "updateAttachmentMenuBots",
        deserialize = "updateAttachmentMenuBots"
    ))]
    AttachmentMenuBots(crate::types::UpdateAttachmentMenuBots),
    /// A message was sent by an opened Web App, so the Web App needs to be closed
    #[serde(rename(
        serialize = "updateWebAppMessageSent",
        deserialize = "updateWebAppMessageSent"
    ))]
    WebAppMessageSent(crate::types::UpdateWebAppMessageSent),
    /// The list of active emoji reactions has changed
    #[serde(rename(
        serialize = "updateActiveEmojiReactions",
        deserialize = "updateActiveEmojiReactions"
    ))]
    ActiveEmojiReactions(crate::types::UpdateActiveEmojiReactions),
    /// The list of available message effects has changed
    #[serde(rename(
        serialize = "updateAvailableMessageEffects",
        deserialize = "updateAvailableMessageEffects"
    ))]
    AvailableMessageEffects(crate::types::UpdateAvailableMessageEffects),
    /// The type of default reaction has changed
    #[serde(rename(
        serialize = "updateDefaultReactionType",
        deserialize = "updateDefaultReactionType"
    ))]
    DefaultReactionType(crate::types::UpdateDefaultReactionType),
    /// The type of default paid reaction has changed
    #[serde(rename(
        serialize = "updateDefaultPaidReactionType",
        deserialize = "updateDefaultPaidReactionType"
    ))]
    DefaultPaidReactionType(crate::types::UpdateDefaultPaidReactionType),
    /// Tags used in Saved Messages or a Saved Messages topic have changed
    #[serde(rename(
        serialize = "updateSavedMessagesTags",
        deserialize = "updateSavedMessagesTags"
    ))]
    SavedMessagesTags(crate::types::UpdateSavedMessagesTags),
    /// The list of messages with active live location that need to be updated by the application has changed. The list is persistent across application restarts only if the message database is used
    #[serde(rename(
        serialize = "updateActiveLiveLocationMessages",
        deserialize = "updateActiveLiveLocationMessages"
    ))]
    ActiveLiveLocationMessages(crate::types::UpdateActiveLiveLocationMessages),
    /// The number of Telegram Stars owned by the current user has changed
    #[serde(rename(
        serialize = "updateOwnedStarCount",
        deserialize = "updateOwnedStarCount"
    ))]
    OwnedStarCount(crate::types::UpdateOwnedStarCount),
    /// The number of Toncoins owned by the current user has changed
    #[serde(rename(serialize = "updateOwnedTonCount", deserialize = "updateOwnedTonCount"))]
    OwnedTonCount(crate::types::UpdateOwnedTonCount),
    /// The revenue earned from sponsored messages in a chat has changed. If chat revenue screen is opened, then getChatRevenueTransactions may be called to fetch new transactions
    #[serde(rename(
        serialize = "updateChatRevenueAmount",
        deserialize = "updateChatRevenueAmount"
    ))]
    ChatRevenueAmount(crate::types::UpdateChatRevenueAmount),
    /// The Telegram Star revenue earned by a user or a chat has changed. If Telegram Star transaction screen of the chat is opened, then getStarTransactions may be called to fetch new transactions
    #[serde(rename(
        serialize = "updateStarRevenueStatus",
        deserialize = "updateStarRevenueStatus"
    ))]
    StarRevenueStatus(crate::types::UpdateStarRevenueStatus),
    /// The Toncoin revenue earned by the current user has changed. If Toncoin transaction screen of the chat is opened, then getTonTransactions may be called to fetch new transactions
    #[serde(rename(
        serialize = "updateTonRevenueStatus",
        deserialize = "updateTonRevenueStatus"
    ))]
    TonRevenueStatus(crate::types::UpdateTonRevenueStatus),
    /// The parameters of speech recognition without Telegram Premium subscription has changed
    #[serde(rename(
        serialize = "updateSpeechRecognitionTrial",
        deserialize = "updateSpeechRecognitionTrial"
    ))]
    SpeechRecognitionTrial(crate::types::UpdateSpeechRecognitionTrial),
    /// The levels of live story group call messages have changed
    #[serde(rename(
        serialize = "updateGroupCallMessageLevels",
        deserialize = "updateGroupCallMessageLevels"
    ))]
    GroupCallMessageLevels(crate::types::UpdateGroupCallMessageLevels),
    /// The list of supported dice emojis has changed
    #[serde(rename(serialize = "updateDiceEmojis", deserialize = "updateDiceEmojis"))]
    DiceEmojis(crate::types::UpdateDiceEmojis),
    /// The stake dice state has changed
    #[serde(rename(
        serialize = "updateStakeDiceState",
        deserialize = "updateStakeDiceState"
    ))]
    StakeDiceState(crate::types::UpdateStakeDiceState),
    /// Some animated emoji message was clicked and a big animated sticker must be played if the message is visible on the screen. chatActionWatchingAnimations with the text of the message needs to be sent if the sticker is played
    #[serde(rename(
        serialize = "updateAnimatedEmojiMessageClicked",
        deserialize = "updateAnimatedEmojiMessageClicked"
    ))]
    AnimatedEmojiMessageClicked(crate::types::UpdateAnimatedEmojiMessageClicked),
    /// The parameters of animation search through getOption("animation_search_bot_username") bot has changed
    #[serde(rename(
        serialize = "updateAnimationSearchParameters",
        deserialize = "updateAnimationSearchParameters"
    ))]
    AnimationSearchParameters(crate::types::UpdateAnimationSearchParameters),
    /// The list of suggested to the user actions has changed
    #[serde(rename(
        serialize = "updateSuggestedActions",
        deserialize = "updateSuggestedActions"
    ))]
    SuggestedActions(crate::types::UpdateSuggestedActions),
    /// Download or upload file speed for the user was limited, but it can be restored by subscription to Telegram Premium. The notification can be postponed until a being downloaded or uploaded file is visible to the user.
    /// Use getOption("premium_download_speedup") or getOption("premium_upload_speedup") to get expected speedup after subscription to Telegram Premium
    #[serde(rename(
        serialize = "updateSpeedLimitNotification",
        deserialize = "updateSpeedLimitNotification"
    ))]
    SpeedLimitNotification(crate::types::UpdateSpeedLimitNotification),
    /// The list of contacts that had birthdays recently or will have birthday soon has changed
    #[serde(rename(
        serialize = "updateContactCloseBirthdays",
        deserialize = "updateContactCloseBirthdays"
    ))]
    ContactCloseBirthdays(crate::types::UpdateContactCloseBirthdays),
    /// Autosave settings for some type of chats were updated
    #[serde(rename(
        serialize = "updateAutosaveSettings",
        deserialize = "updateAutosaveSettings"
    ))]
    AutosaveSettings(crate::types::UpdateAutosaveSettings),
}

/// 机器人收到的按钮回调更新。
///
/// 这里只补当前分页功能所需的字段：
/// - `id` 用于 `answerCallbackQuery`
/// - `sender_user_id/chat_id/message_id` 用于权限校验与编辑原消息
/// - `payload` 用于解析按钮携带的数据
#[serde_as]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpdateNewCallbackQuery {
    /// 回调查询唯一 ID。
    #[serde_as(as = "DisplayFromStr")]
    pub id: i64,
    /// 点击按钮的用户 ID。
    pub sender_user_id: i64,
    /// 按钮所在聊天 ID。
    pub chat_id: i64,
    /// 按钮所在消息 ID。
    pub message_id: i64,
    /// 聊天实例 ID。
    #[serde_as(as = "DisplayFromStr")]
    pub chat_instance: i64,
    /// 按钮携带的回调数据。
    pub payload: crate::enums::CallbackQueryPayload,
}
