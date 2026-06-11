mod error;
pub use error::Error;

mod authentication_code_type_telegram_message;
pub use authentication_code_type_telegram_message::AuthenticationCodeTypeTelegramMessage;

mod authentication_code_type_sms;
pub use authentication_code_type_sms::AuthenticationCodeTypeSms;

mod authentication_code_type_sms_word;
pub use authentication_code_type_sms_word::AuthenticationCodeTypeSmsWord;

mod authentication_code_type_sms_phrase;
pub use authentication_code_type_sms_phrase::AuthenticationCodeTypeSmsPhrase;

mod authentication_code_type_call;
pub use authentication_code_type_call::AuthenticationCodeTypeCall;

mod authentication_code_type_flash_call;
pub use authentication_code_type_flash_call::AuthenticationCodeTypeFlashCall;

mod authentication_code_type_missed_call;
pub use authentication_code_type_missed_call::AuthenticationCodeTypeMissedCall;

mod authentication_code_type_fragment;
pub use authentication_code_type_fragment::AuthenticationCodeTypeFragment;

mod authentication_code_type_firebase_android;
pub use authentication_code_type_firebase_android::AuthenticationCodeTypeFirebaseAndroid;

mod authentication_code_type_firebase_ios;
pub use authentication_code_type_firebase_ios::AuthenticationCodeTypeFirebaseIos;

mod authentication_code_info;
pub use authentication_code_info::AuthenticationCodeInfo;

mod email_address_authentication_code_info;
pub use email_address_authentication_code_info::EmailAddressAuthenticationCodeInfo;

mod email_address_authentication_code;
pub use email_address_authentication_code::EmailAddressAuthenticationCode;

mod email_address_authentication_apple_id;
pub use email_address_authentication_apple_id::EmailAddressAuthenticationAppleId;

mod email_address_authentication_google_id;
pub use email_address_authentication_google_id::EmailAddressAuthenticationGoogleId;

mod email_address_reset_state_available;
pub use email_address_reset_state_available::EmailAddressResetStateAvailable;

mod email_address_reset_state_pending;
pub use email_address_reset_state_pending::EmailAddressResetStatePending;

mod text_entity;
pub use text_entity::TextEntity;

mod text_entities;
pub use text_entities::TextEntities;

mod formatted_text;
pub use formatted_text::FormattedText;

mod terms_of_service;
pub use terms_of_service::TermsOfService;

mod passkey;
pub use passkey::Passkey;

mod passkeys;
pub use passkeys::Passkeys;

mod authorization_state_wait_premium_purchase;
pub use authorization_state_wait_premium_purchase::AuthorizationStateWaitPremiumPurchase;

mod authorization_state_wait_email_address;
pub use authorization_state_wait_email_address::AuthorizationStateWaitEmailAddress;

mod authorization_state_wait_email_code;
pub use authorization_state_wait_email_code::AuthorizationStateWaitEmailCode;

mod authorization_state_wait_code;
pub use authorization_state_wait_code::AuthorizationStateWaitCode;

mod authorization_state_wait_other_device_confirmation;
pub use authorization_state_wait_other_device_confirmation::AuthorizationStateWaitOtherDeviceConfirmation;

mod authorization_state_wait_registration;
pub use authorization_state_wait_registration::AuthorizationStateWaitRegistration;

mod authorization_state_wait_password;
pub use authorization_state_wait_password::AuthorizationStateWaitPassword;

mod firebase_device_verification_parameters_safety_net;
pub use firebase_device_verification_parameters_safety_net::FirebaseDeviceVerificationParametersSafetyNet;

mod firebase_device_verification_parameters_play_integrity;
pub use firebase_device_verification_parameters_play_integrity::FirebaseDeviceVerificationParametersPlayIntegrity;

mod password_state;
pub use password_state::PasswordState;

mod recovery_email_address;
pub use recovery_email_address::RecoveryEmailAddress;

mod temporary_password_state;
pub use temporary_password_state::TemporaryPasswordState;

mod local_file;
pub use local_file::LocalFile;

mod remote_file;
pub use remote_file::RemoteFile;

mod file;
pub use file::File;

mod input_file_id;
pub use input_file_id::InputFileId;

mod input_file_remote;
pub use input_file_remote::InputFileRemote;

mod input_file_local;
pub use input_file_local::InputFileLocal;

mod input_file_generated;
pub use input_file_generated::InputFileGenerated;

mod photo_size;
pub use photo_size::PhotoSize;

mod minithumbnail;
pub use minithumbnail::Minithumbnail;

mod thumbnail;
pub use thumbnail::Thumbnail;

mod mask_position;
pub use mask_position::MaskPosition;

mod sticker_full_type_regular;
pub use sticker_full_type_regular::StickerFullTypeRegular;

mod sticker_full_type_mask;
pub use sticker_full_type_mask::StickerFullTypeMask;

mod sticker_full_type_custom_emoji;
pub use sticker_full_type_custom_emoji::StickerFullTypeCustomEmoji;

mod closed_vector_path;
pub use closed_vector_path::ClosedVectorPath;

mod outline;
pub use outline::Outline;

mod poll_option;
pub use poll_option::PollOption;

mod poll_type_regular;
pub use poll_type_regular::PollTypeRegular;

mod poll_type_quiz;
pub use poll_type_quiz::PollTypeQuiz;

mod checklist_task;
pub use checklist_task::ChecklistTask;

mod input_checklist_task;
pub use input_checklist_task::InputChecklistTask;

mod checklist;
pub use checklist::Checklist;

mod input_checklist;
pub use input_checklist::InputChecklist;

mod animation;
pub use animation::Animation;

mod audio;
pub use audio::Audio;

mod audios;
pub use audios::Audios;

mod document;
pub use document::Document;

mod photo;
pub use photo::Photo;

mod sticker;
pub use sticker::Sticker;

mod video;
pub use video::Video;

mod video_note;
pub use video_note::VideoNote;

mod voice_note;
pub use voice_note::VoiceNote;

mod animated_emoji;
pub use animated_emoji::AnimatedEmoji;

mod contact;
pub use contact::Contact;

mod location;
pub use location::Location;

mod venue;
pub use venue::Venue;

mod game;
pub use game::Game;

mod stake_dice_state;
pub use stake_dice_state::StakeDiceState;

mod web_app;
pub use web_app::WebApp;

mod poll;
pub use poll::Poll;

mod alternative_video;
pub use alternative_video::AlternativeVideo;

mod video_storyboard;
pub use video_storyboard::VideoStoryboard;

mod background;
pub use background::Background;

mod backgrounds;
pub use backgrounds::Backgrounds;

mod chat_background;
pub use chat_background::ChatBackground;

mod profile_photo;
pub use profile_photo::ProfilePhoto;

mod chat_photo_info;
pub use chat_photo_info::ChatPhotoInfo;

mod user_type_bot;
pub use user_type_bot::UserTypeBot;

mod bot_command;
pub use bot_command::BotCommand;

mod bot_commands;
pub use bot_commands::BotCommands;

mod bot_menu_button;
pub use bot_menu_button::BotMenuButton;

mod bot_verification_parameters;
pub use bot_verification_parameters::BotVerificationParameters;

mod bot_verification;
pub use bot_verification::BotVerification;

mod verification_status;
pub use verification_status::VerificationStatus;

mod chat_location;
pub use chat_location::ChatLocation;

mod birthdate;
pub use birthdate::Birthdate;

mod close_birthday_user;
pub use close_birthday_user::CloseBirthdayUser;

mod business_away_message_schedule_custom;
pub use business_away_message_schedule_custom::BusinessAwayMessageScheduleCustom;

mod business_location;
pub use business_location::BusinessLocation;

mod business_recipients;
pub use business_recipients::BusinessRecipients;

mod business_away_message_settings;
pub use business_away_message_settings::BusinessAwayMessageSettings;

mod business_greeting_message_settings;
pub use business_greeting_message_settings::BusinessGreetingMessageSettings;

mod business_bot_rights;
pub use business_bot_rights::BusinessBotRights;

mod business_connected_bot;
pub use business_connected_bot::BusinessConnectedBot;

mod business_start_page;
pub use business_start_page::BusinessStartPage;

mod input_business_start_page;
pub use input_business_start_page::InputBusinessStartPage;

mod business_opening_hours_interval;
pub use business_opening_hours_interval::BusinessOpeningHoursInterval;

mod business_opening_hours;
pub use business_opening_hours::BusinessOpeningHours;

mod business_info;
pub use business_info::BusinessInfo;

mod business_chat_link;
pub use business_chat_link::BusinessChatLink;

mod business_chat_links;
pub use business_chat_links::BusinessChatLinks;

mod input_business_chat_link;
pub use input_business_chat_link::InputBusinessChatLink;

mod business_chat_link_info;
pub use business_chat_link_info::BusinessChatLinkInfo;

mod chat_photo_sticker_type_regular_or_mask;
pub use chat_photo_sticker_type_regular_or_mask::ChatPhotoStickerTypeRegularOrMask;

mod chat_photo_sticker_type_custom_emoji;
pub use chat_photo_sticker_type_custom_emoji::ChatPhotoStickerTypeCustomEmoji;

mod chat_photo_sticker;
pub use chat_photo_sticker::ChatPhotoSticker;

mod animated_chat_photo;
pub use animated_chat_photo::AnimatedChatPhoto;

mod chat_photo;
pub use chat_photo::ChatPhoto;

mod chat_photos;
pub use chat_photos::ChatPhotos;

mod input_chat_photo_previous;
pub use input_chat_photo_previous::InputChatPhotoPrevious;

mod input_chat_photo_static;
pub use input_chat_photo_static::InputChatPhotoStatic;

mod input_chat_photo_animation;
pub use input_chat_photo_animation::InputChatPhotoAnimation;

mod input_chat_photo_sticker;
pub use input_chat_photo_sticker::InputChatPhotoSticker;

mod chat_permissions;
pub use chat_permissions::ChatPermissions;

mod chat_administrator_rights;
pub use chat_administrator_rights::ChatAdministratorRights;

mod gift_resale_price_star;
pub use gift_resale_price_star::GiftResalePriceStar;

mod gift_resale_price_ton;
pub use gift_resale_price_ton::GiftResalePriceTon;

mod suggested_post_price_star;
pub use suggested_post_price_star::SuggestedPostPriceStar;

mod suggested_post_price_ton;
pub use suggested_post_price_ton::SuggestedPostPriceTon;

mod suggested_post_info;
pub use suggested_post_info::SuggestedPostInfo;

mod input_suggested_post_info;
pub use input_suggested_post_info::InputSuggestedPostInfo;

mod star_amount;
pub use star_amount::StarAmount;

mod star_subscription_type_channel;
pub use star_subscription_type_channel::StarSubscriptionTypeChannel;

mod star_subscription_type_bot;
pub use star_subscription_type_bot::StarSubscriptionTypeBot;

mod star_subscription_pricing;
pub use star_subscription_pricing::StarSubscriptionPricing;

mod star_subscription;
pub use star_subscription::StarSubscription;

mod star_subscriptions;
pub use star_subscriptions::StarSubscriptions;

mod affiliate_type_bot;
pub use affiliate_type_bot::AffiliateTypeBot;

mod affiliate_type_channel;
pub use affiliate_type_channel::AffiliateTypeChannel;

mod affiliate_program_parameters;
pub use affiliate_program_parameters::AffiliateProgramParameters;

mod affiliate_program_info;
pub use affiliate_program_info::AffiliateProgramInfo;

mod affiliate_info;
pub use affiliate_info::AffiliateInfo;

mod found_affiliate_program;
pub use found_affiliate_program::FoundAffiliateProgram;

mod found_affiliate_programs;
pub use found_affiliate_programs::FoundAffiliatePrograms;

mod connected_affiliate_program;
pub use connected_affiliate_program::ConnectedAffiliateProgram;

mod connected_affiliate_programs;
pub use connected_affiliate_programs::ConnectedAffiliatePrograms;

mod product_info;
pub use product_info::ProductInfo;

mod premium_payment_option;
pub use premium_payment_option::PremiumPaymentOption;

mod premium_state_payment_option;
pub use premium_state_payment_option::PremiumStatePaymentOption;

mod premium_gift_payment_option;
pub use premium_gift_payment_option::PremiumGiftPaymentOption;

mod premium_gift_payment_options;
pub use premium_gift_payment_options::PremiumGiftPaymentOptions;

mod premium_giveaway_payment_option;
pub use premium_giveaway_payment_option::PremiumGiveawayPaymentOption;

mod premium_giveaway_payment_options;
pub use premium_giveaway_payment_options::PremiumGiveawayPaymentOptions;

mod premium_gift_code_info;
pub use premium_gift_code_info::PremiumGiftCodeInfo;

mod star_payment_option;
pub use star_payment_option::StarPaymentOption;

mod star_payment_options;
pub use star_payment_options::StarPaymentOptions;

mod star_giveaway_winner_option;
pub use star_giveaway_winner_option::StarGiveawayWinnerOption;

mod star_giveaway_payment_option;
pub use star_giveaway_payment_option::StarGiveawayPaymentOption;

mod star_giveaway_payment_options;
pub use star_giveaway_payment_options::StarGiveawayPaymentOptions;

mod accepted_gift_types;
pub use accepted_gift_types::AcceptedGiftTypes;

mod gift_settings;
pub use gift_settings::GiftSettings;

mod gift_auction;
pub use gift_auction::GiftAuction;

mod gift_background;
pub use gift_background::GiftBackground;

mod gift_purchase_limits;
pub use gift_purchase_limits::GiftPurchaseLimits;

mod gift_resale_parameters;
pub use gift_resale_parameters::GiftResaleParameters;

mod gift_collection;
pub use gift_collection::GiftCollection;

mod gift_collections;
pub use gift_collections::GiftCollections;

mod can_send_gift_result_fail;
pub use can_send_gift_result_fail::CanSendGiftResultFail;

mod upgraded_gift_origin_upgrade;
pub use upgraded_gift_origin_upgrade::UpgradedGiftOriginUpgrade;

mod upgraded_gift_origin_resale;
pub use upgraded_gift_origin_resale::UpgradedGiftOriginResale;

mod upgraded_gift_origin_offer;
pub use upgraded_gift_origin_offer::UpgradedGiftOriginOffer;

mod upgraded_gift_attribute_rarity_per_mille;
pub use upgraded_gift_attribute_rarity_per_mille::UpgradedGiftAttributeRarityPerMille;

mod upgraded_gift_model;
pub use upgraded_gift_model::UpgradedGiftModel;

mod upgraded_gift_symbol;
pub use upgraded_gift_symbol::UpgradedGiftSymbol;

mod upgraded_gift_backdrop_colors;
pub use upgraded_gift_backdrop_colors::UpgradedGiftBackdropColors;

mod upgraded_gift_backdrop;
pub use upgraded_gift_backdrop::UpgradedGiftBackdrop;

mod upgraded_gift_original_details;
pub use upgraded_gift_original_details::UpgradedGiftOriginalDetails;

mod upgraded_gift_colors;
pub use upgraded_gift_colors::UpgradedGiftColors;

mod gift;
pub use gift::Gift;

mod upgraded_gift;
pub use upgraded_gift::UpgradedGift;

mod upgraded_gift_value_info;
pub use upgraded_gift_value_info::UpgradedGiftValueInfo;

mod upgrade_gift_result;
pub use upgrade_gift_result::UpgradeGiftResult;

mod craft_gift_result_success;
pub use craft_gift_result_success::CraftGiftResultSuccess;

mod craft_gift_result_too_early;
pub use craft_gift_result_too_early::CraftGiftResultTooEarly;

mod available_gift;
pub use available_gift::AvailableGift;

mod available_gifts;
pub use available_gifts::AvailableGifts;

mod gift_upgrade_price;
pub use gift_upgrade_price::GiftUpgradePrice;

mod upgraded_gift_attribute_id_model;
pub use upgraded_gift_attribute_id_model::UpgradedGiftAttributeIdModel;

mod upgraded_gift_attribute_id_symbol;
pub use upgraded_gift_attribute_id_symbol::UpgradedGiftAttributeIdSymbol;

mod upgraded_gift_attribute_id_backdrop;
pub use upgraded_gift_attribute_id_backdrop::UpgradedGiftAttributeIdBackdrop;

mod upgraded_gift_model_count;
pub use upgraded_gift_model_count::UpgradedGiftModelCount;

mod upgraded_gift_symbol_count;
pub use upgraded_gift_symbol_count::UpgradedGiftSymbolCount;

mod upgraded_gift_backdrop_count;
pub use upgraded_gift_backdrop_count::UpgradedGiftBackdropCount;

mod gift_for_resale;
pub use gift_for_resale::GiftForResale;

mod gifts_for_resale;
pub use gifts_for_resale::GiftsForResale;

mod gift_resale_result_ok;
pub use gift_resale_result_ok::GiftResaleResultOk;

mod gift_resale_result_price_increased;
pub use gift_resale_result_price_increased::GiftResaleResultPriceIncreased;

mod sent_gift_regular;
pub use sent_gift_regular::SentGiftRegular;

mod sent_gift_upgraded;
pub use sent_gift_upgraded::SentGiftUpgraded;

mod received_gift;
pub use received_gift::ReceivedGift;

mod received_gifts;
pub use received_gifts::ReceivedGifts;

mod attribute_craft_persistence_probability;
pub use attribute_craft_persistence_probability::AttributeCraftPersistenceProbability;

mod gifts_for_crafting;
pub use gifts_for_crafting::GiftsForCrafting;

mod gift_upgrade_preview;
pub use gift_upgrade_preview::GiftUpgradePreview;

mod gift_upgrade_variants;
pub use gift_upgrade_variants::GiftUpgradeVariants;

mod auction_bid;
pub use auction_bid::AuctionBid;

mod user_auction_bid;
pub use user_auction_bid::UserAuctionBid;

mod auction_round;
pub use auction_round::AuctionRound;

mod auction_state_active;
pub use auction_state_active::AuctionStateActive;

mod auction_state_finished;
pub use auction_state_finished::AuctionStateFinished;

mod gift_auction_state;
pub use gift_auction_state::GiftAuctionState;

mod gift_auction_acquired_gift;
pub use gift_auction_acquired_gift::GiftAuctionAcquiredGift;

mod gift_auction_acquired_gifts;
pub use gift_auction_acquired_gifts::GiftAuctionAcquiredGifts;

mod star_transaction_type_user_deposit;
pub use star_transaction_type_user_deposit::StarTransactionTypeUserDeposit;

mod star_transaction_type_giveaway_deposit;
pub use star_transaction_type_giveaway_deposit::StarTransactionTypeGiveawayDeposit;

mod star_transaction_type_fragment_withdrawal;
pub use star_transaction_type_fragment_withdrawal::StarTransactionTypeFragmentWithdrawal;

mod star_transaction_type_telegram_api_usage;
pub use star_transaction_type_telegram_api_usage::StarTransactionTypeTelegramApiUsage;

mod star_transaction_type_bot_paid_media_purchase;
pub use star_transaction_type_bot_paid_media_purchase::StarTransactionTypeBotPaidMediaPurchase;

mod star_transaction_type_bot_paid_media_sale;
pub use star_transaction_type_bot_paid_media_sale::StarTransactionTypeBotPaidMediaSale;

mod star_transaction_type_channel_paid_media_purchase;
pub use star_transaction_type_channel_paid_media_purchase::StarTransactionTypeChannelPaidMediaPurchase;

mod star_transaction_type_channel_paid_media_sale;
pub use star_transaction_type_channel_paid_media_sale::StarTransactionTypeChannelPaidMediaSale;

mod star_transaction_type_bot_invoice_purchase;
pub use star_transaction_type_bot_invoice_purchase::StarTransactionTypeBotInvoicePurchase;

mod star_transaction_type_bot_invoice_sale;
pub use star_transaction_type_bot_invoice_sale::StarTransactionTypeBotInvoiceSale;

mod star_transaction_type_bot_subscription_purchase;
pub use star_transaction_type_bot_subscription_purchase::StarTransactionTypeBotSubscriptionPurchase;

mod star_transaction_type_bot_subscription_sale;
pub use star_transaction_type_bot_subscription_sale::StarTransactionTypeBotSubscriptionSale;

mod star_transaction_type_channel_subscription_purchase;
pub use star_transaction_type_channel_subscription_purchase::StarTransactionTypeChannelSubscriptionPurchase;

mod star_transaction_type_channel_subscription_sale;
pub use star_transaction_type_channel_subscription_sale::StarTransactionTypeChannelSubscriptionSale;

mod star_transaction_type_gift_auction_bid;
pub use star_transaction_type_gift_auction_bid::StarTransactionTypeGiftAuctionBid;

mod star_transaction_type_gift_purchase;
pub use star_transaction_type_gift_purchase::StarTransactionTypeGiftPurchase;

mod star_transaction_type_gift_purchase_offer;
pub use star_transaction_type_gift_purchase_offer::StarTransactionTypeGiftPurchaseOffer;

mod star_transaction_type_gift_transfer;
pub use star_transaction_type_gift_transfer::StarTransactionTypeGiftTransfer;

mod star_transaction_type_gift_original_details_drop;
pub use star_transaction_type_gift_original_details_drop::StarTransactionTypeGiftOriginalDetailsDrop;

mod star_transaction_type_gift_sale;
pub use star_transaction_type_gift_sale::StarTransactionTypeGiftSale;

mod star_transaction_type_gift_upgrade;
pub use star_transaction_type_gift_upgrade::StarTransactionTypeGiftUpgrade;

mod star_transaction_type_gift_upgrade_purchase;
pub use star_transaction_type_gift_upgrade_purchase::StarTransactionTypeGiftUpgradePurchase;

mod star_transaction_type_upgraded_gift_purchase;
pub use star_transaction_type_upgraded_gift_purchase::StarTransactionTypeUpgradedGiftPurchase;

mod star_transaction_type_upgraded_gift_sale;
pub use star_transaction_type_upgraded_gift_sale::StarTransactionTypeUpgradedGiftSale;

mod star_transaction_type_channel_paid_reaction_send;
pub use star_transaction_type_channel_paid_reaction_send::StarTransactionTypeChannelPaidReactionSend;

mod star_transaction_type_channel_paid_reaction_receive;
pub use star_transaction_type_channel_paid_reaction_receive::StarTransactionTypeChannelPaidReactionReceive;

mod star_transaction_type_affiliate_program_commission;
pub use star_transaction_type_affiliate_program_commission::StarTransactionTypeAffiliateProgramCommission;

mod star_transaction_type_paid_message_send;
pub use star_transaction_type_paid_message_send::StarTransactionTypePaidMessageSend;

mod star_transaction_type_paid_message_receive;
pub use star_transaction_type_paid_message_receive::StarTransactionTypePaidMessageReceive;

mod star_transaction_type_paid_group_call_message_send;
pub use star_transaction_type_paid_group_call_message_send::StarTransactionTypePaidGroupCallMessageSend;

mod star_transaction_type_paid_group_call_message_receive;
pub use star_transaction_type_paid_group_call_message_receive::StarTransactionTypePaidGroupCallMessageReceive;

mod star_transaction_type_paid_group_call_reaction_send;
pub use star_transaction_type_paid_group_call_reaction_send::StarTransactionTypePaidGroupCallReactionSend;

mod star_transaction_type_paid_group_call_reaction_receive;
pub use star_transaction_type_paid_group_call_reaction_receive::StarTransactionTypePaidGroupCallReactionReceive;

mod star_transaction_type_suggested_post_payment_send;
pub use star_transaction_type_suggested_post_payment_send::StarTransactionTypeSuggestedPostPaymentSend;

mod star_transaction_type_suggested_post_payment_receive;
pub use star_transaction_type_suggested_post_payment_receive::StarTransactionTypeSuggestedPostPaymentReceive;

mod star_transaction_type_premium_purchase;
pub use star_transaction_type_premium_purchase::StarTransactionTypePremiumPurchase;

mod star_transaction_type_business_bot_transfer_send;
pub use star_transaction_type_business_bot_transfer_send::StarTransactionTypeBusinessBotTransferSend;

mod star_transaction_type_business_bot_transfer_receive;
pub use star_transaction_type_business_bot_transfer_receive::StarTransactionTypeBusinessBotTransferReceive;

mod star_transaction;
pub use star_transaction::StarTransaction;

mod star_transactions;
pub use star_transactions::StarTransactions;

mod ton_transaction_type_fragment_deposit;
pub use ton_transaction_type_fragment_deposit::TonTransactionTypeFragmentDeposit;

mod ton_transaction_type_fragment_withdrawal;
pub use ton_transaction_type_fragment_withdrawal::TonTransactionTypeFragmentWithdrawal;

mod ton_transaction_type_suggested_post_payment;
pub use ton_transaction_type_suggested_post_payment::TonTransactionTypeSuggestedPostPayment;

mod ton_transaction_type_gift_purchase_offer;
pub use ton_transaction_type_gift_purchase_offer::TonTransactionTypeGiftPurchaseOffer;

mod ton_transaction_type_upgraded_gift_purchase;
pub use ton_transaction_type_upgraded_gift_purchase::TonTransactionTypeUpgradedGiftPurchase;

mod ton_transaction_type_upgraded_gift_sale;
pub use ton_transaction_type_upgraded_gift_sale::TonTransactionTypeUpgradedGiftSale;

mod ton_transaction;
pub use ton_transaction::TonTransaction;

mod ton_transactions;
pub use ton_transactions::TonTransactions;

mod active_story_state_live;
pub use active_story_state_live::ActiveStoryStateLive;

mod giveaway_participant_status_already_was_member;
pub use giveaway_participant_status_already_was_member::GiveawayParticipantStatusAlreadyWasMember;

mod giveaway_participant_status_administrator;
pub use giveaway_participant_status_administrator::GiveawayParticipantStatusAdministrator;

mod giveaway_participant_status_disallowed_country;
pub use giveaway_participant_status_disallowed_country::GiveawayParticipantStatusDisallowedCountry;

mod giveaway_info_ongoing;
pub use giveaway_info_ongoing::GiveawayInfoOngoing;

mod giveaway_info_completed;
pub use giveaway_info_completed::GiveawayInfoCompleted;

mod giveaway_prize_premium;
pub use giveaway_prize_premium::GiveawayPrizePremium;

mod giveaway_prize_stars;
pub use giveaway_prize_stars::GiveawayPrizeStars;

mod accent_color;
pub use accent_color::AccentColor;

mod profile_accent_colors;
pub use profile_accent_colors::ProfileAccentColors;

mod profile_accent_color;
pub use profile_accent_color::ProfileAccentColor;

mod user_rating;
pub use user_rating::UserRating;

mod restriction_info;
pub use restriction_info::RestrictionInfo;

mod emoji_status_type_custom_emoji;
pub use emoji_status_type_custom_emoji::EmojiStatusTypeCustomEmoji;

mod emoji_status_type_upgraded_gift;
pub use emoji_status_type_upgraded_gift::EmojiStatusTypeUpgradedGift;

mod emoji_status;
pub use emoji_status::EmojiStatus;

mod emoji_statuses;
pub use emoji_statuses::EmojiStatuses;

mod emoji_status_custom_emojis;
pub use emoji_status_custom_emojis::EmojiStatusCustomEmojis;

mod usernames;
pub use usernames::Usernames;

mod user;
pub use user::User;

mod bot_info;
pub use bot_info::BotInfo;

mod user_full_info;
pub use user_full_info::UserFullInfo;

mod users;
pub use users::Users;

mod found_users;
pub use found_users::FoundUsers;

mod chat_administrator;
pub use chat_administrator::ChatAdministrator;

mod chat_administrators;
pub use chat_administrators::ChatAdministrators;

mod chat_member_status_creator;
pub use chat_member_status_creator::ChatMemberStatusCreator;

mod chat_member_status_administrator;
pub use chat_member_status_administrator::ChatMemberStatusAdministrator;

mod chat_member_status_member;
pub use chat_member_status_member::ChatMemberStatusMember;

mod chat_member_status_restricted;
pub use chat_member_status_restricted::ChatMemberStatusRestricted;

mod chat_member_status_banned;
pub use chat_member_status_banned::ChatMemberStatusBanned;

mod chat_member;
pub use chat_member::ChatMember;

mod chat_members;
pub use chat_members::ChatMembers;

mod chat_members_filter_mention;
pub use chat_members_filter_mention::ChatMembersFilterMention;

mod supergroup_members_filter_contacts;
pub use supergroup_members_filter_contacts::SupergroupMembersFilterContacts;

mod supergroup_members_filter_search;
pub use supergroup_members_filter_search::SupergroupMembersFilterSearch;

mod supergroup_members_filter_restricted;
pub use supergroup_members_filter_restricted::SupergroupMembersFilterRestricted;

mod supergroup_members_filter_banned;
pub use supergroup_members_filter_banned::SupergroupMembersFilterBanned;

mod supergroup_members_filter_mention;
pub use supergroup_members_filter_mention::SupergroupMembersFilterMention;

mod chat_invite_link;
pub use chat_invite_link::ChatInviteLink;

mod chat_invite_links;
pub use chat_invite_links::ChatInviteLinks;

mod chat_invite_link_count;
pub use chat_invite_link_count::ChatInviteLinkCount;

mod chat_invite_link_counts;
pub use chat_invite_link_counts::ChatInviteLinkCounts;

mod chat_invite_link_member;
pub use chat_invite_link_member::ChatInviteLinkMember;

mod chat_invite_link_members;
pub use chat_invite_link_members::ChatInviteLinkMembers;

mod chat_invite_link_subscription_info;
pub use chat_invite_link_subscription_info::ChatInviteLinkSubscriptionInfo;

mod chat_invite_link_info;
pub use chat_invite_link_info::ChatInviteLinkInfo;

mod chat_join_request;
pub use chat_join_request::ChatJoinRequest;

mod chat_join_requests;
pub use chat_join_requests::ChatJoinRequests;

mod chat_join_requests_info;
pub use chat_join_requests_info::ChatJoinRequestsInfo;

mod basic_group;
pub use basic_group::BasicGroup;

mod basic_group_full_info;
pub use basic_group_full_info::BasicGroupFullInfo;

mod supergroup;
pub use supergroup::Supergroup;

mod supergroup_full_info;
pub use supergroup_full_info::SupergroupFullInfo;

mod secret_chat;
pub use secret_chat::SecretChat;

mod public_post_search_limits;
pub use public_post_search_limits::PublicPostSearchLimits;

mod message_sender_user;
pub use message_sender_user::MessageSenderUser;

mod message_sender_chat;
pub use message_sender_chat::MessageSenderChat;

mod message_senders;
pub use message_senders::MessageSenders;

mod chat_message_sender;
pub use chat_message_sender::ChatMessageSender;

mod chat_message_senders;
pub use chat_message_senders::ChatMessageSenders;

mod poll_voter;
pub use poll_voter::PollVoter;

mod poll_voters;
pub use poll_voters::PollVoters;

mod message_read_date_read;
pub use message_read_date_read::MessageReadDateRead;

mod message_viewer;
pub use message_viewer::MessageViewer;

mod message_viewers;
pub use message_viewers::MessageViewers;

mod message_origin_user;
pub use message_origin_user::MessageOriginUser;

mod message_origin_hidden_user;
pub use message_origin_hidden_user::MessageOriginHiddenUser;

mod message_origin_chat;
pub use message_origin_chat::MessageOriginChat;

mod message_origin_channel;
pub use message_origin_channel::MessageOriginChannel;

mod forward_source;
pub use forward_source::ForwardSource;

mod reaction_type_emoji;
pub use reaction_type_emoji::ReactionTypeEmoji;

mod reaction_type_custom_emoji;
pub use reaction_type_custom_emoji::ReactionTypeCustomEmoji;

mod paid_reaction_type_chat;
pub use paid_reaction_type_chat::PaidReactionTypeChat;

mod paid_reactor;
pub use paid_reactor::PaidReactor;

mod live_story_donors;
pub use live_story_donors::LiveStoryDonors;

mod message_forward_info;
pub use message_forward_info::MessageForwardInfo;

mod message_import_info;
pub use message_import_info::MessageImportInfo;

mod message_reply_info;
pub use message_reply_info::MessageReplyInfo;

mod message_reaction;
pub use message_reaction::MessageReaction;

mod message_reactions;
pub use message_reactions::MessageReactions;

mod message_interaction_info;
pub use message_interaction_info::MessageInteractionInfo;

mod unread_reaction;
pub use unread_reaction::UnreadReaction;

mod message_topic_thread;
pub use message_topic_thread::MessageTopicThread;

mod message_topic_forum;
pub use message_topic_forum::MessageTopicForum;

mod message_topic_direct_messages;
pub use message_topic_direct_messages::MessageTopicDirectMessages;

mod message_topic_saved_messages;
pub use message_topic_saved_messages::MessageTopicSavedMessages;

mod message_effect_type_emoji_reaction;
pub use message_effect_type_emoji_reaction::MessageEffectTypeEmojiReaction;

mod message_effect_type_premium_sticker;
pub use message_effect_type_premium_sticker::MessageEffectTypePremiumSticker;

mod message_effect;
pub use message_effect::MessageEffect;

mod message_sending_state_pending;
pub use message_sending_state_pending::MessageSendingStatePending;

mod message_sending_state_failed;
pub use message_sending_state_failed::MessageSendingStateFailed;

mod text_quote;
pub use text_quote::TextQuote;

mod input_text_quote;
pub use input_text_quote::InputTextQuote;

mod message_reply_to_message;
pub use message_reply_to_message::MessageReplyToMessage;

mod message_reply_to_story;
pub use message_reply_to_story::MessageReplyToStory;

mod input_message_reply_to_message;
pub use input_message_reply_to_message::InputMessageReplyToMessage;

mod input_message_reply_to_external_message;
pub use input_message_reply_to_external_message::InputMessageReplyToExternalMessage;

mod input_message_reply_to_story;
pub use input_message_reply_to_story::InputMessageReplyToStory;

mod fact_check;
pub use fact_check::FactCheck;

mod message;
pub use message::Message;

mod messages;
pub use messages::Messages;

mod found_messages;
pub use found_messages::FoundMessages;

mod found_chat_messages;
pub use found_chat_messages::FoundChatMessages;

mod found_public_posts;
pub use found_public_posts::FoundPublicPosts;

mod message_position;
pub use message_position::MessagePosition;

mod message_positions;
pub use message_positions::MessagePositions;

mod message_calendar_day;
pub use message_calendar_day::MessageCalendarDay;

mod message_calendar;
pub use message_calendar::MessageCalendar;

mod business_message;
pub use business_message::BusinessMessage;

mod business_messages;
pub use business_messages::BusinessMessages;

mod advertisement_sponsor;
pub use advertisement_sponsor::AdvertisementSponsor;

mod sponsored_message;
pub use sponsored_message::SponsoredMessage;

mod sponsored_messages;
pub use sponsored_messages::SponsoredMessages;

mod sponsored_chat;
pub use sponsored_chat::SponsoredChat;

mod sponsored_chats;
pub use sponsored_chats::SponsoredChats;

mod video_message_advertisement;
pub use video_message_advertisement::VideoMessageAdvertisement;

mod video_message_advertisements;
pub use video_message_advertisements::VideoMessageAdvertisements;

mod report_option;
pub use report_option::ReportOption;

mod report_sponsored_result_option_required;
pub use report_sponsored_result_option_required::ReportSponsoredResultOptionRequired;

mod file_download;
pub use file_download::FileDownload;

mod downloaded_file_counts;
pub use downloaded_file_counts::DownloadedFileCounts;

mod found_file_downloads;
pub use found_file_downloads::FoundFileDownloads;

mod chat_notification_settings;
pub use chat_notification_settings::ChatNotificationSettings;

mod scope_notification_settings;
pub use scope_notification_settings::ScopeNotificationSettings;

mod reaction_notification_settings;
pub use reaction_notification_settings::ReactionNotificationSettings;

mod draft_message;
pub use draft_message::DraftMessage;

mod chat_type_private;
pub use chat_type_private::ChatTypePrivate;

mod chat_type_basic_group;
pub use chat_type_basic_group::ChatTypeBasicGroup;

mod chat_type_supergroup;
pub use chat_type_supergroup::ChatTypeSupergroup;

mod chat_type_secret;
pub use chat_type_secret::ChatTypeSecret;

mod chat_folder_icon;
pub use chat_folder_icon::ChatFolderIcon;

mod chat_folder_name;
pub use chat_folder_name::ChatFolderName;

mod chat_folder;
pub use chat_folder::ChatFolder;

mod chat_folder_info;
pub use chat_folder_info::ChatFolderInfo;

mod chat_folder_invite_link;
pub use chat_folder_invite_link::ChatFolderInviteLink;

mod chat_folder_invite_links;
pub use chat_folder_invite_links::ChatFolderInviteLinks;

mod chat_folder_invite_link_info;
pub use chat_folder_invite_link_info::ChatFolderInviteLinkInfo;

mod recommended_chat_folder;
pub use recommended_chat_folder::RecommendedChatFolder;

mod recommended_chat_folders;
pub use recommended_chat_folders::RecommendedChatFolders;

mod archive_chat_list_settings;
pub use archive_chat_list_settings::ArchiveChatListSettings;

mod chat_list_folder;
pub use chat_list_folder::ChatListFolder;

mod chat_lists;
pub use chat_lists::ChatLists;

mod chat_source_public_service_announcement;
pub use chat_source_public_service_announcement::ChatSourcePublicServiceAnnouncement;

mod chat_position;
pub use chat_position::ChatPosition;

mod chat_available_reactions_all;
pub use chat_available_reactions_all::ChatAvailableReactionsAll;

mod chat_available_reactions_some;
pub use chat_available_reactions_some::ChatAvailableReactionsSome;

mod saved_messages_tag;
pub use saved_messages_tag::SavedMessagesTag;

mod saved_messages_tags;
pub use saved_messages_tags::SavedMessagesTags;

mod business_bot_manage_bar;
pub use business_bot_manage_bar::BusinessBotManageBar;

mod video_chat;
pub use video_chat::VideoChat;

mod chat;
pub use chat::Chat;

mod chats;
pub use chats::Chats;

mod failed_to_add_member;
pub use failed_to_add_member::FailedToAddMember;

mod failed_to_add_members;
pub use failed_to_add_members::FailedToAddMembers;

mod created_basic_group_chat;
pub use created_basic_group_chat::CreatedBasicGroupChat;

mod account_info;
pub use account_info::AccountInfo;

mod chat_action_bar_report_spam;
pub use chat_action_bar_report_spam::ChatActionBarReportSpam;

mod chat_action_bar_report_add_block;
pub use chat_action_bar_report_add_block::ChatActionBarReportAddBlock;

mod chat_action_bar_join_request;
pub use chat_action_bar_join_request::ChatActionBarJoinRequest;

mod keyboard_button_type_request_poll;
pub use keyboard_button_type_request_poll::KeyboardButtonTypeRequestPoll;

mod keyboard_button_type_request_users;
pub use keyboard_button_type_request_users::KeyboardButtonTypeRequestUsers;

mod keyboard_button_type_request_chat;
pub use keyboard_button_type_request_chat::KeyboardButtonTypeRequestChat;

mod keyboard_button_type_web_app;
pub use keyboard_button_type_web_app::KeyboardButtonTypeWebApp;

mod keyboard_button;
pub use keyboard_button::KeyboardButton;

mod inline_keyboard_button_type_url;
pub use inline_keyboard_button_type_url::InlineKeyboardButtonTypeUrl;

mod inline_keyboard_button_type_login_url;
pub use inline_keyboard_button_type_login_url::InlineKeyboardButtonTypeLoginUrl;

mod inline_keyboard_button_type_web_app;
pub use inline_keyboard_button_type_web_app::InlineKeyboardButtonTypeWebApp;

mod inline_keyboard_button_type_callback;
pub use inline_keyboard_button_type_callback::InlineKeyboardButtonTypeCallback;

mod inline_keyboard_button_type_callback_with_password;
pub use inline_keyboard_button_type_callback_with_password::InlineKeyboardButtonTypeCallbackWithPassword;

mod inline_keyboard_button_type_switch_inline;
pub use inline_keyboard_button_type_switch_inline::InlineKeyboardButtonTypeSwitchInline;

mod inline_keyboard_button_type_user;
pub use inline_keyboard_button_type_user::InlineKeyboardButtonTypeUser;

mod inline_keyboard_button_type_copy_text;
pub use inline_keyboard_button_type_copy_text::InlineKeyboardButtonTypeCopyText;

mod inline_keyboard_button;
pub use inline_keyboard_button::InlineKeyboardButton;

mod reply_markup_remove_keyboard;
pub use reply_markup_remove_keyboard::ReplyMarkupRemoveKeyboard;

mod reply_markup_force_reply;
pub use reply_markup_force_reply::ReplyMarkupForceReply;

mod reply_markup_show_keyboard;
pub use reply_markup_show_keyboard::ReplyMarkupShowKeyboard;

mod reply_markup_inline_keyboard;
pub use reply_markup_inline_keyboard::ReplyMarkupInlineKeyboard;

mod login_url_info_open;
pub use login_url_info_open::LoginUrlInfoOpen;

mod login_url_info_request_confirmation;
pub use login_url_info_request_confirmation::LoginUrlInfoRequestConfirmation;

mod oauth_link_info;
pub use oauth_link_info::OauthLinkInfo;

mod theme_parameters;
pub use theme_parameters::ThemeParameters;

mod found_web_app;
pub use found_web_app::FoundWebApp;

mod web_app_info;
pub use web_app_info::WebAppInfo;

mod main_web_app;
pub use main_web_app::MainWebApp;

mod web_app_open_parameters;
pub use web_app_open_parameters::WebAppOpenParameters;

mod message_thread_info;
pub use message_thread_info::MessageThreadInfo;

mod saved_messages_topic_type_saved_from_chat;
pub use saved_messages_topic_type_saved_from_chat::SavedMessagesTopicTypeSavedFromChat;

mod saved_messages_topic;
pub use saved_messages_topic::SavedMessagesTopic;

mod direct_messages_chat_topic;
pub use direct_messages_chat_topic::DirectMessagesChatTopic;

mod forum_topic_icon;
pub use forum_topic_icon::ForumTopicIcon;

mod forum_topic_info;
pub use forum_topic_info::ForumTopicInfo;

mod forum_topic;
pub use forum_topic::ForumTopic;

mod forum_topics;
pub use forum_topics::ForumTopics;

mod link_preview_options;
pub use link_preview_options::LinkPreviewOptions;

mod shared_user;
pub use shared_user::SharedUser;

mod shared_chat;
pub use shared_chat::SharedChat;

mod theme_settings;
pub use theme_settings::ThemeSettings;

mod rich_text_plain;
pub use rich_text_plain::RichTextPlain;

mod rich_text_bold;
pub use rich_text_bold::RichTextBold;

mod rich_text_italic;
pub use rich_text_italic::RichTextItalic;

mod rich_text_underline;
pub use rich_text_underline::RichTextUnderline;

mod rich_text_strikethrough;
pub use rich_text_strikethrough::RichTextStrikethrough;

mod rich_text_fixed;
pub use rich_text_fixed::RichTextFixed;

mod rich_text_url;
pub use rich_text_url::RichTextUrl;

mod rich_text_email_address;
pub use rich_text_email_address::RichTextEmailAddress;

mod rich_text_subscript;
pub use rich_text_subscript::RichTextSubscript;

mod rich_text_superscript;
pub use rich_text_superscript::RichTextSuperscript;

mod rich_text_marked;
pub use rich_text_marked::RichTextMarked;

mod rich_text_phone_number;
pub use rich_text_phone_number::RichTextPhoneNumber;

mod rich_text_icon;
pub use rich_text_icon::RichTextIcon;

mod rich_text_reference;
pub use rich_text_reference::RichTextReference;

mod rich_text_anchor;
pub use rich_text_anchor::RichTextAnchor;

mod rich_text_anchor_link;
pub use rich_text_anchor_link::RichTextAnchorLink;

mod rich_texts;
pub use rich_texts::RichTexts;

mod page_block_caption;
pub use page_block_caption::PageBlockCaption;

mod page_block_list_item;
pub use page_block_list_item::PageBlockListItem;

mod page_block_table_cell;
pub use page_block_table_cell::PageBlockTableCell;

mod page_block_related_article;
pub use page_block_related_article::PageBlockRelatedArticle;

mod page_block_title;
pub use page_block_title::PageBlockTitle;

mod page_block_subtitle;
pub use page_block_subtitle::PageBlockSubtitle;

mod page_block_author_date;
pub use page_block_author_date::PageBlockAuthorDate;

mod page_block_header;
pub use page_block_header::PageBlockHeader;

mod page_block_subheader;
pub use page_block_subheader::PageBlockSubheader;

mod page_block_kicker;
pub use page_block_kicker::PageBlockKicker;

mod page_block_paragraph;
pub use page_block_paragraph::PageBlockParagraph;

mod page_block_preformatted;
pub use page_block_preformatted::PageBlockPreformatted;

mod page_block_footer;
pub use page_block_footer::PageBlockFooter;

mod page_block_anchor;
pub use page_block_anchor::PageBlockAnchor;

mod page_block_list;
pub use page_block_list::PageBlockList;

mod page_block_block_quote;
pub use page_block_block_quote::PageBlockBlockQuote;

mod page_block_pull_quote;
pub use page_block_pull_quote::PageBlockPullQuote;

mod page_block_animation;
pub use page_block_animation::PageBlockAnimation;

mod page_block_audio;
pub use page_block_audio::PageBlockAudio;

mod page_block_photo;
pub use page_block_photo::PageBlockPhoto;

mod page_block_video;
pub use page_block_video::PageBlockVideo;

mod page_block_voice_note;
pub use page_block_voice_note::PageBlockVoiceNote;

mod page_block_cover;
pub use page_block_cover::PageBlockCover;

mod page_block_embedded;
pub use page_block_embedded::PageBlockEmbedded;

mod page_block_embedded_post;
pub use page_block_embedded_post::PageBlockEmbeddedPost;

mod page_block_collage;
pub use page_block_collage::PageBlockCollage;

mod page_block_slideshow;
pub use page_block_slideshow::PageBlockSlideshow;

mod page_block_chat_link;
pub use page_block_chat_link::PageBlockChatLink;

mod page_block_table;
pub use page_block_table::PageBlockTable;

mod page_block_details;
pub use page_block_details::PageBlockDetails;

mod page_block_related_articles;
pub use page_block_related_articles::PageBlockRelatedArticles;

mod page_block_map;
pub use page_block_map::PageBlockMap;

mod web_page_instant_view;
pub use web_page_instant_view::WebPageInstantView;

mod link_preview_album_media_photo;
pub use link_preview_album_media_photo::LinkPreviewAlbumMediaPhoto;

mod link_preview_album_media_video;
pub use link_preview_album_media_video::LinkPreviewAlbumMediaVideo;

mod link_preview_type_album;
pub use link_preview_type_album::LinkPreviewTypeAlbum;

mod link_preview_type_animation;
pub use link_preview_type_animation::LinkPreviewTypeAnimation;

mod link_preview_type_app;
pub use link_preview_type_app::LinkPreviewTypeApp;

mod link_preview_type_article;
pub use link_preview_type_article::LinkPreviewTypeArticle;

mod link_preview_type_audio;
pub use link_preview_type_audio::LinkPreviewTypeAudio;

mod link_preview_type_background;
pub use link_preview_type_background::LinkPreviewTypeBackground;

mod link_preview_type_channel_boost;
pub use link_preview_type_channel_boost::LinkPreviewTypeChannelBoost;

mod link_preview_type_chat;
pub use link_preview_type_chat::LinkPreviewTypeChat;

mod link_preview_type_direct_messages_chat;
pub use link_preview_type_direct_messages_chat::LinkPreviewTypeDirectMessagesChat;

mod link_preview_type_document;
pub use link_preview_type_document::LinkPreviewTypeDocument;

mod link_preview_type_embedded_animation_player;
pub use link_preview_type_embedded_animation_player::LinkPreviewTypeEmbeddedAnimationPlayer;

mod link_preview_type_embedded_audio_player;
pub use link_preview_type_embedded_audio_player::LinkPreviewTypeEmbeddedAudioPlayer;

mod link_preview_type_embedded_video_player;
pub use link_preview_type_embedded_video_player::LinkPreviewTypeEmbeddedVideoPlayer;

mod link_preview_type_external_audio;
pub use link_preview_type_external_audio::LinkPreviewTypeExternalAudio;

mod link_preview_type_external_video;
pub use link_preview_type_external_video::LinkPreviewTypeExternalVideo;

mod link_preview_type_gift_auction;
pub use link_preview_type_gift_auction::LinkPreviewTypeGiftAuction;

mod link_preview_type_gift_collection;
pub use link_preview_type_gift_collection::LinkPreviewTypeGiftCollection;

mod link_preview_type_live_story;
pub use link_preview_type_live_story::LinkPreviewTypeLiveStory;

mod link_preview_type_photo;
pub use link_preview_type_photo::LinkPreviewTypePhoto;

mod link_preview_type_sticker;
pub use link_preview_type_sticker::LinkPreviewTypeSticker;

mod link_preview_type_sticker_set;
pub use link_preview_type_sticker_set::LinkPreviewTypeStickerSet;

mod link_preview_type_story;
pub use link_preview_type_story::LinkPreviewTypeStory;

mod link_preview_type_story_album;
pub use link_preview_type_story_album::LinkPreviewTypeStoryAlbum;

mod link_preview_type_supergroup_boost;
pub use link_preview_type_supergroup_boost::LinkPreviewTypeSupergroupBoost;

mod link_preview_type_theme;
pub use link_preview_type_theme::LinkPreviewTypeTheme;

mod link_preview_type_upgraded_gift;
pub use link_preview_type_upgraded_gift::LinkPreviewTypeUpgradedGift;

mod link_preview_type_user;
pub use link_preview_type_user::LinkPreviewTypeUser;

mod link_preview_type_video;
pub use link_preview_type_video::LinkPreviewTypeVideo;

mod link_preview_type_video_chat;
pub use link_preview_type_video_chat::LinkPreviewTypeVideoChat;

mod link_preview_type_video_note;
pub use link_preview_type_video_note::LinkPreviewTypeVideoNote;

mod link_preview_type_voice_note;
pub use link_preview_type_voice_note::LinkPreviewTypeVoiceNote;

mod link_preview_type_web_app;
pub use link_preview_type_web_app::LinkPreviewTypeWebApp;

mod link_preview;
pub use link_preview::LinkPreview;

mod country_info;
pub use country_info::CountryInfo;

mod countries;
pub use countries::Countries;

mod phone_number_info;
pub use phone_number_info::PhoneNumberInfo;

mod collectible_item_type_username;
pub use collectible_item_type_username::CollectibleItemTypeUsername;

mod collectible_item_type_phone_number;
pub use collectible_item_type_phone_number::CollectibleItemTypePhoneNumber;

mod collectible_item_info;
pub use collectible_item_info::CollectibleItemInfo;

mod bank_card_action_open_url;
pub use bank_card_action_open_url::BankCardActionOpenUrl;

mod bank_card_info;
pub use bank_card_info::BankCardInfo;

mod address;
pub use address::Address;

mod location_address;
pub use location_address::LocationAddress;

mod labeled_price_part;
pub use labeled_price_part::LabeledPricePart;

mod invoice;
pub use invoice::Invoice;

mod order_info;
pub use order_info::OrderInfo;

mod shipping_option;
pub use shipping_option::ShippingOption;

mod saved_credentials;
pub use saved_credentials::SavedCredentials;

mod input_credentials_saved;
pub use input_credentials_saved::InputCredentialsSaved;

mod input_credentials_new;
pub use input_credentials_new::InputCredentialsNew;

mod input_credentials_apple_pay;
pub use input_credentials_apple_pay::InputCredentialsApplePay;

mod input_credentials_google_pay;
pub use input_credentials_google_pay::InputCredentialsGooglePay;

mod payment_provider_smart_glocal;
pub use payment_provider_smart_glocal::PaymentProviderSmartGlocal;

mod payment_provider_stripe;
pub use payment_provider_stripe::PaymentProviderStripe;

mod payment_provider_other;
pub use payment_provider_other::PaymentProviderOther;

mod payment_option;
pub use payment_option::PaymentOption;

mod payment_form_type_regular;
pub use payment_form_type_regular::PaymentFormTypeRegular;

mod payment_form_type_stars;
pub use payment_form_type_stars::PaymentFormTypeStars;

mod payment_form_type_star_subscription;
pub use payment_form_type_star_subscription::PaymentFormTypeStarSubscription;

mod payment_form;
pub use payment_form::PaymentForm;

mod validated_order_info;
pub use validated_order_info::ValidatedOrderInfo;

mod payment_result;
pub use payment_result::PaymentResult;

mod payment_receipt_type_regular;
pub use payment_receipt_type_regular::PaymentReceiptTypeRegular;

mod payment_receipt_type_stars;
pub use payment_receipt_type_stars::PaymentReceiptTypeStars;

mod payment_receipt;
pub use payment_receipt::PaymentReceipt;

mod input_invoice_message;
pub use input_invoice_message::InputInvoiceMessage;

mod input_invoice_name;
pub use input_invoice_name::InputInvoiceName;

mod input_invoice_telegram;
pub use input_invoice_telegram::InputInvoiceTelegram;

mod paid_media_preview;
pub use paid_media_preview::PaidMediaPreview;

mod paid_media_photo;
pub use paid_media_photo::PaidMediaPhoto;

mod paid_media_video;
pub use paid_media_video::PaidMediaVideo;

mod giveaway_parameters;
pub use giveaway_parameters::GiveawayParameters;

mod dated_file;
pub use dated_file::DatedFile;

mod date;
pub use date::Date;

mod personal_details;
pub use personal_details::PersonalDetails;

mod identity_document;
pub use identity_document::IdentityDocument;

mod input_identity_document;
pub use input_identity_document::InputIdentityDocument;

mod personal_document;
pub use personal_document::PersonalDocument;

mod input_personal_document;
pub use input_personal_document::InputPersonalDocument;

mod passport_element_personal_details;
pub use passport_element_personal_details::PassportElementPersonalDetails;

mod passport_element_passport;
pub use passport_element_passport::PassportElementPassport;

mod passport_element_driver_license;
pub use passport_element_driver_license::PassportElementDriverLicense;

mod passport_element_identity_card;
pub use passport_element_identity_card::PassportElementIdentityCard;

mod passport_element_internal_passport;
pub use passport_element_internal_passport::PassportElementInternalPassport;

mod passport_element_address;
pub use passport_element_address::PassportElementAddress;

mod passport_element_utility_bill;
pub use passport_element_utility_bill::PassportElementUtilityBill;

mod passport_element_bank_statement;
pub use passport_element_bank_statement::PassportElementBankStatement;

mod passport_element_rental_agreement;
pub use passport_element_rental_agreement::PassportElementRentalAgreement;

mod passport_element_passport_registration;
pub use passport_element_passport_registration::PassportElementPassportRegistration;

mod passport_element_temporary_registration;
pub use passport_element_temporary_registration::PassportElementTemporaryRegistration;

mod passport_element_phone_number;
pub use passport_element_phone_number::PassportElementPhoneNumber;

mod passport_element_email_address;
pub use passport_element_email_address::PassportElementEmailAddress;

mod input_passport_element_personal_details;
pub use input_passport_element_personal_details::InputPassportElementPersonalDetails;

mod input_passport_element_passport;
pub use input_passport_element_passport::InputPassportElementPassport;

mod input_passport_element_driver_license;
pub use input_passport_element_driver_license::InputPassportElementDriverLicense;

mod input_passport_element_identity_card;
pub use input_passport_element_identity_card::InputPassportElementIdentityCard;

mod input_passport_element_internal_passport;
pub use input_passport_element_internal_passport::InputPassportElementInternalPassport;

mod input_passport_element_address;
pub use input_passport_element_address::InputPassportElementAddress;

mod input_passport_element_utility_bill;
pub use input_passport_element_utility_bill::InputPassportElementUtilityBill;

mod input_passport_element_bank_statement;
pub use input_passport_element_bank_statement::InputPassportElementBankStatement;

mod input_passport_element_rental_agreement;
pub use input_passport_element_rental_agreement::InputPassportElementRentalAgreement;

mod input_passport_element_passport_registration;
pub use input_passport_element_passport_registration::InputPassportElementPassportRegistration;

mod input_passport_element_temporary_registration;
pub use input_passport_element_temporary_registration::InputPassportElementTemporaryRegistration;

mod input_passport_element_phone_number;
pub use input_passport_element_phone_number::InputPassportElementPhoneNumber;

mod input_passport_element_email_address;
pub use input_passport_element_email_address::InputPassportElementEmailAddress;

mod passport_elements;
pub use passport_elements::PassportElements;

mod passport_element_error_source_data_field;
pub use passport_element_error_source_data_field::PassportElementErrorSourceDataField;

mod passport_element_error_source_translation_file;
pub use passport_element_error_source_translation_file::PassportElementErrorSourceTranslationFile;

mod passport_element_error_source_file;
pub use passport_element_error_source_file::PassportElementErrorSourceFile;

mod passport_element_error;
pub use passport_element_error::PassportElementError;

mod passport_suitable_element;
pub use passport_suitable_element::PassportSuitableElement;

mod passport_required_element;
pub use passport_required_element::PassportRequiredElement;

mod passport_authorization_form;
pub use passport_authorization_form::PassportAuthorizationForm;

mod passport_elements_with_errors;
pub use passport_elements_with_errors::PassportElementsWithErrors;

mod encrypted_credentials;
pub use encrypted_credentials::EncryptedCredentials;

mod encrypted_passport_element;
pub use encrypted_passport_element::EncryptedPassportElement;

mod input_passport_element_error_source_unspecified;
pub use input_passport_element_error_source_unspecified::InputPassportElementErrorSourceUnspecified;

mod input_passport_element_error_source_data_field;
pub use input_passport_element_error_source_data_field::InputPassportElementErrorSourceDataField;

mod input_passport_element_error_source_front_side;
pub use input_passport_element_error_source_front_side::InputPassportElementErrorSourceFrontSide;

mod input_passport_element_error_source_reverse_side;
pub use input_passport_element_error_source_reverse_side::InputPassportElementErrorSourceReverseSide;

mod input_passport_element_error_source_selfie;
pub use input_passport_element_error_source_selfie::InputPassportElementErrorSourceSelfie;

mod input_passport_element_error_source_translation_file;
pub use input_passport_element_error_source_translation_file::InputPassportElementErrorSourceTranslationFile;

mod input_passport_element_error_source_translation_files;
pub use input_passport_element_error_source_translation_files::InputPassportElementErrorSourceTranslationFiles;

mod input_passport_element_error_source_file;
pub use input_passport_element_error_source_file::InputPassportElementErrorSourceFile;

mod input_passport_element_error_source_files;
pub use input_passport_element_error_source_files::InputPassportElementErrorSourceFiles;

mod input_passport_element_error;
pub use input_passport_element_error::InputPassportElementError;

mod message_text;
pub use message_text::MessageText;

mod message_animation;
pub use message_animation::MessageAnimation;

mod message_audio;
pub use message_audio::MessageAudio;

mod message_document;
pub use message_document::MessageDocument;

mod message_paid_media;
pub use message_paid_media::MessagePaidMedia;

mod message_photo;
pub use message_photo::MessagePhoto;

mod message_sticker;
pub use message_sticker::MessageSticker;

mod message_video;
pub use message_video::MessageVideo;

mod message_video_note;
pub use message_video_note::MessageVideoNote;

mod message_voice_note;
pub use message_voice_note::MessageVoiceNote;

mod message_location;
pub use message_location::MessageLocation;

mod message_venue;
pub use message_venue::MessageVenue;

mod message_contact;
pub use message_contact::MessageContact;

mod message_animated_emoji;
pub use message_animated_emoji::MessageAnimatedEmoji;

mod message_dice;
pub use message_dice::MessageDice;

mod message_game;
pub use message_game::MessageGame;

mod message_poll;
pub use message_poll::MessagePoll;

mod message_stake_dice;
pub use message_stake_dice::MessageStakeDice;

mod message_story;
pub use message_story::MessageStory;

mod message_checklist;
pub use message_checklist::MessageChecklist;

mod message_invoice;
pub use message_invoice::MessageInvoice;

mod message_call;
pub use message_call::MessageCall;

mod message_group_call;
pub use message_group_call::MessageGroupCall;

mod message_video_chat_scheduled;
pub use message_video_chat_scheduled::MessageVideoChatScheduled;

mod message_video_chat_started;
pub use message_video_chat_started::MessageVideoChatStarted;

mod message_video_chat_ended;
pub use message_video_chat_ended::MessageVideoChatEnded;

mod message_invite_video_chat_participants;
pub use message_invite_video_chat_participants::MessageInviteVideoChatParticipants;

mod message_basic_group_chat_create;
pub use message_basic_group_chat_create::MessageBasicGroupChatCreate;

mod message_supergroup_chat_create;
pub use message_supergroup_chat_create::MessageSupergroupChatCreate;

mod message_chat_change_title;
pub use message_chat_change_title::MessageChatChangeTitle;

mod message_chat_change_photo;
pub use message_chat_change_photo::MessageChatChangePhoto;

mod message_chat_owner_left;
pub use message_chat_owner_left::MessageChatOwnerLeft;

mod message_chat_owner_changed;
pub use message_chat_owner_changed::MessageChatOwnerChanged;

mod message_chat_has_protected_content_toggled;
pub use message_chat_has_protected_content_toggled::MessageChatHasProtectedContentToggled;

mod message_chat_has_protected_content_disable_requested;
pub use message_chat_has_protected_content_disable_requested::MessageChatHasProtectedContentDisableRequested;

mod message_chat_add_members;
pub use message_chat_add_members::MessageChatAddMembers;

mod message_chat_delete_member;
pub use message_chat_delete_member::MessageChatDeleteMember;

mod message_chat_upgrade_to;
pub use message_chat_upgrade_to::MessageChatUpgradeTo;

mod message_chat_upgrade_from;
pub use message_chat_upgrade_from::MessageChatUpgradeFrom;

mod message_pin_message;
pub use message_pin_message::MessagePinMessage;

mod message_chat_set_background;
pub use message_chat_set_background::MessageChatSetBackground;

mod message_chat_set_theme;
pub use message_chat_set_theme::MessageChatSetTheme;

mod message_chat_set_message_auto_delete_time;
pub use message_chat_set_message_auto_delete_time::MessageChatSetMessageAutoDeleteTime;

mod message_chat_boost;
pub use message_chat_boost::MessageChatBoost;

mod message_forum_topic_created;
pub use message_forum_topic_created::MessageForumTopicCreated;

mod message_forum_topic_edited;
pub use message_forum_topic_edited::MessageForumTopicEdited;

mod message_forum_topic_is_closed_toggled;
pub use message_forum_topic_is_closed_toggled::MessageForumTopicIsClosedToggled;

mod message_forum_topic_is_hidden_toggled;
pub use message_forum_topic_is_hidden_toggled::MessageForumTopicIsHiddenToggled;

mod message_suggest_profile_photo;
pub use message_suggest_profile_photo::MessageSuggestProfilePhoto;

mod message_suggest_birthdate;
pub use message_suggest_birthdate::MessageSuggestBirthdate;

mod message_custom_service_action;
pub use message_custom_service_action::MessageCustomServiceAction;

mod message_game_score;
pub use message_game_score::MessageGameScore;

mod message_payment_successful;
pub use message_payment_successful::MessagePaymentSuccessful;

mod message_payment_successful_bot;
pub use message_payment_successful_bot::MessagePaymentSuccessfulBot;

mod message_payment_refunded;
pub use message_payment_refunded::MessagePaymentRefunded;

mod message_gifted_premium;
pub use message_gifted_premium::MessageGiftedPremium;

mod message_premium_gift_code;
pub use message_premium_gift_code::MessagePremiumGiftCode;

mod message_giveaway_created;
pub use message_giveaway_created::MessageGiveawayCreated;

mod message_giveaway;
pub use message_giveaway::MessageGiveaway;

mod message_giveaway_completed;
pub use message_giveaway_completed::MessageGiveawayCompleted;

mod message_giveaway_winners;
pub use message_giveaway_winners::MessageGiveawayWinners;

mod message_gifted_stars;
pub use message_gifted_stars::MessageGiftedStars;

mod message_gifted_ton;
pub use message_gifted_ton::MessageGiftedTon;

mod message_giveaway_prize_stars;
pub use message_giveaway_prize_stars::MessageGiveawayPrizeStars;

mod message_gift;
pub use message_gift::MessageGift;

mod message_upgraded_gift;
pub use message_upgraded_gift::MessageUpgradedGift;

mod message_refunded_upgraded_gift;
pub use message_refunded_upgraded_gift::MessageRefundedUpgradedGift;

mod message_upgraded_gift_purchase_offer;
pub use message_upgraded_gift_purchase_offer::MessageUpgradedGiftPurchaseOffer;

mod message_upgraded_gift_purchase_offer_rejected;
pub use message_upgraded_gift_purchase_offer_rejected::MessageUpgradedGiftPurchaseOfferRejected;

mod message_paid_messages_refunded;
pub use message_paid_messages_refunded::MessagePaidMessagesRefunded;

mod message_paid_message_price_changed;
pub use message_paid_message_price_changed::MessagePaidMessagePriceChanged;

mod message_direct_message_price_changed;
pub use message_direct_message_price_changed::MessageDirectMessagePriceChanged;

mod message_checklist_tasks_done;
pub use message_checklist_tasks_done::MessageChecklistTasksDone;

mod message_checklist_tasks_added;
pub use message_checklist_tasks_added::MessageChecklistTasksAdded;

mod message_suggested_post_approval_failed;
pub use message_suggested_post_approval_failed::MessageSuggestedPostApprovalFailed;

mod message_suggested_post_approved;
pub use message_suggested_post_approved::MessageSuggestedPostApproved;

mod message_suggested_post_declined;
pub use message_suggested_post_declined::MessageSuggestedPostDeclined;

mod message_suggested_post_paid;
pub use message_suggested_post_paid::MessageSuggestedPostPaid;

mod message_suggested_post_refunded;
pub use message_suggested_post_refunded::MessageSuggestedPostRefunded;

mod message_users_shared;
pub use message_users_shared::MessageUsersShared;

mod message_chat_shared;
pub use message_chat_shared::MessageChatShared;

mod message_bot_write_access_allowed;
pub use message_bot_write_access_allowed::MessageBotWriteAccessAllowed;

mod message_web_app_data_sent;
pub use message_web_app_data_sent::MessageWebAppDataSent;

mod message_web_app_data_received;
pub use message_web_app_data_received::MessageWebAppDataReceived;

mod message_passport_data_sent;
pub use message_passport_data_sent::MessagePassportDataSent;

mod message_passport_data_received;
pub use message_passport_data_received::MessagePassportDataReceived;

mod message_proximity_alert_triggered;
pub use message_proximity_alert_triggered::MessageProximityAlertTriggered;

mod date_time_formatting_type_absolute;
pub use date_time_formatting_type_absolute::DateTimeFormattingTypeAbsolute;

mod text_entity_type_pre_code;
pub use text_entity_type_pre_code::TextEntityTypePreCode;

mod text_entity_type_text_url;
pub use text_entity_type_text_url::TextEntityTypeTextUrl;

mod text_entity_type_mention_name;
pub use text_entity_type_mention_name::TextEntityTypeMentionName;

mod text_entity_type_custom_emoji;
pub use text_entity_type_custom_emoji::TextEntityTypeCustomEmoji;

mod text_entity_type_media_timestamp;
pub use text_entity_type_media_timestamp::TextEntityTypeMediaTimestamp;

mod text_entity_type_date_time;
pub use text_entity_type_date_time::TextEntityTypeDateTime;

mod input_thumbnail;
pub use input_thumbnail::InputThumbnail;

mod input_paid_media_type_video;
pub use input_paid_media_type_video::InputPaidMediaTypeVideo;

mod input_paid_media;
pub use input_paid_media::InputPaidMedia;

mod message_scheduling_state_send_at_date;
pub use message_scheduling_state_send_at_date::MessageSchedulingStateSendAtDate;

mod message_scheduling_state_send_when_video_processed;
pub use message_scheduling_state_send_when_video_processed::MessageSchedulingStateSendWhenVideoProcessed;

mod message_self_destruct_type_timer;
pub use message_self_destruct_type_timer::MessageSelfDestructTypeTimer;

mod message_send_options;
pub use message_send_options::MessageSendOptions;

mod message_copy_options;
pub use message_copy_options::MessageCopyOptions;

mod input_message_text;
pub use input_message_text::InputMessageText;

mod input_message_animation;
pub use input_message_animation::InputMessageAnimation;

mod input_message_audio;
pub use input_message_audio::InputMessageAudio;

mod input_message_document;
pub use input_message_document::InputMessageDocument;

mod input_message_paid_media;
pub use input_message_paid_media::InputMessagePaidMedia;

mod input_message_photo;
pub use input_message_photo::InputMessagePhoto;

mod input_message_sticker;
pub use input_message_sticker::InputMessageSticker;

mod input_message_video;
pub use input_message_video::InputMessageVideo;

mod input_message_video_note;
pub use input_message_video_note::InputMessageVideoNote;

mod input_message_voice_note;
pub use input_message_voice_note::InputMessageVoiceNote;

mod input_message_location;
pub use input_message_location::InputMessageLocation;

mod input_message_venue;
pub use input_message_venue::InputMessageVenue;

mod input_message_contact;
pub use input_message_contact::InputMessageContact;

mod input_message_dice;
pub use input_message_dice::InputMessageDice;

mod input_message_game;
pub use input_message_game::InputMessageGame;

mod input_message_invoice;
pub use input_message_invoice::InputMessageInvoice;

mod input_message_poll;
pub use input_message_poll::InputMessagePoll;

mod input_message_stake_dice;
pub use input_message_stake_dice::InputMessageStakeDice;

mod input_message_story;
pub use input_message_story::InputMessageStory;

mod input_message_checklist;
pub use input_message_checklist::InputMessageChecklist;

mod input_message_forwarded;
pub use input_message_forwarded::InputMessageForwarded;

mod message_properties;
pub use message_properties::MessageProperties;

mod chat_action_uploading_video;
pub use chat_action_uploading_video::ChatActionUploadingVideo;

mod chat_action_uploading_voice_note;
pub use chat_action_uploading_voice_note::ChatActionUploadingVoiceNote;

mod chat_action_uploading_photo;
pub use chat_action_uploading_photo::ChatActionUploadingPhoto;

mod chat_action_uploading_document;
pub use chat_action_uploading_document::ChatActionUploadingDocument;

mod chat_action_uploading_video_note;
pub use chat_action_uploading_video_note::ChatActionUploadingVideoNote;

mod chat_action_watching_animations;
pub use chat_action_watching_animations::ChatActionWatchingAnimations;

mod user_status_online;
pub use user_status_online::UserStatusOnline;

mod user_status_offline;
pub use user_status_offline::UserStatusOffline;

mod user_status_recently;
pub use user_status_recently::UserStatusRecently;

mod user_status_last_week;
pub use user_status_last_week::UserStatusLastWeek;

mod user_status_last_month;
pub use user_status_last_month::UserStatusLastMonth;

mod emoji_keyword;
pub use emoji_keyword::EmojiKeyword;

mod emoji_keywords;
pub use emoji_keywords::EmojiKeywords;

mod stickers;
pub use stickers::Stickers;

mod emojis;
pub use emojis::Emojis;

mod sticker_set;
pub use sticker_set::StickerSet;

mod sticker_set_info;
pub use sticker_set_info::StickerSetInfo;

mod sticker_sets;
pub use sticker_sets::StickerSets;

mod trending_sticker_sets;
pub use trending_sticker_sets::TrendingStickerSets;

mod emoji_category_source_search;
pub use emoji_category_source_search::EmojiCategorySourceSearch;

mod emoji_category;
pub use emoji_category::EmojiCategory;

mod emoji_categories;
pub use emoji_categories::EmojiCategories;

mod current_weather;
pub use current_weather::CurrentWeather;

mod story_area_position;
pub use story_area_position::StoryAreaPosition;

mod story_area_type_location;
pub use story_area_type_location::StoryAreaTypeLocation;

mod story_area_type_venue;
pub use story_area_type_venue::StoryAreaTypeVenue;

mod story_area_type_suggested_reaction;
pub use story_area_type_suggested_reaction::StoryAreaTypeSuggestedReaction;

mod story_area_type_message;
pub use story_area_type_message::StoryAreaTypeMessage;

mod story_area_type_link;
pub use story_area_type_link::StoryAreaTypeLink;

mod story_area_type_weather;
pub use story_area_type_weather::StoryAreaTypeWeather;

mod story_area_type_upgraded_gift;
pub use story_area_type_upgraded_gift::StoryAreaTypeUpgradedGift;

mod story_area;
pub use story_area::StoryArea;

mod input_story_area_type_location;
pub use input_story_area_type_location::InputStoryAreaTypeLocation;

mod input_story_area_type_found_venue;
pub use input_story_area_type_found_venue::InputStoryAreaTypeFoundVenue;

mod input_story_area_type_previous_venue;
pub use input_story_area_type_previous_venue::InputStoryAreaTypePreviousVenue;

mod input_story_area_type_suggested_reaction;
pub use input_story_area_type_suggested_reaction::InputStoryAreaTypeSuggestedReaction;

mod input_story_area_type_message;
pub use input_story_area_type_message::InputStoryAreaTypeMessage;

mod input_story_area_type_link;
pub use input_story_area_type_link::InputStoryAreaTypeLink;

mod input_story_area_type_weather;
pub use input_story_area_type_weather::InputStoryAreaTypeWeather;

mod input_story_area_type_upgraded_gift;
pub use input_story_area_type_upgraded_gift::InputStoryAreaTypeUpgradedGift;

mod input_story_area;
pub use input_story_area::InputStoryArea;

mod input_story_areas;
pub use input_story_areas::InputStoryAreas;

mod story_video;
pub use story_video::StoryVideo;

mod story_content_photo;
pub use story_content_photo::StoryContentPhoto;

mod story_content_video;
pub use story_content_video::StoryContentVideo;

mod story_content_live;
pub use story_content_live::StoryContentLive;

mod input_story_content_photo;
pub use input_story_content_photo::InputStoryContentPhoto;

mod input_story_content_video;
pub use input_story_content_video::InputStoryContentVideo;

mod story_origin_public_story;
pub use story_origin_public_story::StoryOriginPublicStory;

mod story_origin_hidden_user;
pub use story_origin_hidden_user::StoryOriginHiddenUser;

mod story_repost_info;
pub use story_repost_info::StoryRepostInfo;

mod story_interaction_info;
pub use story_interaction_info::StoryInteractionInfo;

mod story;
pub use story::Story;

mod stories;
pub use stories::Stories;

mod found_stories;
pub use found_stories::FoundStories;

mod story_album;
pub use story_album::StoryAlbum;

mod story_albums;
pub use story_albums::StoryAlbums;

mod story_full_id;
pub use story_full_id::StoryFullId;

mod story_info;
pub use story_info::StoryInfo;

mod chat_active_stories;
pub use chat_active_stories::ChatActiveStories;

mod story_interaction_type_view;
pub use story_interaction_type_view::StoryInteractionTypeView;

mod story_interaction_type_forward;
pub use story_interaction_type_forward::StoryInteractionTypeForward;

mod story_interaction_type_repost;
pub use story_interaction_type_repost::StoryInteractionTypeRepost;

mod story_interaction;
pub use story_interaction::StoryInteraction;

mod story_interactions;
pub use story_interactions::StoryInteractions;

mod quick_reply_message;
pub use quick_reply_message::QuickReplyMessage;

mod quick_reply_messages;
pub use quick_reply_messages::QuickReplyMessages;

mod quick_reply_shortcut;
pub use quick_reply_shortcut::QuickReplyShortcut;

mod public_forward_message;
pub use public_forward_message::PublicForwardMessage;

mod public_forward_story;
pub use public_forward_story::PublicForwardStory;

mod public_forwards;
pub use public_forwards::PublicForwards;

mod bot_media_preview;
pub use bot_media_preview::BotMediaPreview;

mod bot_media_previews;
pub use bot_media_previews::BotMediaPreviews;

mod bot_media_preview_info;
pub use bot_media_preview_info::BotMediaPreviewInfo;

mod chat_boost_level_features;
pub use chat_boost_level_features::ChatBoostLevelFeatures;

mod chat_boost_features;
pub use chat_boost_features::ChatBoostFeatures;

mod chat_boost_source_gift_code;
pub use chat_boost_source_gift_code::ChatBoostSourceGiftCode;

mod chat_boost_source_giveaway;
pub use chat_boost_source_giveaway::ChatBoostSourceGiveaway;

mod chat_boost_source_premium;
pub use chat_boost_source_premium::ChatBoostSourcePremium;

mod prepaid_giveaway;
pub use prepaid_giveaway::PrepaidGiveaway;

mod chat_boost_status;
pub use chat_boost_status::ChatBoostStatus;

mod chat_boost;
pub use chat_boost::ChatBoost;

mod found_chat_boosts;
pub use found_chat_boosts::FoundChatBoosts;

mod chat_boost_slot;
pub use chat_boost_slot::ChatBoostSlot;

mod chat_boost_slots;
pub use chat_boost_slots::ChatBoostSlots;

mod resend_code_reason_verification_failed;
pub use resend_code_reason_verification_failed::ResendCodeReasonVerificationFailed;

mod call_discard_reason_upgrade_to_group_call;
pub use call_discard_reason_upgrade_to_group_call::CallDiscardReasonUpgradeToGroupCall;

mod call_protocol;
pub use call_protocol::CallProtocol;

mod call_server_type_telegram_reflector;
pub use call_server_type_telegram_reflector::CallServerTypeTelegramReflector;

mod call_server_type_webrtc;
pub use call_server_type_webrtc::CallServerTypeWebrtc;

mod call_server;
pub use call_server::CallServer;

mod call_id;
pub use call_id::CallId;

mod group_call_id;
pub use group_call_id::GroupCallId;

mod input_call_discarded;
pub use input_call_discarded::InputCallDiscarded;

mod input_call_from_message;
pub use input_call_from_message::InputCallFromMessage;

mod call_state_pending;
pub use call_state_pending::CallStatePending;

mod call_state_ready;
pub use call_state_ready::CallStateReady;

mod call_state_discarded;
pub use call_state_discarded::CallStateDiscarded;

mod call_state_error;
pub use call_state_error::CallStateError;

mod group_call_join_parameters;
pub use group_call_join_parameters::GroupCallJoinParameters;

mod group_call_stream;
pub use group_call_stream::GroupCallStream;

mod group_call_streams;
pub use group_call_streams::GroupCallStreams;

mod rtmp_url;
pub use rtmp_url::RtmpUrl;

mod group_call_recent_speaker;
pub use group_call_recent_speaker::GroupCallRecentSpeaker;

mod group_call;
pub use group_call::GroupCall;

mod group_call_video_source_group;
pub use group_call_video_source_group::GroupCallVideoSourceGroup;

mod group_call_participant_video_info;
pub use group_call_participant_video_info::GroupCallParticipantVideoInfo;

mod group_call_participant;
pub use group_call_participant::GroupCallParticipant;

mod group_call_participants;
pub use group_call_participants::GroupCallParticipants;

mod group_call_info;
pub use group_call_info::GroupCallInfo;

mod group_call_message;
pub use group_call_message::GroupCallMessage;

mod group_call_message_level;
pub use group_call_message_level::GroupCallMessageLevel;

mod invite_group_call_participant_result_success;
pub use invite_group_call_participant_result_success::InviteGroupCallParticipantResultSuccess;

mod input_group_call_link;
pub use input_group_call_link::InputGroupCallLink;

mod input_group_call_message;
pub use input_group_call_message::InputGroupCallMessage;

mod call;
pub use call::Call;

mod firebase_authentication_settings_ios;
pub use firebase_authentication_settings_ios::FirebaseAuthenticationSettingsIos;

mod phone_number_authentication_settings;
pub use phone_number_authentication_settings::PhoneNumberAuthenticationSettings;

mod added_reaction;
pub use added_reaction::AddedReaction;

mod added_reactions;
pub use added_reactions::AddedReactions;

mod available_reaction;
pub use available_reaction::AvailableReaction;

mod available_reactions;
pub use available_reactions::AvailableReactions;

mod emoji_reaction;
pub use emoji_reaction::EmojiReaction;

mod animations;
pub use animations::Animations;

mod dice_stickers_regular;
pub use dice_stickers_regular::DiceStickersRegular;

mod dice_stickers_slot_machine;
pub use dice_stickers_slot_machine::DiceStickersSlotMachine;

mod imported_contact;
pub use imported_contact::ImportedContact;

mod imported_contacts;
pub use imported_contacts::ImportedContacts;

mod speech_recognition_result_pending;
pub use speech_recognition_result_pending::SpeechRecognitionResultPending;

mod speech_recognition_result_text;
pub use speech_recognition_result_text::SpeechRecognitionResultText;

mod speech_recognition_result_error;
pub use speech_recognition_result_error::SpeechRecognitionResultError;

mod business_connection;
pub use business_connection::BusinessConnection;

mod attachment_menu_bot_color;
pub use attachment_menu_bot_color::AttachmentMenuBotColor;

mod attachment_menu_bot;
pub use attachment_menu_bot::AttachmentMenuBot;

mod sent_web_app_message;
pub use sent_web_app_message::SentWebAppMessage;

mod bot_write_access_allow_reason_connected_website;
pub use bot_write_access_allow_reason_connected_website::BotWriteAccessAllowReasonConnectedWebsite;

mod bot_write_access_allow_reason_launched_web_app;
pub use bot_write_access_allow_reason_launched_web_app::BotWriteAccessAllowReasonLaunchedWebApp;

mod http_url;
pub use http_url::HttpUrl;

mod user_link;
pub use user_link::UserLink;

mod target_chat_types;
pub use target_chat_types::TargetChatTypes;

mod target_chat_chosen;
pub use target_chat_chosen::TargetChatChosen;

mod target_chat_internal_link;
pub use target_chat_internal_link::TargetChatInternalLink;

mod input_inline_query_result_animation;
pub use input_inline_query_result_animation::InputInlineQueryResultAnimation;

mod input_inline_query_result_article;
pub use input_inline_query_result_article::InputInlineQueryResultArticle;

mod input_inline_query_result_audio;
pub use input_inline_query_result_audio::InputInlineQueryResultAudio;

mod input_inline_query_result_contact;
pub use input_inline_query_result_contact::InputInlineQueryResultContact;

mod input_inline_query_result_document;
pub use input_inline_query_result_document::InputInlineQueryResultDocument;

mod input_inline_query_result_game;
pub use input_inline_query_result_game::InputInlineQueryResultGame;

mod input_inline_query_result_location;
pub use input_inline_query_result_location::InputInlineQueryResultLocation;

mod input_inline_query_result_photo;
pub use input_inline_query_result_photo::InputInlineQueryResultPhoto;

mod input_inline_query_result_sticker;
pub use input_inline_query_result_sticker::InputInlineQueryResultSticker;

mod input_inline_query_result_venue;
pub use input_inline_query_result_venue::InputInlineQueryResultVenue;

mod input_inline_query_result_video;
pub use input_inline_query_result_video::InputInlineQueryResultVideo;

mod input_inline_query_result_voice_note;
pub use input_inline_query_result_voice_note::InputInlineQueryResultVoiceNote;

mod inline_query_result_article;
pub use inline_query_result_article::InlineQueryResultArticle;

mod inline_query_result_contact;
pub use inline_query_result_contact::InlineQueryResultContact;

mod inline_query_result_location;
pub use inline_query_result_location::InlineQueryResultLocation;

mod inline_query_result_venue;
pub use inline_query_result_venue::InlineQueryResultVenue;

mod inline_query_result_game;
pub use inline_query_result_game::InlineQueryResultGame;

mod inline_query_result_animation;
pub use inline_query_result_animation::InlineQueryResultAnimation;

mod inline_query_result_audio;
pub use inline_query_result_audio::InlineQueryResultAudio;

mod inline_query_result_document;
pub use inline_query_result_document::InlineQueryResultDocument;

mod inline_query_result_photo;
pub use inline_query_result_photo::InlineQueryResultPhoto;

mod inline_query_result_sticker;
pub use inline_query_result_sticker::InlineQueryResultSticker;

mod inline_query_result_video;
pub use inline_query_result_video::InlineQueryResultVideo;

mod inline_query_result_voice_note;
pub use inline_query_result_voice_note::InlineQueryResultVoiceNote;

mod inline_query_results_button_type_start_bot;
pub use inline_query_results_button_type_start_bot::InlineQueryResultsButtonTypeStartBot;

mod inline_query_results_button_type_web_app;
pub use inline_query_results_button_type_web_app::InlineQueryResultsButtonTypeWebApp;

mod inline_query_results_button;
pub use inline_query_results_button::InlineQueryResultsButton;

mod inline_query_results;
pub use inline_query_results::InlineQueryResults;

mod prepared_inline_message_id;
pub use prepared_inline_message_id::PreparedInlineMessageId;

mod prepared_inline_message;
pub use prepared_inline_message::PreparedInlineMessage;

mod callback_query_payload_data;
pub use callback_query_payload_data::CallbackQueryPayloadData;

mod callback_query_payload_data_with_password;
pub use callback_query_payload_data_with_password::CallbackQueryPayloadDataWithPassword;

mod callback_query_payload_game;
pub use callback_query_payload_game::CallbackQueryPayloadGame;

mod callback_query_answer;
pub use callback_query_answer::CallbackQueryAnswer;

mod custom_request_result;
pub use custom_request_result::CustomRequestResult;

mod game_high_score;
pub use game_high_score::GameHighScore;

mod game_high_scores;
pub use game_high_scores::GameHighScores;

mod chat_event_message_edited;
pub use chat_event_message_edited::ChatEventMessageEdited;

mod chat_event_message_deleted;
pub use chat_event_message_deleted::ChatEventMessageDeleted;

mod chat_event_message_pinned;
pub use chat_event_message_pinned::ChatEventMessagePinned;

mod chat_event_message_unpinned;
pub use chat_event_message_unpinned::ChatEventMessageUnpinned;

mod chat_event_poll_stopped;
pub use chat_event_poll_stopped::ChatEventPollStopped;

mod chat_event_member_joined_by_invite_link;
pub use chat_event_member_joined_by_invite_link::ChatEventMemberJoinedByInviteLink;

mod chat_event_member_joined_by_request;
pub use chat_event_member_joined_by_request::ChatEventMemberJoinedByRequest;

mod chat_event_member_invited;
pub use chat_event_member_invited::ChatEventMemberInvited;

mod chat_event_member_promoted;
pub use chat_event_member_promoted::ChatEventMemberPromoted;

mod chat_event_member_restricted;
pub use chat_event_member_restricted::ChatEventMemberRestricted;

mod chat_event_member_tag_changed;
pub use chat_event_member_tag_changed::ChatEventMemberTagChanged;

mod chat_event_member_subscription_extended;
pub use chat_event_member_subscription_extended::ChatEventMemberSubscriptionExtended;

mod chat_event_available_reactions_changed;
pub use chat_event_available_reactions_changed::ChatEventAvailableReactionsChanged;

mod chat_event_background_changed;
pub use chat_event_background_changed::ChatEventBackgroundChanged;

mod chat_event_description_changed;
pub use chat_event_description_changed::ChatEventDescriptionChanged;

mod chat_event_emoji_status_changed;
pub use chat_event_emoji_status_changed::ChatEventEmojiStatusChanged;

mod chat_event_linked_chat_changed;
pub use chat_event_linked_chat_changed::ChatEventLinkedChatChanged;

mod chat_event_location_changed;
pub use chat_event_location_changed::ChatEventLocationChanged;

mod chat_event_message_auto_delete_time_changed;
pub use chat_event_message_auto_delete_time_changed::ChatEventMessageAutoDeleteTimeChanged;

mod chat_event_permissions_changed;
pub use chat_event_permissions_changed::ChatEventPermissionsChanged;

mod chat_event_photo_changed;
pub use chat_event_photo_changed::ChatEventPhotoChanged;

mod chat_event_slow_mode_delay_changed;
pub use chat_event_slow_mode_delay_changed::ChatEventSlowModeDelayChanged;

mod chat_event_sticker_set_changed;
pub use chat_event_sticker_set_changed::ChatEventStickerSetChanged;

mod chat_event_custom_emoji_sticker_set_changed;
pub use chat_event_custom_emoji_sticker_set_changed::ChatEventCustomEmojiStickerSetChanged;

mod chat_event_title_changed;
pub use chat_event_title_changed::ChatEventTitleChanged;

mod chat_event_username_changed;
pub use chat_event_username_changed::ChatEventUsernameChanged;

mod chat_event_active_usernames_changed;
pub use chat_event_active_usernames_changed::ChatEventActiveUsernamesChanged;

mod chat_event_accent_color_changed;
pub use chat_event_accent_color_changed::ChatEventAccentColorChanged;

mod chat_event_profile_accent_color_changed;
pub use chat_event_profile_accent_color_changed::ChatEventProfileAccentColorChanged;

mod chat_event_has_protected_content_toggled;
pub use chat_event_has_protected_content_toggled::ChatEventHasProtectedContentToggled;

mod chat_event_invites_toggled;
pub use chat_event_invites_toggled::ChatEventInvitesToggled;

mod chat_event_is_all_history_available_toggled;
pub use chat_event_is_all_history_available_toggled::ChatEventIsAllHistoryAvailableToggled;

mod chat_event_has_aggressive_anti_spam_enabled_toggled;
pub use chat_event_has_aggressive_anti_spam_enabled_toggled::ChatEventHasAggressiveAntiSpamEnabledToggled;

mod chat_event_sign_messages_toggled;
pub use chat_event_sign_messages_toggled::ChatEventSignMessagesToggled;

mod chat_event_show_message_sender_toggled;
pub use chat_event_show_message_sender_toggled::ChatEventShowMessageSenderToggled;

mod chat_event_automatic_translation_toggled;
pub use chat_event_automatic_translation_toggled::ChatEventAutomaticTranslationToggled;

mod chat_event_invite_link_edited;
pub use chat_event_invite_link_edited::ChatEventInviteLinkEdited;

mod chat_event_invite_link_revoked;
pub use chat_event_invite_link_revoked::ChatEventInviteLinkRevoked;

mod chat_event_invite_link_deleted;
pub use chat_event_invite_link_deleted::ChatEventInviteLinkDeleted;

mod chat_event_video_chat_created;
pub use chat_event_video_chat_created::ChatEventVideoChatCreated;

mod chat_event_video_chat_ended;
pub use chat_event_video_chat_ended::ChatEventVideoChatEnded;

mod chat_event_video_chat_mute_new_participants_toggled;
pub use chat_event_video_chat_mute_new_participants_toggled::ChatEventVideoChatMuteNewParticipantsToggled;

mod chat_event_video_chat_participant_is_muted_toggled;
pub use chat_event_video_chat_participant_is_muted_toggled::ChatEventVideoChatParticipantIsMutedToggled;

mod chat_event_video_chat_participant_volume_level_changed;
pub use chat_event_video_chat_participant_volume_level_changed::ChatEventVideoChatParticipantVolumeLevelChanged;

mod chat_event_is_forum_toggled;
pub use chat_event_is_forum_toggled::ChatEventIsForumToggled;

mod chat_event_forum_topic_created;
pub use chat_event_forum_topic_created::ChatEventForumTopicCreated;

mod chat_event_forum_topic_edited;
pub use chat_event_forum_topic_edited::ChatEventForumTopicEdited;

mod chat_event_forum_topic_toggle_is_closed;
pub use chat_event_forum_topic_toggle_is_closed::ChatEventForumTopicToggleIsClosed;

mod chat_event_forum_topic_toggle_is_hidden;
pub use chat_event_forum_topic_toggle_is_hidden::ChatEventForumTopicToggleIsHidden;

mod chat_event_forum_topic_deleted;
pub use chat_event_forum_topic_deleted::ChatEventForumTopicDeleted;

mod chat_event_forum_topic_pinned;
pub use chat_event_forum_topic_pinned::ChatEventForumTopicPinned;

mod chat_event;
pub use chat_event::ChatEvent;

mod chat_events;
pub use chat_events::ChatEvents;

mod chat_event_log_filters;
pub use chat_event_log_filters::ChatEventLogFilters;

mod language_pack_string_value_ordinary;
pub use language_pack_string_value_ordinary::LanguagePackStringValueOrdinary;

mod language_pack_string_value_pluralized;
pub use language_pack_string_value_pluralized::LanguagePackStringValuePluralized;

mod language_pack_string;
pub use language_pack_string::LanguagePackString;

mod language_pack_strings;
pub use language_pack_strings::LanguagePackStrings;

mod language_pack_info;
pub use language_pack_info::LanguagePackInfo;

mod localization_target_info;
pub use localization_target_info::LocalizationTargetInfo;

mod premium_limit;
pub use premium_limit::PremiumLimit;

mod premium_features;
pub use premium_features::PremiumFeatures;

mod business_features;
pub use business_features::BusinessFeatures;

mod premium_source_limit_exceeded;
pub use premium_source_limit_exceeded::PremiumSourceLimitExceeded;

mod premium_source_feature;
pub use premium_source_feature::PremiumSourceFeature;

mod premium_source_business_feature;
pub use premium_source_business_feature::PremiumSourceBusinessFeature;

mod premium_source_story_feature;
pub use premium_source_story_feature::PremiumSourceStoryFeature;

mod premium_source_link;
pub use premium_source_link::PremiumSourceLink;

mod premium_feature_promotion_animation;
pub use premium_feature_promotion_animation::PremiumFeaturePromotionAnimation;

mod business_feature_promotion_animation;
pub use business_feature_promotion_animation::BusinessFeaturePromotionAnimation;

mod premium_state;
pub use premium_state::PremiumState;

mod store_payment_purpose_premium_subscription;
pub use store_payment_purpose_premium_subscription::StorePaymentPurposePremiumSubscription;

mod store_payment_purpose_premium_gift;
pub use store_payment_purpose_premium_gift::StorePaymentPurposePremiumGift;

mod store_payment_purpose_premium_gift_codes;
pub use store_payment_purpose_premium_gift_codes::StorePaymentPurposePremiumGiftCodes;

mod store_payment_purpose_premium_giveaway;
pub use store_payment_purpose_premium_giveaway::StorePaymentPurposePremiumGiveaway;

mod store_payment_purpose_star_giveaway;
pub use store_payment_purpose_star_giveaway::StorePaymentPurposeStarGiveaway;

mod store_payment_purpose_stars;
pub use store_payment_purpose_stars::StorePaymentPurposeStars;

mod store_payment_purpose_gifted_stars;
pub use store_payment_purpose_gifted_stars::StorePaymentPurposeGiftedStars;

mod store_transaction_app_store;
pub use store_transaction_app_store::StoreTransactionAppStore;

mod store_transaction_google_play;
pub use store_transaction_google_play::StoreTransactionGooglePlay;

mod telegram_payment_purpose_premium_gift;
pub use telegram_payment_purpose_premium_gift::TelegramPaymentPurposePremiumGift;

mod telegram_payment_purpose_premium_gift_codes;
pub use telegram_payment_purpose_premium_gift_codes::TelegramPaymentPurposePremiumGiftCodes;

mod telegram_payment_purpose_premium_giveaway;
pub use telegram_payment_purpose_premium_giveaway::TelegramPaymentPurposePremiumGiveaway;

mod telegram_payment_purpose_stars;
pub use telegram_payment_purpose_stars::TelegramPaymentPurposeStars;

mod telegram_payment_purpose_gifted_stars;
pub use telegram_payment_purpose_gifted_stars::TelegramPaymentPurposeGiftedStars;

mod telegram_payment_purpose_star_giveaway;
pub use telegram_payment_purpose_star_giveaway::TelegramPaymentPurposeStarGiveaway;

mod telegram_payment_purpose_join_chat;
pub use telegram_payment_purpose_join_chat::TelegramPaymentPurposeJoinChat;

mod device_token_firebase_cloud_messaging;
pub use device_token_firebase_cloud_messaging::DeviceTokenFirebaseCloudMessaging;

mod device_token_apple_push;
pub use device_token_apple_push::DeviceTokenApplePush;

mod device_token_apple_push_vo_ip;
pub use device_token_apple_push_vo_ip::DeviceTokenApplePushVoIp;

mod device_token_windows_push;
pub use device_token_windows_push::DeviceTokenWindowsPush;

mod device_token_microsoft_push;
pub use device_token_microsoft_push::DeviceTokenMicrosoftPush;

mod device_token_microsoft_push_vo_ip;
pub use device_token_microsoft_push_vo_ip::DeviceTokenMicrosoftPushVoIp;

mod device_token_web_push;
pub use device_token_web_push::DeviceTokenWebPush;

mod device_token_simple_push;
pub use device_token_simple_push::DeviceTokenSimplePush;

mod device_token_ubuntu_push;
pub use device_token_ubuntu_push::DeviceTokenUbuntuPush;

mod device_token_black_berry_push;
pub use device_token_black_berry_push::DeviceTokenBlackBerryPush;

mod device_token_tizen_push;
pub use device_token_tizen_push::DeviceTokenTizenPush;

mod device_token_huawei_push;
pub use device_token_huawei_push::DeviceTokenHuaweiPush;

mod push_receiver_id;
pub use push_receiver_id::PushReceiverId;

mod background_fill_solid;
pub use background_fill_solid::BackgroundFillSolid;

mod background_fill_gradient;
pub use background_fill_gradient::BackgroundFillGradient;

mod background_fill_freeform_gradient;
pub use background_fill_freeform_gradient::BackgroundFillFreeformGradient;

mod background_type_wallpaper;
pub use background_type_wallpaper::BackgroundTypeWallpaper;

mod background_type_pattern;
pub use background_type_pattern::BackgroundTypePattern;

mod background_type_fill;
pub use background_type_fill::BackgroundTypeFill;

mod background_type_chat_theme;
pub use background_type_chat_theme::BackgroundTypeChatTheme;

mod input_background_local;
pub use input_background_local::InputBackgroundLocal;

mod input_background_remote;
pub use input_background_remote::InputBackgroundRemote;

mod input_background_previous;
pub use input_background_previous::InputBackgroundPrevious;

mod emoji_chat_theme;
pub use emoji_chat_theme::EmojiChatTheme;

mod gift_chat_theme;
pub use gift_chat_theme::GiftChatTheme;

mod gift_chat_themes;
pub use gift_chat_themes::GiftChatThemes;

mod chat_theme_emoji;
pub use chat_theme_emoji::ChatThemeEmoji;

mod chat_theme_gift;
pub use chat_theme_gift::ChatThemeGift;

mod input_chat_theme_emoji;
pub use input_chat_theme_emoji::InputChatThemeEmoji;

mod input_chat_theme_gift;
pub use input_chat_theme_gift::InputChatThemeGift;

mod time_zone;
pub use time_zone::TimeZone;

mod time_zones;
pub use time_zones::TimeZones;

mod hashtags;
pub use hashtags::Hashtags;

mod can_post_story_result_ok;
pub use can_post_story_result_ok::CanPostStoryResultOk;

mod can_post_story_result_weekly_limit_exceeded;
pub use can_post_story_result_weekly_limit_exceeded::CanPostStoryResultWeeklyLimitExceeded;

mod can_post_story_result_monthly_limit_exceeded;
pub use can_post_story_result_monthly_limit_exceeded::CanPostStoryResultMonthlyLimitExceeded;

mod can_post_story_result_live_story_is_active;
pub use can_post_story_result_live_story_is_active::CanPostStoryResultLiveStoryIsActive;

mod start_live_story_result_ok;
pub use start_live_story_result_ok::StartLiveStoryResultOk;

mod start_live_story_result_fail;
pub use start_live_story_result_fail::StartLiveStoryResultFail;

mod can_transfer_ownership_result_password_too_fresh;
pub use can_transfer_ownership_result_password_too_fresh::CanTransferOwnershipResultPasswordTooFresh;

mod can_transfer_ownership_result_session_too_fresh;
pub use can_transfer_ownership_result_session_too_fresh::CanTransferOwnershipResultSessionTooFresh;

mod reset_password_result_pending;
pub use reset_password_result_pending::ResetPasswordResultPending;

mod reset_password_result_declined;
pub use reset_password_result_declined::ResetPasswordResultDeclined;

mod message_file_type_private;
pub use message_file_type_private::MessageFileTypePrivate;

mod message_file_type_group;
pub use message_file_type_group::MessageFileTypeGroup;

mod push_message_content_hidden;
pub use push_message_content_hidden::PushMessageContentHidden;

mod push_message_content_animation;
pub use push_message_content_animation::PushMessageContentAnimation;

mod push_message_content_audio;
pub use push_message_content_audio::PushMessageContentAudio;

mod push_message_content_contact;
pub use push_message_content_contact::PushMessageContentContact;

mod push_message_content_contact_registered;
pub use push_message_content_contact_registered::PushMessageContentContactRegistered;

mod push_message_content_document;
pub use push_message_content_document::PushMessageContentDocument;

mod push_message_content_game;
pub use push_message_content_game::PushMessageContentGame;

mod push_message_content_game_score;
pub use push_message_content_game_score::PushMessageContentGameScore;

mod push_message_content_invoice;
pub use push_message_content_invoice::PushMessageContentInvoice;

mod push_message_content_location;
pub use push_message_content_location::PushMessageContentLocation;

mod push_message_content_paid_media;
pub use push_message_content_paid_media::PushMessageContentPaidMedia;

mod push_message_content_photo;
pub use push_message_content_photo::PushMessageContentPhoto;

mod push_message_content_poll;
pub use push_message_content_poll::PushMessageContentPoll;

mod push_message_content_premium_gift_code;
pub use push_message_content_premium_gift_code::PushMessageContentPremiumGiftCode;

mod push_message_content_giveaway;
pub use push_message_content_giveaway::PushMessageContentGiveaway;

mod push_message_content_gift;
pub use push_message_content_gift::PushMessageContentGift;

mod push_message_content_upgraded_gift;
pub use push_message_content_upgraded_gift::PushMessageContentUpgradedGift;

mod push_message_content_sticker;
pub use push_message_content_sticker::PushMessageContentSticker;

mod push_message_content_story;
pub use push_message_content_story::PushMessageContentStory;

mod push_message_content_text;
pub use push_message_content_text::PushMessageContentText;

mod push_message_content_checklist;
pub use push_message_content_checklist::PushMessageContentChecklist;

mod push_message_content_video;
pub use push_message_content_video::PushMessageContentVideo;

mod push_message_content_video_note;
pub use push_message_content_video_note::PushMessageContentVideoNote;

mod push_message_content_voice_note;
pub use push_message_content_voice_note::PushMessageContentVoiceNote;

mod push_message_content_invite_video_chat_participants;
pub use push_message_content_invite_video_chat_participants::PushMessageContentInviteVideoChatParticipants;

mod push_message_content_chat_add_members;
pub use push_message_content_chat_add_members::PushMessageContentChatAddMembers;

mod push_message_content_chat_change_title;
pub use push_message_content_chat_change_title::PushMessageContentChatChangeTitle;

mod push_message_content_chat_set_background;
pub use push_message_content_chat_set_background::PushMessageContentChatSetBackground;

mod push_message_content_chat_set_theme;
pub use push_message_content_chat_set_theme::PushMessageContentChatSetTheme;

mod push_message_content_chat_delete_member;
pub use push_message_content_chat_delete_member::PushMessageContentChatDeleteMember;

mod push_message_content_recurring_payment;
pub use push_message_content_recurring_payment::PushMessageContentRecurringPayment;

mod push_message_content_proximity_alert_triggered;
pub use push_message_content_proximity_alert_triggered::PushMessageContentProximityAlertTriggered;

mod push_message_content_checklist_tasks_added;
pub use push_message_content_checklist_tasks_added::PushMessageContentChecklistTasksAdded;

mod push_message_content_checklist_tasks_done;
pub use push_message_content_checklist_tasks_done::PushMessageContentChecklistTasksDone;

mod push_message_content_message_forwards;
pub use push_message_content_message_forwards::PushMessageContentMessageForwards;

mod push_message_content_media_album;
pub use push_message_content_media_album::PushMessageContentMediaAlbum;

mod notification_type_new_message;
pub use notification_type_new_message::NotificationTypeNewMessage;

mod notification_type_new_call;
pub use notification_type_new_call::NotificationTypeNewCall;

mod notification_type_new_push_message;
pub use notification_type_new_push_message::NotificationTypeNewPushMessage;

mod notification_sound;
pub use notification_sound::NotificationSound;

mod notification_sounds;
pub use notification_sounds::NotificationSounds;

mod notification;
pub use notification::Notification;

mod notification_group;
pub use notification_group::NotificationGroup;

mod proxy;
pub use proxy::Proxy;

mod option_value_boolean;
pub use option_value_boolean::OptionValueBoolean;

mod option_value_integer;
pub use option_value_integer::OptionValueInteger;

mod option_value_string;
pub use option_value_string::OptionValueString;

mod json_object_member;
pub use json_object_member::JsonObjectMember;

mod json_value_boolean;
pub use json_value_boolean::JsonValueBoolean;

mod json_value_number;
pub use json_value_number::JsonValueNumber;

mod json_value_string;
pub use json_value_string::JsonValueString;

mod json_value_array;
pub use json_value_array::JsonValueArray;

mod json_value_object;
pub use json_value_object::JsonValueObject;

mod story_privacy_settings_everyone;
pub use story_privacy_settings_everyone::StoryPrivacySettingsEveryone;

mod story_privacy_settings_contacts;
pub use story_privacy_settings_contacts::StoryPrivacySettingsContacts;

mod story_privacy_settings_selected_users;
pub use story_privacy_settings_selected_users::StoryPrivacySettingsSelectedUsers;

mod user_privacy_setting_rule_allow_users;
pub use user_privacy_setting_rule_allow_users::UserPrivacySettingRuleAllowUsers;

mod user_privacy_setting_rule_allow_chat_members;
pub use user_privacy_setting_rule_allow_chat_members::UserPrivacySettingRuleAllowChatMembers;

mod user_privacy_setting_rule_restrict_users;
pub use user_privacy_setting_rule_restrict_users::UserPrivacySettingRuleRestrictUsers;

mod user_privacy_setting_rule_restrict_chat_members;
pub use user_privacy_setting_rule_restrict_chat_members::UserPrivacySettingRuleRestrictChatMembers;

mod user_privacy_setting_rules;
pub use user_privacy_setting_rules::UserPrivacySettingRules;

mod read_date_privacy_settings;
pub use read_date_privacy_settings::ReadDatePrivacySettings;

mod new_chat_privacy_settings;
pub use new_chat_privacy_settings::NewChatPrivacySettings;

mod can_send_message_to_user_result_user_has_paid_messages;
pub use can_send_message_to_user_result_user_has_paid_messages::CanSendMessageToUserResultUserHasPaidMessages;

mod account_ttl;
pub use account_ttl::AccountTtl;

mod message_auto_delete_time;
pub use message_auto_delete_time::MessageAutoDeleteTime;

mod session;
pub use session::Session;

mod sessions;
pub use sessions::Sessions;

mod unconfirmed_session;
pub use unconfirmed_session::UnconfirmedSession;

mod connected_website;
pub use connected_website::ConnectedWebsite;

mod connected_websites;
pub use connected_websites::ConnectedWebsites;

mod report_chat_result_option_required;
pub use report_chat_result_option_required::ReportChatResultOptionRequired;

mod report_chat_result_text_required;
pub use report_chat_result_text_required::ReportChatResultTextRequired;

mod report_story_result_option_required;
pub use report_story_result_option_required::ReportStoryResultOptionRequired;

mod report_story_result_text_required;
pub use report_story_result_text_required::ReportStoryResultTextRequired;

mod settings_section_appearance;
pub use settings_section_appearance::SettingsSectionAppearance;

mod settings_section_business;
pub use settings_section_business::SettingsSectionBusiness;

mod settings_section_chat_folders;
pub use settings_section_chat_folders::SettingsSectionChatFolders;

mod settings_section_data_and_storage;
pub use settings_section_data_and_storage::SettingsSectionDataAndStorage;

mod settings_section_devices;
pub use settings_section_devices::SettingsSectionDevices;

mod settings_section_edit_profile;
pub use settings_section_edit_profile::SettingsSectionEditProfile;

mod settings_section_in_app_browser;
pub use settings_section_in_app_browser::SettingsSectionInAppBrowser;

mod settings_section_language;
pub use settings_section_language::SettingsSectionLanguage;

mod settings_section_my_stars;
pub use settings_section_my_stars::SettingsSectionMyStars;

mod settings_section_notifications;
pub use settings_section_notifications::SettingsSectionNotifications;

mod settings_section_power_saving;
pub use settings_section_power_saving::SettingsSectionPowerSaving;

mod settings_section_privacy_and_security;
pub use settings_section_privacy_and_security::SettingsSectionPrivacyAndSecurity;

mod settings_section_qr_code;
pub use settings_section_qr_code::SettingsSectionQrCode;

mod settings_section_send_gift;
pub use settings_section_send_gift::SettingsSectionSendGift;

mod internal_link_type_attachment_menu_bot;
pub use internal_link_type_attachment_menu_bot::InternalLinkTypeAttachmentMenuBot;

mod internal_link_type_authentication_code;
pub use internal_link_type_authentication_code::InternalLinkTypeAuthenticationCode;

mod internal_link_type_background;
pub use internal_link_type_background::InternalLinkTypeBackground;

mod internal_link_type_bot_add_to_channel;
pub use internal_link_type_bot_add_to_channel::InternalLinkTypeBotAddToChannel;

mod internal_link_type_bot_start;
pub use internal_link_type_bot_start::InternalLinkTypeBotStart;

mod internal_link_type_bot_start_in_group;
pub use internal_link_type_bot_start_in_group::InternalLinkTypeBotStartInGroup;

mod internal_link_type_business_chat;
pub use internal_link_type_business_chat::InternalLinkTypeBusinessChat;

mod internal_link_type_calls_page;
pub use internal_link_type_calls_page::InternalLinkTypeCallsPage;

mod internal_link_type_chat_affiliate_program;
pub use internal_link_type_chat_affiliate_program::InternalLinkTypeChatAffiliateProgram;

mod internal_link_type_chat_boost;
pub use internal_link_type_chat_boost::InternalLinkTypeChatBoost;

mod internal_link_type_chat_folder_invite;
pub use internal_link_type_chat_folder_invite::InternalLinkTypeChatFolderInvite;

mod internal_link_type_chat_invite;
pub use internal_link_type_chat_invite::InternalLinkTypeChatInvite;

mod internal_link_type_contacts_page;
pub use internal_link_type_contacts_page::InternalLinkTypeContactsPage;

mod internal_link_type_direct_messages_chat;
pub use internal_link_type_direct_messages_chat::InternalLinkTypeDirectMessagesChat;

mod internal_link_type_game;
pub use internal_link_type_game::InternalLinkTypeGame;

mod internal_link_type_gift_auction;
pub use internal_link_type_gift_auction::InternalLinkTypeGiftAuction;

mod internal_link_type_gift_collection;
pub use internal_link_type_gift_collection::InternalLinkTypeGiftCollection;

mod internal_link_type_group_call;
pub use internal_link_type_group_call::InternalLinkTypeGroupCall;

mod internal_link_type_instant_view;
pub use internal_link_type_instant_view::InternalLinkTypeInstantView;

mod internal_link_type_invoice;
pub use internal_link_type_invoice::InternalLinkTypeInvoice;

mod internal_link_type_language_pack;
pub use internal_link_type_language_pack::InternalLinkTypeLanguagePack;

mod internal_link_type_live_story;
pub use internal_link_type_live_story::InternalLinkTypeLiveStory;

mod internal_link_type_main_web_app;
pub use internal_link_type_main_web_app::InternalLinkTypeMainWebApp;

mod internal_link_type_message;
pub use internal_link_type_message::InternalLinkTypeMessage;

mod internal_link_type_message_draft;
pub use internal_link_type_message_draft::InternalLinkTypeMessageDraft;

mod internal_link_type_my_profile_page;
pub use internal_link_type_my_profile_page::InternalLinkTypeMyProfilePage;

mod internal_link_type_new_story;
pub use internal_link_type_new_story::InternalLinkTypeNewStory;

mod internal_link_type_oauth;
pub use internal_link_type_oauth::InternalLinkTypeOauth;

mod internal_link_type_passport_data_request;
pub use internal_link_type_passport_data_request::InternalLinkTypePassportDataRequest;

mod internal_link_type_phone_number_confirmation;
pub use internal_link_type_phone_number_confirmation::InternalLinkTypePhoneNumberConfirmation;

mod internal_link_type_premium_features_page;
pub use internal_link_type_premium_features_page::InternalLinkTypePremiumFeaturesPage;

mod internal_link_type_premium_gift_code;
pub use internal_link_type_premium_gift_code::InternalLinkTypePremiumGiftCode;

mod internal_link_type_premium_gift_purchase;
pub use internal_link_type_premium_gift_purchase::InternalLinkTypePremiumGiftPurchase;

mod internal_link_type_proxy;
pub use internal_link_type_proxy::InternalLinkTypeProxy;

mod internal_link_type_public_chat;
pub use internal_link_type_public_chat::InternalLinkTypePublicChat;

mod internal_link_type_settings;
pub use internal_link_type_settings::InternalLinkTypeSettings;

mod internal_link_type_star_purchase;
pub use internal_link_type_star_purchase::InternalLinkTypeStarPurchase;

mod internal_link_type_sticker_set;
pub use internal_link_type_sticker_set::InternalLinkTypeStickerSet;

mod internal_link_type_story;
pub use internal_link_type_story::InternalLinkTypeStory;

mod internal_link_type_story_album;
pub use internal_link_type_story_album::InternalLinkTypeStoryAlbum;

mod internal_link_type_theme;
pub use internal_link_type_theme::InternalLinkTypeTheme;

mod internal_link_type_unknown_deep_link;
pub use internal_link_type_unknown_deep_link::InternalLinkTypeUnknownDeepLink;

mod internal_link_type_upgraded_gift;
pub use internal_link_type_upgraded_gift::InternalLinkTypeUpgradedGift;

mod internal_link_type_user_phone_number;
pub use internal_link_type_user_phone_number::InternalLinkTypeUserPhoneNumber;

mod internal_link_type_user_token;
pub use internal_link_type_user_token::InternalLinkTypeUserToken;

mod internal_link_type_video_chat;
pub use internal_link_type_video_chat::InternalLinkTypeVideoChat;

mod internal_link_type_web_app;
pub use internal_link_type_web_app::InternalLinkTypeWebApp;

mod message_link;
pub use message_link::MessageLink;

mod message_link_info;
pub use message_link_info::MessageLinkInfo;

mod chat_boost_link;
pub use chat_boost_link::ChatBoostLink;

mod chat_boost_link_info;
pub use chat_boost_link_info::ChatBoostLinkInfo;

mod storage_statistics_by_file_type;
pub use storage_statistics_by_file_type::StorageStatisticsByFileType;

mod storage_statistics_by_chat;
pub use storage_statistics_by_chat::StorageStatisticsByChat;

mod storage_statistics;
pub use storage_statistics::StorageStatistics;

mod storage_statistics_fast;
pub use storage_statistics_fast::StorageStatisticsFast;

mod database_statistics;
pub use database_statistics::DatabaseStatistics;

mod network_statistics_entry_file;
pub use network_statistics_entry_file::NetworkStatisticsEntryFile;

mod network_statistics_entry_call;
pub use network_statistics_entry_call::NetworkStatisticsEntryCall;

mod network_statistics;
pub use network_statistics::NetworkStatistics;

mod auto_download_settings;
pub use auto_download_settings::AutoDownloadSettings;

mod auto_download_settings_presets;
pub use auto_download_settings_presets::AutoDownloadSettingsPresets;

mod autosave_settings_scope_chat;
pub use autosave_settings_scope_chat::AutosaveSettingsScopeChat;

mod scope_autosave_settings;
pub use scope_autosave_settings::ScopeAutosaveSettings;

mod autosave_settings_exception;
pub use autosave_settings_exception::AutosaveSettingsException;

mod autosave_settings;
pub use autosave_settings::AutosaveSettings;

mod age_verification_parameters;
pub use age_verification_parameters::AgeVerificationParameters;

mod found_position;
pub use found_position::FoundPosition;

mod found_positions;
pub use found_positions::FoundPositions;

mod tme_url_type_user;
pub use tme_url_type_user::TmeUrlTypeUser;

mod tme_url_type_supergroup;
pub use tme_url_type_supergroup::TmeUrlTypeSupergroup;

mod tme_url_type_chat_invite;
pub use tme_url_type_chat_invite::TmeUrlTypeChatInvite;

mod tme_url_type_sticker_set;
pub use tme_url_type_sticker_set::TmeUrlTypeStickerSet;

mod tme_url;
pub use tme_url::TmeUrl;

mod tme_urls;
pub use tme_urls::TmeUrls;

mod suggested_action_convert_to_broadcast_group;
pub use suggested_action_convert_to_broadcast_group::SuggestedActionConvertToBroadcastGroup;

mod suggested_action_set_password;
pub use suggested_action_set_password::SuggestedActionSetPassword;

mod suggested_action_extend_premium;
pub use suggested_action_extend_premium::SuggestedActionExtendPremium;

mod suggested_action_custom;
pub use suggested_action_custom::SuggestedActionCustom;

mod suggested_action_set_login_email_address;
pub use suggested_action_set_login_email_address::SuggestedActionSetLoginEmailAddress;

mod count;
pub use count::Count;

mod text;
pub use text::Text;

mod data;
pub use data::Data;

mod seconds;
pub use seconds::Seconds;

mod file_downloaded_prefix_size;
pub use file_downloaded_prefix_size::FileDownloadedPrefixSize;

mod star_count;
pub use star_count::StarCount;

mod deep_link_info;
pub use deep_link_info::DeepLinkInfo;

mod text_parse_mode_markdown;
pub use text_parse_mode_markdown::TextParseModeMarkdown;

mod proxy_type_socks_5;
pub use proxy_type_socks_5::ProxyTypeSocks5;

mod proxy_type_http;
pub use proxy_type_http::ProxyTypeHttp;

mod proxy_type_mtproto;
pub use proxy_type_mtproto::ProxyTypeMtproto;

mod added_proxy;
pub use added_proxy::AddedProxy;

mod added_proxies;
pub use added_proxies::AddedProxies;

mod input_sticker;
pub use input_sticker::InputSticker;

mod date_range;
pub use date_range::DateRange;

mod statistical_value;
pub use statistical_value::StatisticalValue;

mod statistical_graph_data;
pub use statistical_graph_data::StatisticalGraphData;

mod statistical_graph_async;
pub use statistical_graph_async::StatisticalGraphAsync;

mod statistical_graph_error;
pub use statistical_graph_error::StatisticalGraphError;

mod chat_statistics_object_type_message;
pub use chat_statistics_object_type_message::ChatStatisticsObjectTypeMessage;

mod chat_statistics_object_type_story;
pub use chat_statistics_object_type_story::ChatStatisticsObjectTypeStory;

mod chat_statistics_interaction_info;
pub use chat_statistics_interaction_info::ChatStatisticsInteractionInfo;

mod chat_statistics_message_sender_info;
pub use chat_statistics_message_sender_info::ChatStatisticsMessageSenderInfo;

mod chat_statistics_administrator_actions_info;
pub use chat_statistics_administrator_actions_info::ChatStatisticsAdministratorActionsInfo;

mod chat_statistics_inviter_info;
pub use chat_statistics_inviter_info::ChatStatisticsInviterInfo;

mod chat_statistics_supergroup;
pub use chat_statistics_supergroup::ChatStatisticsSupergroup;

mod chat_statistics_channel;
pub use chat_statistics_channel::ChatStatisticsChannel;

mod chat_revenue_amount;
pub use chat_revenue_amount::ChatRevenueAmount;

mod chat_revenue_statistics;
pub use chat_revenue_statistics::ChatRevenueStatistics;

mod message_statistics;
pub use message_statistics::MessageStatistics;

mod story_statistics;
pub use story_statistics::StoryStatistics;

mod revenue_withdrawal_state_succeeded;
pub use revenue_withdrawal_state_succeeded::RevenueWithdrawalStateSucceeded;

mod chat_revenue_transaction_type_sponsored_message_earnings;
pub use chat_revenue_transaction_type_sponsored_message_earnings::ChatRevenueTransactionTypeSponsoredMessageEarnings;

mod chat_revenue_transaction_type_suggested_post_earnings;
pub use chat_revenue_transaction_type_suggested_post_earnings::ChatRevenueTransactionTypeSuggestedPostEarnings;

mod chat_revenue_transaction_type_fragment_withdrawal;
pub use chat_revenue_transaction_type_fragment_withdrawal::ChatRevenueTransactionTypeFragmentWithdrawal;

mod chat_revenue_transaction_type_fragment_refund;
pub use chat_revenue_transaction_type_fragment_refund::ChatRevenueTransactionTypeFragmentRefund;

mod chat_revenue_transaction;
pub use chat_revenue_transaction::ChatRevenueTransaction;

mod chat_revenue_transactions;
pub use chat_revenue_transactions::ChatRevenueTransactions;

mod star_revenue_status;
pub use star_revenue_status::StarRevenueStatus;

mod star_revenue_statistics;
pub use star_revenue_statistics::StarRevenueStatistics;

mod ton_revenue_status;
pub use ton_revenue_status::TonRevenueStatus;

mod ton_revenue_statistics;
pub use ton_revenue_statistics::TonRevenueStatistics;

mod point;
pub use point::Point;

mod vector_path_command_line;
pub use vector_path_command_line::VectorPathCommandLine;

mod vector_path_command_cubic_bezier_curve;
pub use vector_path_command_cubic_bezier_curve::VectorPathCommandCubicBezierCurve;

mod bot_command_scope_chat;
pub use bot_command_scope_chat::BotCommandScopeChat;

mod bot_command_scope_chat_administrators;
pub use bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;

mod bot_command_scope_chat_member;
pub use bot_command_scope_chat_member::BotCommandScopeChatMember;

mod phone_number_code_type_confirm_ownership;
pub use phone_number_code_type_confirm_ownership::PhoneNumberCodeTypeConfirmOwnership;

mod update_authorization_state;
pub use update_authorization_state::UpdateAuthorizationState;

mod update_new_message;
pub use update_new_message::UpdateNewMessage;

mod update_message_send_acknowledged;
pub use update_message_send_acknowledged::UpdateMessageSendAcknowledged;

mod update_message_send_succeeded;
pub use update_message_send_succeeded::UpdateMessageSendSucceeded;

mod update_message_send_failed;
pub use update_message_send_failed::UpdateMessageSendFailed;

mod update_message_content;
pub use update_message_content::UpdateMessageContent;

mod update_message_edited;
pub use update_message_edited::UpdateMessageEdited;

mod update_message_is_pinned;
pub use update_message_is_pinned::UpdateMessageIsPinned;

mod update_message_interaction_info;
pub use update_message_interaction_info::UpdateMessageInteractionInfo;

mod update_message_content_opened;
pub use update_message_content_opened::UpdateMessageContentOpened;

mod update_message_mention_read;
pub use update_message_mention_read::UpdateMessageMentionRead;

mod update_message_unread_reactions;
pub use update_message_unread_reactions::UpdateMessageUnreadReactions;

mod update_message_fact_check;
pub use update_message_fact_check::UpdateMessageFactCheck;

mod update_message_suggested_post_info;
pub use update_message_suggested_post_info::UpdateMessageSuggestedPostInfo;

mod update_message_live_location_viewed;
pub use update_message_live_location_viewed::UpdateMessageLiveLocationViewed;

mod update_video_published;
pub use update_video_published::UpdateVideoPublished;

mod update_new_chat;
pub use update_new_chat::UpdateNewChat;

mod update_chat_title;
pub use update_chat_title::UpdateChatTitle;

mod update_chat_photo;
pub use update_chat_photo::UpdateChatPhoto;

mod update_chat_accent_colors;
pub use update_chat_accent_colors::UpdateChatAccentColors;

mod update_chat_permissions;
pub use update_chat_permissions::UpdateChatPermissions;

mod update_chat_last_message;
pub use update_chat_last_message::UpdateChatLastMessage;

mod update_chat_position;
pub use update_chat_position::UpdateChatPosition;

mod update_chat_added_to_list;
pub use update_chat_added_to_list::UpdateChatAddedToList;

mod update_chat_removed_from_list;
pub use update_chat_removed_from_list::UpdateChatRemovedFromList;

mod update_chat_read_inbox;
pub use update_chat_read_inbox::UpdateChatReadInbox;

mod update_chat_read_outbox;
pub use update_chat_read_outbox::UpdateChatReadOutbox;

mod update_chat_action_bar;
pub use update_chat_action_bar::UpdateChatActionBar;

mod update_chat_business_bot_manage_bar;
pub use update_chat_business_bot_manage_bar::UpdateChatBusinessBotManageBar;

mod update_chat_available_reactions;
pub use update_chat_available_reactions::UpdateChatAvailableReactions;

mod update_chat_draft_message;
pub use update_chat_draft_message::UpdateChatDraftMessage;

mod update_chat_emoji_status;
pub use update_chat_emoji_status::UpdateChatEmojiStatus;

mod update_chat_message_sender;
pub use update_chat_message_sender::UpdateChatMessageSender;

mod update_chat_message_auto_delete_time;
pub use update_chat_message_auto_delete_time::UpdateChatMessageAutoDeleteTime;

mod update_chat_notification_settings;
pub use update_chat_notification_settings::UpdateChatNotificationSettings;

mod update_chat_pending_join_requests;
pub use update_chat_pending_join_requests::UpdateChatPendingJoinRequests;

mod update_chat_reply_markup;
pub use update_chat_reply_markup::UpdateChatReplyMarkup;

mod update_chat_background;
pub use update_chat_background::UpdateChatBackground;

mod update_chat_theme;
pub use update_chat_theme::UpdateChatTheme;

mod update_chat_unread_mention_count;
pub use update_chat_unread_mention_count::UpdateChatUnreadMentionCount;

mod update_chat_unread_reaction_count;
pub use update_chat_unread_reaction_count::UpdateChatUnreadReactionCount;

mod update_chat_video_chat;
pub use update_chat_video_chat::UpdateChatVideoChat;

mod update_chat_default_disable_notification;
pub use update_chat_default_disable_notification::UpdateChatDefaultDisableNotification;

mod update_chat_has_protected_content;
pub use update_chat_has_protected_content::UpdateChatHasProtectedContent;

mod update_chat_is_translatable;
pub use update_chat_is_translatable::UpdateChatIsTranslatable;

mod update_chat_is_marked_as_unread;
pub use update_chat_is_marked_as_unread::UpdateChatIsMarkedAsUnread;

mod update_chat_view_as_topics;
pub use update_chat_view_as_topics::UpdateChatViewAsTopics;

mod update_chat_block_list;
pub use update_chat_block_list::UpdateChatBlockList;

mod update_chat_has_scheduled_messages;
pub use update_chat_has_scheduled_messages::UpdateChatHasScheduledMessages;

mod update_chat_folders;
pub use update_chat_folders::UpdateChatFolders;

mod update_chat_online_member_count;
pub use update_chat_online_member_count::UpdateChatOnlineMemberCount;

mod update_saved_messages_topic;
pub use update_saved_messages_topic::UpdateSavedMessagesTopic;

mod update_saved_messages_topic_count;
pub use update_saved_messages_topic_count::UpdateSavedMessagesTopicCount;

mod update_direct_messages_chat_topic;
pub use update_direct_messages_chat_topic::UpdateDirectMessagesChatTopic;

mod update_topic_message_count;
pub use update_topic_message_count::UpdateTopicMessageCount;

mod update_quick_reply_shortcut;
pub use update_quick_reply_shortcut::UpdateQuickReplyShortcut;

mod update_quick_reply_shortcut_deleted;
pub use update_quick_reply_shortcut_deleted::UpdateQuickReplyShortcutDeleted;

mod update_quick_reply_shortcuts;
pub use update_quick_reply_shortcuts::UpdateQuickReplyShortcuts;

mod update_quick_reply_shortcut_messages;
pub use update_quick_reply_shortcut_messages::UpdateQuickReplyShortcutMessages;

mod update_forum_topic_info;
pub use update_forum_topic_info::UpdateForumTopicInfo;

mod update_forum_topic;
pub use update_forum_topic::UpdateForumTopic;

mod update_scope_notification_settings;
pub use update_scope_notification_settings::UpdateScopeNotificationSettings;

mod update_reaction_notification_settings;
pub use update_reaction_notification_settings::UpdateReactionNotificationSettings;

mod update_notification;
pub use update_notification::UpdateNotification;

mod update_notification_group;
pub use update_notification_group::UpdateNotificationGroup;

mod update_active_notifications;
pub use update_active_notifications::UpdateActiveNotifications;

mod update_have_pending_notifications;
pub use update_have_pending_notifications::UpdateHavePendingNotifications;

mod update_delete_messages;
pub use update_delete_messages::UpdateDeleteMessages;

mod update_chat_action;
pub use update_chat_action::UpdateChatAction;

mod update_pending_text_message;
pub use update_pending_text_message::UpdatePendingTextMessage;

mod update_user_status;
pub use update_user_status::UpdateUserStatus;

mod update_user;
pub use update_user::UpdateUser;

mod update_basic_group;
pub use update_basic_group::UpdateBasicGroup;

mod update_supergroup;
pub use update_supergroup::UpdateSupergroup;

mod update_secret_chat;
pub use update_secret_chat::UpdateSecretChat;

mod update_user_full_info;
pub use update_user_full_info::UpdateUserFullInfo;

mod update_basic_group_full_info;
pub use update_basic_group_full_info::UpdateBasicGroupFullInfo;

mod update_supergroup_full_info;
pub use update_supergroup_full_info::UpdateSupergroupFullInfo;

mod update_service_notification;
pub use update_service_notification::UpdateServiceNotification;

mod update_new_oauth_request;
pub use update_new_oauth_request::UpdateNewOauthRequest;

mod update_file;
pub use update_file::UpdateFile;

mod update_file_generation_start;
pub use update_file_generation_start::UpdateFileGenerationStart;

mod update_file_generation_stop;
pub use update_file_generation_stop::UpdateFileGenerationStop;

mod update_file_downloads;
pub use update_file_downloads::UpdateFileDownloads;

mod update_file_added_to_downloads;
pub use update_file_added_to_downloads::UpdateFileAddedToDownloads;

mod update_file_download;
pub use update_file_download::UpdateFileDownload;

mod update_file_removed_from_downloads;
pub use update_file_removed_from_downloads::UpdateFileRemovedFromDownloads;

mod update_application_verification_required;
pub use update_application_verification_required::UpdateApplicationVerificationRequired;

mod update_application_recaptcha_verification_required;
pub use update_application_recaptcha_verification_required::UpdateApplicationRecaptchaVerificationRequired;

mod update_call;
pub use update_call::UpdateCall;

mod update_group_call;
pub use update_group_call::UpdateGroupCall;

mod update_group_call_participant;
pub use update_group_call_participant::UpdateGroupCallParticipant;

mod update_group_call_participants;
pub use update_group_call_participants::UpdateGroupCallParticipants;

mod update_group_call_verification_state;
pub use update_group_call_verification_state::UpdateGroupCallVerificationState;

mod update_new_group_call_message;
pub use update_new_group_call_message::UpdateNewGroupCallMessage;

mod update_new_group_call_paid_reaction;
pub use update_new_group_call_paid_reaction::UpdateNewGroupCallPaidReaction;

mod update_group_call_message_send_failed;
pub use update_group_call_message_send_failed::UpdateGroupCallMessageSendFailed;

mod update_group_call_messages_deleted;
pub use update_group_call_messages_deleted::UpdateGroupCallMessagesDeleted;

mod update_live_story_top_donors;
pub use update_live_story_top_donors::UpdateLiveStoryTopDonors;

mod update_new_call_signaling_data;
pub use update_new_call_signaling_data::UpdateNewCallSignalingData;

mod update_gift_auction_state;
pub use update_gift_auction_state::UpdateGiftAuctionState;

mod update_active_gift_auctions;
pub use update_active_gift_auctions::UpdateActiveGiftAuctions;

mod update_user_privacy_setting_rules;
pub use update_user_privacy_setting_rules::UpdateUserPrivacySettingRules;

mod update_unread_message_count;
pub use update_unread_message_count::UpdateUnreadMessageCount;

mod update_unread_chat_count;
pub use update_unread_chat_count::UpdateUnreadChatCount;

mod update_story;
pub use update_story::UpdateStory;

mod update_story_deleted;
pub use update_story_deleted::UpdateStoryDeleted;

mod update_story_post_succeeded;
pub use update_story_post_succeeded::UpdateStoryPostSucceeded;

mod update_story_post_failed;
pub use update_story_post_failed::UpdateStoryPostFailed;

mod update_chat_active_stories;
pub use update_chat_active_stories::UpdateChatActiveStories;

mod update_story_list_chat_count;
pub use update_story_list_chat_count::UpdateStoryListChatCount;

mod update_story_stealth_mode;
pub use update_story_stealth_mode::UpdateStoryStealthMode;

mod update_trusted_mini_app_bots;
pub use update_trusted_mini_app_bots::UpdateTrustedMiniAppBots;

mod update_option;
pub use update_option::UpdateOption;

mod update_sticker_set;
pub use update_sticker_set::UpdateStickerSet;

mod update_installed_sticker_sets;
pub use update_installed_sticker_sets::UpdateInstalledStickerSets;

mod update_trending_sticker_sets;
pub use update_trending_sticker_sets::UpdateTrendingStickerSets;

mod update_recent_stickers;
pub use update_recent_stickers::UpdateRecentStickers;

mod update_favorite_stickers;
pub use update_favorite_stickers::UpdateFavoriteStickers;

mod update_saved_animations;
pub use update_saved_animations::UpdateSavedAnimations;

mod update_saved_notification_sounds;
pub use update_saved_notification_sounds::UpdateSavedNotificationSounds;

mod update_default_background;
pub use update_default_background::UpdateDefaultBackground;

mod update_emoji_chat_themes;
pub use update_emoji_chat_themes::UpdateEmojiChatThemes;

mod update_accent_colors;
pub use update_accent_colors::UpdateAccentColors;

mod update_profile_accent_colors;
pub use update_profile_accent_colors::UpdateProfileAccentColors;

mod update_language_pack_strings;
pub use update_language_pack_strings::UpdateLanguagePackStrings;

mod update_connection_state;
pub use update_connection_state::UpdateConnectionState;

mod update_freeze_state;
pub use update_freeze_state::UpdateFreezeState;

mod update_age_verification_parameters;
pub use update_age_verification_parameters::UpdateAgeVerificationParameters;

mod update_terms_of_service;
pub use update_terms_of_service::UpdateTermsOfService;

mod update_unconfirmed_session;
pub use update_unconfirmed_session::UpdateUnconfirmedSession;

mod update_attachment_menu_bots;
pub use update_attachment_menu_bots::UpdateAttachmentMenuBots;

mod update_web_app_message_sent;
pub use update_web_app_message_sent::UpdateWebAppMessageSent;

mod update_active_emoji_reactions;
pub use update_active_emoji_reactions::UpdateActiveEmojiReactions;

mod update_available_message_effects;
pub use update_available_message_effects::UpdateAvailableMessageEffects;

mod update_default_reaction_type;
pub use update_default_reaction_type::UpdateDefaultReactionType;

mod update_default_paid_reaction_type;
pub use update_default_paid_reaction_type::UpdateDefaultPaidReactionType;

mod update_saved_messages_tags;
pub use update_saved_messages_tags::UpdateSavedMessagesTags;

mod update_active_live_location_messages;
pub use update_active_live_location_messages::UpdateActiveLiveLocationMessages;

mod update_owned_star_count;
pub use update_owned_star_count::UpdateOwnedStarCount;

mod update_owned_ton_count;
pub use update_owned_ton_count::UpdateOwnedTonCount;

mod update_chat_revenue_amount;
pub use update_chat_revenue_amount::UpdateChatRevenueAmount;

mod update_star_revenue_status;
pub use update_star_revenue_status::UpdateStarRevenueStatus;

mod update_ton_revenue_status;
pub use update_ton_revenue_status::UpdateTonRevenueStatus;

mod update_speech_recognition_trial;
pub use update_speech_recognition_trial::UpdateSpeechRecognitionTrial;

mod update_group_call_message_levels;
pub use update_group_call_message_levels::UpdateGroupCallMessageLevels;

mod update_dice_emojis;
pub use update_dice_emojis::UpdateDiceEmojis;

mod update_stake_dice_state;
pub use update_stake_dice_state::UpdateStakeDiceState;

mod update_animated_emoji_message_clicked;
pub use update_animated_emoji_message_clicked::UpdateAnimatedEmojiMessageClicked;

mod update_animation_search_parameters;
pub use update_animation_search_parameters::UpdateAnimationSearchParameters;

mod update_suggested_actions;
pub use update_suggested_actions::UpdateSuggestedActions;

mod update_speed_limit_notification;
pub use update_speed_limit_notification::UpdateSpeedLimitNotification;

mod update_contact_close_birthdays;
pub use update_contact_close_birthdays::UpdateContactCloseBirthdays;

mod update_autosave_settings;
pub use update_autosave_settings::UpdateAutosaveSettings;

mod update_business_connection;
pub use update_business_connection::UpdateBusinessConnection;

mod update_new_business_message;
pub use update_new_business_message::UpdateNewBusinessMessage;

mod update_business_message_edited;
pub use update_business_message_edited::UpdateBusinessMessageEdited;

mod update_business_messages_deleted;
pub use update_business_messages_deleted::UpdateBusinessMessagesDeleted;

mod update_new_inline_query;
pub use update_new_inline_query::UpdateNewInlineQuery;

mod update_new_chosen_inline_result;
pub use update_new_chosen_inline_result::UpdateNewChosenInlineResult;

mod update_new_callback_query;
pub use update_new_callback_query::UpdateNewCallbackQuery;

mod update_new_inline_callback_query;
pub use update_new_inline_callback_query::UpdateNewInlineCallbackQuery;

mod update_new_business_callback_query;
pub use update_new_business_callback_query::UpdateNewBusinessCallbackQuery;

mod update_new_shipping_query;
pub use update_new_shipping_query::UpdateNewShippingQuery;

mod update_new_pre_checkout_query;
pub use update_new_pre_checkout_query::UpdateNewPreCheckoutQuery;

mod update_new_custom_event;
pub use update_new_custom_event::UpdateNewCustomEvent;

mod update_new_custom_query;
pub use update_new_custom_query::UpdateNewCustomQuery;

mod update_poll;
pub use update_poll::UpdatePoll;

mod update_poll_answer;
pub use update_poll_answer::UpdatePollAnswer;

mod update_chat_member;
pub use update_chat_member::UpdateChatMember;

mod update_new_chat_join_request;
pub use update_new_chat_join_request::UpdateNewChatJoinRequest;

mod update_chat_boost;
pub use update_chat_boost::UpdateChatBoost;

mod update_message_reaction;
pub use update_message_reaction::UpdateMessageReaction;

mod update_message_reactions;
pub use update_message_reactions::UpdateMessageReactions;

mod update_paid_media_purchased;
pub use update_paid_media_purchased::UpdatePaidMediaPurchased;

mod updates;
pub use updates::Updates;

mod log_stream_file;
pub use log_stream_file::LogStreamFile;

mod log_verbosity_level;
pub use log_verbosity_level::LogVerbosityLevel;

mod log_tags;
pub use log_tags::LogTags;

mod user_support_info;
pub use user_support_info::UserSupportInfo;

mod test_int;
pub use test_int::TestInt;

mod test_string;
pub use test_string::TestString;

mod test_bytes;
pub use test_bytes::TestBytes;

mod test_vector_int;
pub use test_vector_int::TestVectorInt;

mod test_vector_int_object;
pub use test_vector_int_object::TestVectorIntObject;

mod test_vector_string;
pub use test_vector_string::TestVectorString;

mod test_vector_string_object;
pub use test_vector_string_object::TestVectorStringObject;
