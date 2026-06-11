mod get_authorization_state;
pub use get_authorization_state::get_authorization_state;

mod set_tdlib_parameters;
pub use set_tdlib_parameters::set_tdlib_parameters;

mod set_authentication_phone_number;
pub use set_authentication_phone_number::set_authentication_phone_number;

mod check_authentication_premium_purchase;
pub use check_authentication_premium_purchase::check_authentication_premium_purchase;

mod set_authentication_premium_purchase_transaction;
pub use set_authentication_premium_purchase_transaction::set_authentication_premium_purchase_transaction;

mod set_authentication_email_address;
pub use set_authentication_email_address::set_authentication_email_address;

mod resend_authentication_code;
pub use resend_authentication_code::resend_authentication_code;

mod check_authentication_email_code;
pub use check_authentication_email_code::check_authentication_email_code;

mod check_authentication_code;
pub use check_authentication_code::check_authentication_code;

mod request_qr_code_authentication;
pub use request_qr_code_authentication::request_qr_code_authentication;

mod get_authentication_passkey_parameters;
pub use get_authentication_passkey_parameters::get_authentication_passkey_parameters;

mod check_authentication_passkey;
pub use check_authentication_passkey::check_authentication_passkey;

mod register_user;
pub use register_user::register_user;

mod reset_authentication_email_address;
pub use reset_authentication_email_address::reset_authentication_email_address;

mod check_authentication_password;
pub use check_authentication_password::check_authentication_password;

mod request_authentication_password_recovery;
pub use request_authentication_password_recovery::request_authentication_password_recovery;

mod check_authentication_password_recovery_code;
pub use check_authentication_password_recovery_code::check_authentication_password_recovery_code;

mod recover_authentication_password;
pub use recover_authentication_password::recover_authentication_password;

mod send_authentication_firebase_sms;
pub use send_authentication_firebase_sms::send_authentication_firebase_sms;

mod report_authentication_code_missing;
pub use report_authentication_code_missing::report_authentication_code_missing;

mod check_authentication_bot_token;
pub use check_authentication_bot_token::check_authentication_bot_token;

mod log_out;
pub use log_out::log_out;

mod close;
pub use close::close;

mod destroy;
pub use destroy::destroy;

mod confirm_qr_code_authentication;
pub use confirm_qr_code_authentication::confirm_qr_code_authentication;

mod get_current_state;
pub use get_current_state::get_current_state;

mod set_database_encryption_key;
pub use set_database_encryption_key::set_database_encryption_key;

mod get_password_state;
pub use get_password_state::get_password_state;

mod set_password;
pub use set_password::set_password;

mod is_login_email_address_required;
pub use is_login_email_address_required::is_login_email_address_required;

mod set_login_email_address;
pub use set_login_email_address::set_login_email_address;

mod resend_login_email_address_code;
pub use resend_login_email_address_code::resend_login_email_address_code;

mod check_login_email_address_code;
pub use check_login_email_address_code::check_login_email_address_code;

mod get_recovery_email_address;
pub use get_recovery_email_address::get_recovery_email_address;

mod set_recovery_email_address;
pub use set_recovery_email_address::set_recovery_email_address;

mod check_recovery_email_address_code;
pub use check_recovery_email_address_code::check_recovery_email_address_code;

mod resend_recovery_email_address_code;
pub use resend_recovery_email_address_code::resend_recovery_email_address_code;

mod cancel_recovery_email_address_verification;
pub use cancel_recovery_email_address_verification::cancel_recovery_email_address_verification;

mod request_password_recovery;
pub use request_password_recovery::request_password_recovery;

mod check_password_recovery_code;
pub use check_password_recovery_code::check_password_recovery_code;

mod recover_password;
pub use recover_password::recover_password;

mod reset_password;
pub use reset_password::reset_password;

mod cancel_password_reset;
pub use cancel_password_reset::cancel_password_reset;

mod create_temporary_password;
pub use create_temporary_password::create_temporary_password;

mod get_temporary_password_state;
pub use get_temporary_password_state::get_temporary_password_state;

mod get_me;
pub use get_me::get_me;

mod get_user;
pub use get_user::get_user;

mod get_user_full_info;
pub use get_user_full_info::get_user_full_info;

mod get_basic_group;
pub use get_basic_group::get_basic_group;

mod get_basic_group_full_info;
pub use get_basic_group_full_info::get_basic_group_full_info;

mod get_supergroup;
pub use get_supergroup::get_supergroup;

mod get_supergroup_full_info;
pub use get_supergroup_full_info::get_supergroup_full_info;

mod get_secret_chat;
pub use get_secret_chat::get_secret_chat;

mod get_chat;
pub use get_chat::get_chat;

mod get_message;
pub use get_message::get_message;

mod get_message_locally;
pub use get_message_locally::get_message_locally;

mod get_replied_message;
pub use get_replied_message::get_replied_message;

mod get_chat_pinned_message;
pub use get_chat_pinned_message::get_chat_pinned_message;

mod get_callback_query_message;
pub use get_callback_query_message::get_callback_query_message;

mod get_messages;
pub use get_messages::get_messages;

mod get_message_properties;
pub use get_message_properties::get_message_properties;

mod get_message_thread;
pub use get_message_thread::get_message_thread;

mod get_message_read_date;
pub use get_message_read_date::get_message_read_date;

mod get_message_viewers;
pub use get_message_viewers::get_message_viewers;

mod get_message_author;
pub use get_message_author::get_message_author;

mod get_file;
pub use get_file::get_file;

mod get_remote_file;
pub use get_remote_file::get_remote_file;

mod load_chats;
pub use load_chats::load_chats;

mod get_chats;
pub use get_chats::get_chats;

mod search_public_chat;
pub use search_public_chat::search_public_chat;

mod search_public_chats;
pub use search_public_chats::search_public_chats;

mod search_chats;
pub use search_chats::search_chats;

mod search_chats_on_server;
pub use search_chats_on_server::search_chats_on_server;

mod get_recommended_chats;
pub use get_recommended_chats::get_recommended_chats;

mod get_chat_similar_chats;
pub use get_chat_similar_chats::get_chat_similar_chats;

mod get_chat_similar_chat_count;
pub use get_chat_similar_chat_count::get_chat_similar_chat_count;

mod open_chat_similar_chat;
pub use open_chat_similar_chat::open_chat_similar_chat;

mod get_bot_similar_bots;
pub use get_bot_similar_bots::get_bot_similar_bots;

mod get_bot_similar_bot_count;
pub use get_bot_similar_bot_count::get_bot_similar_bot_count;

mod open_bot_similar_bot;
pub use open_bot_similar_bot::open_bot_similar_bot;

mod get_top_chats;
pub use get_top_chats::get_top_chats;

mod remove_top_chat;
pub use remove_top_chat::remove_top_chat;

mod search_recently_found_chats;
pub use search_recently_found_chats::search_recently_found_chats;

mod add_recently_found_chat;
pub use add_recently_found_chat::add_recently_found_chat;

mod remove_recently_found_chat;
pub use remove_recently_found_chat::remove_recently_found_chat;

mod clear_recently_found_chats;
pub use clear_recently_found_chats::clear_recently_found_chats;

mod get_recently_opened_chats;
pub use get_recently_opened_chats::get_recently_opened_chats;

mod check_chat_username;
pub use check_chat_username::check_chat_username;

mod get_created_public_chats;
pub use get_created_public_chats::get_created_public_chats;

mod check_created_public_chats_limit;
pub use check_created_public_chats_limit::check_created_public_chats_limit;

mod get_suitable_discussion_chats;
pub use get_suitable_discussion_chats::get_suitable_discussion_chats;

mod get_inactive_supergroup_chats;
pub use get_inactive_supergroup_chats::get_inactive_supergroup_chats;

mod get_suitable_personal_chats;
pub use get_suitable_personal_chats::get_suitable_personal_chats;

mod load_direct_messages_chat_topics;
pub use load_direct_messages_chat_topics::load_direct_messages_chat_topics;

mod get_direct_messages_chat_topic;
pub use get_direct_messages_chat_topic::get_direct_messages_chat_topic;

mod get_direct_messages_chat_topic_history;
pub use get_direct_messages_chat_topic_history::get_direct_messages_chat_topic_history;

mod get_direct_messages_chat_topic_message_by_date;
pub use get_direct_messages_chat_topic_message_by_date::get_direct_messages_chat_topic_message_by_date;

mod delete_direct_messages_chat_topic_history;
pub use delete_direct_messages_chat_topic_history::delete_direct_messages_chat_topic_history;

mod delete_direct_messages_chat_topic_messages_by_date;
pub use delete_direct_messages_chat_topic_messages_by_date::delete_direct_messages_chat_topic_messages_by_date;

mod set_direct_messages_chat_topic_is_marked_as_unread;
pub use set_direct_messages_chat_topic_is_marked_as_unread::set_direct_messages_chat_topic_is_marked_as_unread;

mod unpin_all_direct_messages_chat_topic_messages;
pub use unpin_all_direct_messages_chat_topic_messages::unpin_all_direct_messages_chat_topic_messages;

mod read_all_direct_messages_chat_topic_reactions;
pub use read_all_direct_messages_chat_topic_reactions::read_all_direct_messages_chat_topic_reactions;

mod get_direct_messages_chat_topic_revenue;
pub use get_direct_messages_chat_topic_revenue::get_direct_messages_chat_topic_revenue;

mod toggle_direct_messages_chat_topic_can_send_unpaid_messages;
pub use toggle_direct_messages_chat_topic_can_send_unpaid_messages::toggle_direct_messages_chat_topic_can_send_unpaid_messages;

mod load_saved_messages_topics;
pub use load_saved_messages_topics::load_saved_messages_topics;

mod get_saved_messages_topic_history;
pub use get_saved_messages_topic_history::get_saved_messages_topic_history;

mod get_saved_messages_topic_message_by_date;
pub use get_saved_messages_topic_message_by_date::get_saved_messages_topic_message_by_date;

mod delete_saved_messages_topic_history;
pub use delete_saved_messages_topic_history::delete_saved_messages_topic_history;

mod delete_saved_messages_topic_messages_by_date;
pub use delete_saved_messages_topic_messages_by_date::delete_saved_messages_topic_messages_by_date;

mod toggle_saved_messages_topic_is_pinned;
pub use toggle_saved_messages_topic_is_pinned::toggle_saved_messages_topic_is_pinned;

mod set_pinned_saved_messages_topics;
pub use set_pinned_saved_messages_topics::set_pinned_saved_messages_topics;

mod get_groups_in_common;
pub use get_groups_in_common::get_groups_in_common;

mod get_chat_history;
pub use get_chat_history::get_chat_history;

mod get_message_thread_history;
pub use get_message_thread_history::get_message_thread_history;

mod delete_chat_history;
pub use delete_chat_history::delete_chat_history;

mod delete_chat;
pub use delete_chat::delete_chat;

mod search_chat_messages;
pub use search_chat_messages::search_chat_messages;

mod search_messages;
pub use search_messages::search_messages;

mod search_secret_messages;
pub use search_secret_messages::search_secret_messages;

mod search_saved_messages;
pub use search_saved_messages::search_saved_messages;

mod search_call_messages;
pub use search_call_messages::search_call_messages;

mod search_outgoing_document_messages;
pub use search_outgoing_document_messages::search_outgoing_document_messages;

mod get_public_post_search_limits;
pub use get_public_post_search_limits::get_public_post_search_limits;

mod search_public_posts;
pub use search_public_posts::search_public_posts;

mod search_public_messages_by_tag;
pub use search_public_messages_by_tag::search_public_messages_by_tag;

mod search_public_stories_by_tag;
pub use search_public_stories_by_tag::search_public_stories_by_tag;

mod search_public_stories_by_location;
pub use search_public_stories_by_location::search_public_stories_by_location;

mod search_public_stories_by_venue;
pub use search_public_stories_by_venue::search_public_stories_by_venue;

mod get_searched_for_tags;
pub use get_searched_for_tags::get_searched_for_tags;

mod remove_searched_for_tag;
pub use remove_searched_for_tag::remove_searched_for_tag;

mod clear_searched_for_tags;
pub use clear_searched_for_tags::clear_searched_for_tags;

mod delete_all_call_messages;
pub use delete_all_call_messages::delete_all_call_messages;

mod search_chat_recent_location_messages;
pub use search_chat_recent_location_messages::search_chat_recent_location_messages;

mod get_chat_message_by_date;
pub use get_chat_message_by_date::get_chat_message_by_date;

mod get_chat_sparse_message_positions;
pub use get_chat_sparse_message_positions::get_chat_sparse_message_positions;

mod get_chat_message_calendar;
pub use get_chat_message_calendar::get_chat_message_calendar;

mod get_chat_message_count;
pub use get_chat_message_count::get_chat_message_count;

mod get_chat_message_position;
pub use get_chat_message_position::get_chat_message_position;

mod get_chat_scheduled_messages;
pub use get_chat_scheduled_messages::get_chat_scheduled_messages;

mod get_chat_sponsored_messages;
pub use get_chat_sponsored_messages::get_chat_sponsored_messages;

mod click_chat_sponsored_message;
pub use click_chat_sponsored_message::click_chat_sponsored_message;

mod report_chat_sponsored_message;
pub use report_chat_sponsored_message::report_chat_sponsored_message;

mod get_search_sponsored_chats;
pub use get_search_sponsored_chats::get_search_sponsored_chats;

mod view_sponsored_chat;
pub use view_sponsored_chat::view_sponsored_chat;

mod open_sponsored_chat;
pub use open_sponsored_chat::open_sponsored_chat;

mod report_sponsored_chat;
pub use report_sponsored_chat::report_sponsored_chat;

mod get_video_message_advertisements;
pub use get_video_message_advertisements::get_video_message_advertisements;

mod view_video_message_advertisement;
pub use view_video_message_advertisement::view_video_message_advertisement;

mod click_video_message_advertisement;
pub use click_video_message_advertisement::click_video_message_advertisement;

mod report_video_message_advertisement;
pub use report_video_message_advertisement::report_video_message_advertisement;

mod remove_notification;
pub use remove_notification::remove_notification;

mod remove_notification_group;
pub use remove_notification_group::remove_notification_group;

mod get_message_link;
pub use get_message_link::get_message_link;

mod get_message_embedding_code;
pub use get_message_embedding_code::get_message_embedding_code;

mod get_message_link_info;
pub use get_message_link_info::get_message_link_info;

mod translate_text;
pub use translate_text::translate_text;

mod translate_message_text;
pub use translate_message_text::translate_message_text;

mod summarize_message;
pub use summarize_message::summarize_message;

mod recognize_speech;
pub use recognize_speech::recognize_speech;

mod rate_speech_recognition;
pub use rate_speech_recognition::rate_speech_recognition;

mod get_chat_available_message_senders;
pub use get_chat_available_message_senders::get_chat_available_message_senders;

mod set_chat_message_sender;
pub use set_chat_message_sender::set_chat_message_sender;

mod send_message;
pub use send_message::send_message;

mod send_message_album;
pub use send_message_album::send_message_album;

mod send_bot_start_message;
pub use send_bot_start_message::send_bot_start_message;

mod send_inline_query_result_message;
pub use send_inline_query_result_message::send_inline_query_result_message;

mod forward_messages;
pub use forward_messages::forward_messages;

mod send_quick_reply_shortcut_messages;
pub use send_quick_reply_shortcut_messages::send_quick_reply_shortcut_messages;

mod resend_messages;
pub use resend_messages::resend_messages;

mod add_local_message;
pub use add_local_message::add_local_message;

mod delete_messages;
pub use delete_messages::delete_messages;

mod delete_chat_messages_by_sender;
pub use delete_chat_messages_by_sender::delete_chat_messages_by_sender;

mod delete_chat_messages_by_date;
pub use delete_chat_messages_by_date::delete_chat_messages_by_date;

mod edit_message_text;
pub use edit_message_text::edit_message_text;

mod edit_message_live_location;
pub use edit_message_live_location::edit_message_live_location;

mod edit_message_checklist;
pub use edit_message_checklist::edit_message_checklist;

mod edit_message_media;
pub use edit_message_media::edit_message_media;

mod edit_message_caption;
pub use edit_message_caption::edit_message_caption;

mod edit_message_reply_markup;
pub use edit_message_reply_markup::edit_message_reply_markup;

mod edit_inline_message_text;
pub use edit_inline_message_text::edit_inline_message_text;

mod edit_inline_message_live_location;
pub use edit_inline_message_live_location::edit_inline_message_live_location;

mod edit_inline_message_media;
pub use edit_inline_message_media::edit_inline_message_media;

mod edit_inline_message_caption;
pub use edit_inline_message_caption::edit_inline_message_caption;

mod edit_inline_message_reply_markup;
pub use edit_inline_message_reply_markup::edit_inline_message_reply_markup;

mod edit_message_scheduling_state;
pub use edit_message_scheduling_state::edit_message_scheduling_state;

mod set_message_fact_check;
pub use set_message_fact_check::set_message_fact_check;

mod send_business_message;
pub use send_business_message::send_business_message;

mod send_business_message_album;
pub use send_business_message_album::send_business_message_album;

mod edit_business_message_text;
pub use edit_business_message_text::edit_business_message_text;

mod edit_business_message_live_location;
pub use edit_business_message_live_location::edit_business_message_live_location;

mod edit_business_message_checklist;
pub use edit_business_message_checklist::edit_business_message_checklist;

mod edit_business_message_media;
pub use edit_business_message_media::edit_business_message_media;

mod edit_business_message_caption;
pub use edit_business_message_caption::edit_business_message_caption;

mod edit_business_message_reply_markup;
pub use edit_business_message_reply_markup::edit_business_message_reply_markup;

mod stop_business_poll;
pub use stop_business_poll::stop_business_poll;

mod set_business_message_is_pinned;
pub use set_business_message_is_pinned::set_business_message_is_pinned;

mod read_business_message;
pub use read_business_message::read_business_message;

mod delete_business_messages;
pub use delete_business_messages::delete_business_messages;

mod edit_business_story;
pub use edit_business_story::edit_business_story;

mod delete_business_story;
pub use delete_business_story::delete_business_story;

mod set_business_account_name;
pub use set_business_account_name::set_business_account_name;

mod set_business_account_bio;
pub use set_business_account_bio::set_business_account_bio;

mod set_business_account_profile_photo;
pub use set_business_account_profile_photo::set_business_account_profile_photo;

mod set_business_account_username;
pub use set_business_account_username::set_business_account_username;

mod set_business_account_gift_settings;
pub use set_business_account_gift_settings::set_business_account_gift_settings;

mod get_business_account_star_amount;
pub use get_business_account_star_amount::get_business_account_star_amount;

mod transfer_business_account_stars;
pub use transfer_business_account_stars::transfer_business_account_stars;

mod check_quick_reply_shortcut_name;
pub use check_quick_reply_shortcut_name::check_quick_reply_shortcut_name;

mod load_quick_reply_shortcuts;
pub use load_quick_reply_shortcuts::load_quick_reply_shortcuts;

mod set_quick_reply_shortcut_name;
pub use set_quick_reply_shortcut_name::set_quick_reply_shortcut_name;

mod delete_quick_reply_shortcut;
pub use delete_quick_reply_shortcut::delete_quick_reply_shortcut;

mod reorder_quick_reply_shortcuts;
pub use reorder_quick_reply_shortcuts::reorder_quick_reply_shortcuts;

mod load_quick_reply_shortcut_messages;
pub use load_quick_reply_shortcut_messages::load_quick_reply_shortcut_messages;

mod delete_quick_reply_shortcut_messages;
pub use delete_quick_reply_shortcut_messages::delete_quick_reply_shortcut_messages;

mod add_quick_reply_shortcut_message;
pub use add_quick_reply_shortcut_message::add_quick_reply_shortcut_message;

mod add_quick_reply_shortcut_inline_query_result_message;
pub use add_quick_reply_shortcut_inline_query_result_message::add_quick_reply_shortcut_inline_query_result_message;

mod add_quick_reply_shortcut_message_album;
pub use add_quick_reply_shortcut_message_album::add_quick_reply_shortcut_message_album;

mod readd_quick_reply_shortcut_messages;
pub use readd_quick_reply_shortcut_messages::readd_quick_reply_shortcut_messages;

mod edit_quick_reply_message;
pub use edit_quick_reply_message::edit_quick_reply_message;

mod get_forum_topic_default_icons;
pub use get_forum_topic_default_icons::get_forum_topic_default_icons;

mod create_forum_topic;
pub use create_forum_topic::create_forum_topic;

mod edit_forum_topic;
pub use edit_forum_topic::edit_forum_topic;

mod get_forum_topic;
pub use get_forum_topic::get_forum_topic;

mod get_forum_topic_history;
pub use get_forum_topic_history::get_forum_topic_history;

mod get_forum_topic_link;
pub use get_forum_topic_link::get_forum_topic_link;

mod get_forum_topics;
pub use get_forum_topics::get_forum_topics;

mod set_forum_topic_notification_settings;
pub use set_forum_topic_notification_settings::set_forum_topic_notification_settings;

mod toggle_forum_topic_is_closed;
pub use toggle_forum_topic_is_closed::toggle_forum_topic_is_closed;

mod toggle_general_forum_topic_is_hidden;
pub use toggle_general_forum_topic_is_hidden::toggle_general_forum_topic_is_hidden;

mod toggle_forum_topic_is_pinned;
pub use toggle_forum_topic_is_pinned::toggle_forum_topic_is_pinned;

mod set_pinned_forum_topics;
pub use set_pinned_forum_topics::set_pinned_forum_topics;

mod delete_forum_topic;
pub use delete_forum_topic::delete_forum_topic;

mod read_all_forum_topic_mentions;
pub use read_all_forum_topic_mentions::read_all_forum_topic_mentions;

mod read_all_forum_topic_reactions;
pub use read_all_forum_topic_reactions::read_all_forum_topic_reactions;

mod unpin_all_forum_topic_messages;
pub use unpin_all_forum_topic_messages::unpin_all_forum_topic_messages;

mod get_passkey_parameters;
pub use get_passkey_parameters::get_passkey_parameters;

mod add_login_passkey;
pub use add_login_passkey::add_login_passkey;

mod get_login_passkeys;
pub use get_login_passkeys::get_login_passkeys;

mod remove_login_passkey;
pub use remove_login_passkey::remove_login_passkey;

mod get_emoji_reaction;
pub use get_emoji_reaction::get_emoji_reaction;

mod get_custom_emoji_reaction_animations;
pub use get_custom_emoji_reaction_animations::get_custom_emoji_reaction_animations;

mod get_message_available_reactions;
pub use get_message_available_reactions::get_message_available_reactions;

mod clear_recent_reactions;
pub use clear_recent_reactions::clear_recent_reactions;

mod add_message_reaction;
pub use add_message_reaction::add_message_reaction;

mod remove_message_reaction;
pub use remove_message_reaction::remove_message_reaction;

mod get_chat_available_paid_message_reaction_senders;
pub use get_chat_available_paid_message_reaction_senders::get_chat_available_paid_message_reaction_senders;

mod add_pending_paid_message_reaction;
pub use add_pending_paid_message_reaction::add_pending_paid_message_reaction;

mod commit_pending_paid_message_reactions;
pub use commit_pending_paid_message_reactions::commit_pending_paid_message_reactions;

mod remove_pending_paid_message_reactions;
pub use remove_pending_paid_message_reactions::remove_pending_paid_message_reactions;

mod set_paid_message_reaction_type;
pub use set_paid_message_reaction_type::set_paid_message_reaction_type;

mod set_message_reactions;
pub use set_message_reactions::set_message_reactions;

mod get_message_added_reactions;
pub use get_message_added_reactions::get_message_added_reactions;

mod set_default_reaction_type;
pub use set_default_reaction_type::set_default_reaction_type;

mod get_saved_messages_tags;
pub use get_saved_messages_tags::get_saved_messages_tags;

mod set_saved_messages_tag_label;
pub use set_saved_messages_tag_label::set_saved_messages_tag_label;

mod get_message_effect;
pub use get_message_effect::get_message_effect;

mod search_quote;
pub use search_quote::search_quote;

mod get_text_entities;
pub use get_text_entities::get_text_entities;

mod parse_text_entities;
pub use parse_text_entities::parse_text_entities;

mod parse_markdown;
pub use parse_markdown::parse_markdown;

mod get_markdown_text;
pub use get_markdown_text::get_markdown_text;

mod get_country_flag_emoji;
pub use get_country_flag_emoji::get_country_flag_emoji;

mod get_file_mime_type;
pub use get_file_mime_type::get_file_mime_type;

mod get_file_extension;
pub use get_file_extension::get_file_extension;

mod clean_file_name;
pub use clean_file_name::clean_file_name;

mod get_language_pack_string;
pub use get_language_pack_string::get_language_pack_string;

mod get_json_value;
pub use get_json_value::get_json_value;

mod get_json_string;
pub use get_json_string::get_json_string;

mod get_theme_parameters_json_string;
pub use get_theme_parameters_json_string::get_theme_parameters_json_string;

mod set_poll_answer;
pub use set_poll_answer::set_poll_answer;

mod get_poll_voters;
pub use get_poll_voters::get_poll_voters;

mod stop_poll;
pub use stop_poll::stop_poll;

mod add_checklist_tasks;
pub use add_checklist_tasks::add_checklist_tasks;

mod mark_checklist_tasks_as_done;
pub use mark_checklist_tasks_as_done::mark_checklist_tasks_as_done;

mod hide_suggested_action;
pub use hide_suggested_action::hide_suggested_action;

mod hide_contact_close_birthdays;
pub use hide_contact_close_birthdays::hide_contact_close_birthdays;

mod get_business_connection;
pub use get_business_connection::get_business_connection;

mod get_login_url_info;
pub use get_login_url_info::get_login_url_info;

mod get_login_url;
pub use get_login_url::get_login_url;

mod share_users_with_bot;
pub use share_users_with_bot::share_users_with_bot;

mod share_chat_with_bot;
pub use share_chat_with_bot::share_chat_with_bot;

mod get_inline_query_results;
pub use get_inline_query_results::get_inline_query_results;

mod answer_inline_query;
pub use answer_inline_query::answer_inline_query;

mod save_prepared_inline_message;
pub use save_prepared_inline_message::save_prepared_inline_message;

mod get_prepared_inline_message;
pub use get_prepared_inline_message::get_prepared_inline_message;

mod get_grossing_web_app_bots;
pub use get_grossing_web_app_bots::get_grossing_web_app_bots;

mod search_web_app;
pub use search_web_app::search_web_app;

mod get_web_app_placeholder;
pub use get_web_app_placeholder::get_web_app_placeholder;

mod get_web_app_link_url;
pub use get_web_app_link_url::get_web_app_link_url;

mod get_main_web_app;
pub use get_main_web_app::get_main_web_app;

mod get_web_app_url;
pub use get_web_app_url::get_web_app_url;

mod send_web_app_data;
pub use send_web_app_data::send_web_app_data;

mod open_web_app;
pub use open_web_app::open_web_app;

mod close_web_app;
pub use close_web_app::close_web_app;

mod answer_web_app_query;
pub use answer_web_app_query::answer_web_app_query;

mod check_web_app_file_download;
pub use check_web_app_file_download::check_web_app_file_download;

mod get_callback_query_answer;
pub use get_callback_query_answer::get_callback_query_answer;

mod answer_callback_query;
pub use answer_callback_query::answer_callback_query;

mod answer_shipping_query;
pub use answer_shipping_query::answer_shipping_query;

mod answer_pre_checkout_query;
pub use answer_pre_checkout_query::answer_pre_checkout_query;

mod set_game_score;
pub use set_game_score::set_game_score;

mod set_inline_game_score;
pub use set_inline_game_score::set_inline_game_score;

mod get_game_high_scores;
pub use get_game_high_scores::get_game_high_scores;

mod get_inline_game_high_scores;
pub use get_inline_game_high_scores::get_inline_game_high_scores;

mod delete_chat_reply_markup;
pub use delete_chat_reply_markup::delete_chat_reply_markup;

mod send_chat_action;
pub use send_chat_action::send_chat_action;

mod send_text_message_draft;
pub use send_text_message_draft::send_text_message_draft;

mod open_chat;
pub use open_chat::open_chat;

mod close_chat;
pub use close_chat::close_chat;

mod view_messages;
pub use view_messages::view_messages;

mod open_message_content;
pub use open_message_content::open_message_content;

mod click_animated_emoji_message;
pub use click_animated_emoji_message::click_animated_emoji_message;

mod get_internal_link;
pub use get_internal_link::get_internal_link;

mod get_internal_link_type;
pub use get_internal_link_type::get_internal_link_type;

mod get_external_link_info;
pub use get_external_link_info::get_external_link_info;

mod get_external_link;
pub use get_external_link::get_external_link;

mod get_oauth_link_info;
pub use get_oauth_link_info::get_oauth_link_info;

mod check_oauth_request_match_code;
pub use check_oauth_request_match_code::check_oauth_request_match_code;

mod accept_oauth_request;
pub use accept_oauth_request::accept_oauth_request;

mod decline_oauth_request;
pub use decline_oauth_request::decline_oauth_request;

mod read_all_chat_mentions;
pub use read_all_chat_mentions::read_all_chat_mentions;

mod read_all_chat_reactions;
pub use read_all_chat_reactions::read_all_chat_reactions;

mod create_private_chat;
pub use create_private_chat::create_private_chat;

mod create_basic_group_chat;
pub use create_basic_group_chat::create_basic_group_chat;

mod create_supergroup_chat;
pub use create_supergroup_chat::create_supergroup_chat;

mod create_secret_chat;
pub use create_secret_chat::create_secret_chat;

mod create_new_basic_group_chat;
pub use create_new_basic_group_chat::create_new_basic_group_chat;

mod create_new_supergroup_chat;
pub use create_new_supergroup_chat::create_new_supergroup_chat;

mod create_new_secret_chat;
pub use create_new_secret_chat::create_new_secret_chat;

mod upgrade_basic_group_chat_to_supergroup_chat;
pub use upgrade_basic_group_chat_to_supergroup_chat::upgrade_basic_group_chat_to_supergroup_chat;

mod get_chat_lists_to_add_chat;
pub use get_chat_lists_to_add_chat::get_chat_lists_to_add_chat;

mod add_chat_to_list;
pub use add_chat_to_list::add_chat_to_list;

mod get_chat_folder;
pub use get_chat_folder::get_chat_folder;

mod create_chat_folder;
pub use create_chat_folder::create_chat_folder;

mod edit_chat_folder;
pub use edit_chat_folder::edit_chat_folder;

mod delete_chat_folder;
pub use delete_chat_folder::delete_chat_folder;

mod get_chat_folder_chats_to_leave;
pub use get_chat_folder_chats_to_leave::get_chat_folder_chats_to_leave;

mod get_chat_folder_chat_count;
pub use get_chat_folder_chat_count::get_chat_folder_chat_count;

mod reorder_chat_folders;
pub use reorder_chat_folders::reorder_chat_folders;

mod toggle_chat_folder_tags;
pub use toggle_chat_folder_tags::toggle_chat_folder_tags;

mod get_recommended_chat_folders;
pub use get_recommended_chat_folders::get_recommended_chat_folders;

mod get_chat_folder_default_icon_name;
pub use get_chat_folder_default_icon_name::get_chat_folder_default_icon_name;

mod get_chats_for_chat_folder_invite_link;
pub use get_chats_for_chat_folder_invite_link::get_chats_for_chat_folder_invite_link;

mod create_chat_folder_invite_link;
pub use create_chat_folder_invite_link::create_chat_folder_invite_link;

mod get_chat_folder_invite_links;
pub use get_chat_folder_invite_links::get_chat_folder_invite_links;

mod edit_chat_folder_invite_link;
pub use edit_chat_folder_invite_link::edit_chat_folder_invite_link;

mod delete_chat_folder_invite_link;
pub use delete_chat_folder_invite_link::delete_chat_folder_invite_link;

mod check_chat_folder_invite_link;
pub use check_chat_folder_invite_link::check_chat_folder_invite_link;

mod add_chat_folder_by_invite_link;
pub use add_chat_folder_by_invite_link::add_chat_folder_by_invite_link;

mod get_chat_folder_new_chats;
pub use get_chat_folder_new_chats::get_chat_folder_new_chats;

mod process_chat_folder_new_chats;
pub use process_chat_folder_new_chats::process_chat_folder_new_chats;

mod get_archive_chat_list_settings;
pub use get_archive_chat_list_settings::get_archive_chat_list_settings;

mod set_archive_chat_list_settings;
pub use set_archive_chat_list_settings::set_archive_chat_list_settings;

mod set_chat_title;
pub use set_chat_title::set_chat_title;

mod set_chat_photo;
pub use set_chat_photo::set_chat_photo;

mod set_chat_accent_color;
pub use set_chat_accent_color::set_chat_accent_color;

mod set_chat_profile_accent_color;
pub use set_chat_profile_accent_color::set_chat_profile_accent_color;

mod set_chat_message_auto_delete_time;
pub use set_chat_message_auto_delete_time::set_chat_message_auto_delete_time;

mod set_chat_emoji_status;
pub use set_chat_emoji_status::set_chat_emoji_status;

mod set_chat_permissions;
pub use set_chat_permissions::set_chat_permissions;

mod set_chat_background;
pub use set_chat_background::set_chat_background;

mod delete_chat_background;
pub use delete_chat_background::delete_chat_background;

mod get_gift_chat_themes;
pub use get_gift_chat_themes::get_gift_chat_themes;

mod set_chat_theme;
pub use set_chat_theme::set_chat_theme;

mod set_chat_draft_message;
pub use set_chat_draft_message::set_chat_draft_message;

mod set_chat_notification_settings;
pub use set_chat_notification_settings::set_chat_notification_settings;

mod toggle_chat_has_protected_content;
pub use toggle_chat_has_protected_content::toggle_chat_has_protected_content;

mod process_chat_has_protected_content_disable_request;
pub use process_chat_has_protected_content_disable_request::process_chat_has_protected_content_disable_request;

mod toggle_chat_view_as_topics;
pub use toggle_chat_view_as_topics::toggle_chat_view_as_topics;

mod toggle_chat_is_translatable;
pub use toggle_chat_is_translatable::toggle_chat_is_translatable;

mod toggle_chat_is_marked_as_unread;
pub use toggle_chat_is_marked_as_unread::toggle_chat_is_marked_as_unread;

mod toggle_chat_default_disable_notification;
pub use toggle_chat_default_disable_notification::toggle_chat_default_disable_notification;

mod set_chat_available_reactions;
pub use set_chat_available_reactions::set_chat_available_reactions;

mod set_chat_client_data;
pub use set_chat_client_data::set_chat_client_data;

mod set_chat_description;
pub use set_chat_description::set_chat_description;

mod set_chat_discussion_group;
pub use set_chat_discussion_group::set_chat_discussion_group;

mod set_chat_direct_messages_group;
pub use set_chat_direct_messages_group::set_chat_direct_messages_group;

mod set_chat_location;
pub use set_chat_location::set_chat_location;

mod set_chat_slow_mode_delay;
pub use set_chat_slow_mode_delay::set_chat_slow_mode_delay;

mod pin_chat_message;
pub use pin_chat_message::pin_chat_message;

mod unpin_chat_message;
pub use unpin_chat_message::unpin_chat_message;

mod unpin_all_chat_messages;
pub use unpin_all_chat_messages::unpin_all_chat_messages;

mod join_chat;
pub use join_chat::join_chat;

mod leave_chat;
pub use leave_chat::leave_chat;

mod add_chat_member;
pub use add_chat_member::add_chat_member;

mod add_chat_members;
pub use add_chat_members::add_chat_members;

mod set_chat_member_status;
pub use set_chat_member_status::set_chat_member_status;

mod set_chat_member_tag;
pub use set_chat_member_tag::set_chat_member_tag;

mod ban_chat_member;
pub use ban_chat_member::ban_chat_member;

mod can_transfer_ownership;
pub use can_transfer_ownership::can_transfer_ownership;

mod transfer_chat_ownership;
pub use transfer_chat_ownership::transfer_chat_ownership;

mod get_chat_owner_after_leaving;
pub use get_chat_owner_after_leaving::get_chat_owner_after_leaving;

mod get_chat_member;
pub use get_chat_member::get_chat_member;

mod search_chat_members;
pub use search_chat_members::search_chat_members;

mod get_chat_administrators;
pub use get_chat_administrators::get_chat_administrators;

mod clear_all_draft_messages;
pub use clear_all_draft_messages::clear_all_draft_messages;

mod get_stake_dice_state;
pub use get_stake_dice_state::get_stake_dice_state;

mod get_saved_notification_sound;
pub use get_saved_notification_sound::get_saved_notification_sound;

mod get_saved_notification_sounds;
pub use get_saved_notification_sounds::get_saved_notification_sounds;

mod add_saved_notification_sound;
pub use add_saved_notification_sound::add_saved_notification_sound;

mod remove_saved_notification_sound;
pub use remove_saved_notification_sound::remove_saved_notification_sound;

mod get_chat_notification_settings_exceptions;
pub use get_chat_notification_settings_exceptions::get_chat_notification_settings_exceptions;

mod get_scope_notification_settings;
pub use get_scope_notification_settings::get_scope_notification_settings;

mod set_scope_notification_settings;
pub use set_scope_notification_settings::set_scope_notification_settings;

mod set_reaction_notification_settings;
pub use set_reaction_notification_settings::set_reaction_notification_settings;

mod reset_all_notification_settings;
pub use reset_all_notification_settings::reset_all_notification_settings;

mod toggle_chat_is_pinned;
pub use toggle_chat_is_pinned::toggle_chat_is_pinned;

mod set_pinned_chats;
pub use set_pinned_chats::set_pinned_chats;

mod read_chat_list;
pub use read_chat_list::read_chat_list;

mod get_current_weather;
pub use get_current_weather::get_current_weather;

mod get_story;
pub use get_story::get_story;

mod get_chats_to_post_stories;
pub use get_chats_to_post_stories::get_chats_to_post_stories;

mod can_post_story;
pub use can_post_story::can_post_story;

mod post_story;
pub use post_story::post_story;

mod start_live_story;
pub use start_live_story::start_live_story;

mod edit_story;
pub use edit_story::edit_story;

mod edit_story_cover;
pub use edit_story_cover::edit_story_cover;

mod set_story_privacy_settings;
pub use set_story_privacy_settings::set_story_privacy_settings;

mod toggle_story_is_posted_to_chat_page;
pub use toggle_story_is_posted_to_chat_page::toggle_story_is_posted_to_chat_page;

mod delete_story;
pub use delete_story::delete_story;

mod get_story_notification_settings_exceptions;
pub use get_story_notification_settings_exceptions::get_story_notification_settings_exceptions;

mod load_active_stories;
pub use load_active_stories::load_active_stories;

mod set_chat_active_stories_list;
pub use set_chat_active_stories_list::set_chat_active_stories_list;

mod get_chat_active_stories;
pub use get_chat_active_stories::get_chat_active_stories;

mod get_chat_posted_to_chat_page_stories;
pub use get_chat_posted_to_chat_page_stories::get_chat_posted_to_chat_page_stories;

mod get_chat_archived_stories;
pub use get_chat_archived_stories::get_chat_archived_stories;

mod set_chat_pinned_stories;
pub use set_chat_pinned_stories::set_chat_pinned_stories;

mod open_story;
pub use open_story::open_story;

mod close_story;
pub use close_story::close_story;

mod get_story_available_reactions;
pub use get_story_available_reactions::get_story_available_reactions;

mod set_story_reaction;
pub use set_story_reaction::set_story_reaction;

mod get_story_interactions;
pub use get_story_interactions::get_story_interactions;

mod get_chat_story_interactions;
pub use get_chat_story_interactions::get_chat_story_interactions;

mod report_story;
pub use report_story::report_story;

mod activate_story_stealth_mode;
pub use activate_story_stealth_mode::activate_story_stealth_mode;

mod get_story_public_forwards;
pub use get_story_public_forwards::get_story_public_forwards;

mod get_chat_story_albums;
pub use get_chat_story_albums::get_chat_story_albums;

mod get_story_album_stories;
pub use get_story_album_stories::get_story_album_stories;

mod create_story_album;
pub use create_story_album::create_story_album;

mod reorder_story_albums;
pub use reorder_story_albums::reorder_story_albums;

mod delete_story_album;
pub use delete_story_album::delete_story_album;

mod set_story_album_name;
pub use set_story_album_name::set_story_album_name;

mod add_story_album_stories;
pub use add_story_album_stories::add_story_album_stories;

mod remove_story_album_stories;
pub use remove_story_album_stories::remove_story_album_stories;

mod reorder_story_album_stories;
pub use reorder_story_album_stories::reorder_story_album_stories;

mod get_chat_boost_level_features;
pub use get_chat_boost_level_features::get_chat_boost_level_features;

mod get_chat_boost_features;
pub use get_chat_boost_features::get_chat_boost_features;

mod get_available_chat_boost_slots;
pub use get_available_chat_boost_slots::get_available_chat_boost_slots;

mod get_chat_boost_status;
pub use get_chat_boost_status::get_chat_boost_status;

mod boost_chat;
pub use boost_chat::boost_chat;

mod get_chat_boost_link;
pub use get_chat_boost_link::get_chat_boost_link;

mod get_chat_boost_link_info;
pub use get_chat_boost_link_info::get_chat_boost_link_info;

mod get_chat_boosts;
pub use get_chat_boosts::get_chat_boosts;

mod get_user_chat_boosts;
pub use get_user_chat_boosts::get_user_chat_boosts;

mod get_attachment_menu_bot;
pub use get_attachment_menu_bot::get_attachment_menu_bot;

mod toggle_bot_is_added_to_attachment_menu;
pub use toggle_bot_is_added_to_attachment_menu::toggle_bot_is_added_to_attachment_menu;

mod get_themed_emoji_statuses;
pub use get_themed_emoji_statuses::get_themed_emoji_statuses;

mod get_recent_emoji_statuses;
pub use get_recent_emoji_statuses::get_recent_emoji_statuses;

mod get_upgraded_gift_emoji_statuses;
pub use get_upgraded_gift_emoji_statuses::get_upgraded_gift_emoji_statuses;

mod get_default_emoji_statuses;
pub use get_default_emoji_statuses::get_default_emoji_statuses;

mod clear_recent_emoji_statuses;
pub use clear_recent_emoji_statuses::clear_recent_emoji_statuses;

mod get_themed_chat_emoji_statuses;
pub use get_themed_chat_emoji_statuses::get_themed_chat_emoji_statuses;

mod get_default_chat_emoji_statuses;
pub use get_default_chat_emoji_statuses::get_default_chat_emoji_statuses;

mod get_disallowed_chat_emoji_statuses;
pub use get_disallowed_chat_emoji_statuses::get_disallowed_chat_emoji_statuses;

mod download_file;
pub use download_file::download_file;

mod get_file_downloaded_prefix_size;
pub use get_file_downloaded_prefix_size::get_file_downloaded_prefix_size;

mod cancel_download_file;
pub use cancel_download_file::cancel_download_file;

mod get_suggested_file_name;
pub use get_suggested_file_name::get_suggested_file_name;

mod preliminary_upload_file;
pub use preliminary_upload_file::preliminary_upload_file;

mod cancel_preliminary_upload_file;
pub use cancel_preliminary_upload_file::cancel_preliminary_upload_file;

mod write_generated_file_part;
pub use write_generated_file_part::write_generated_file_part;

mod set_file_generation_progress;
pub use set_file_generation_progress::set_file_generation_progress;

mod finish_file_generation;
pub use finish_file_generation::finish_file_generation;

mod read_file_part;
pub use read_file_part::read_file_part;

mod delete_file;
pub use delete_file::delete_file;

mod add_file_to_downloads;
pub use add_file_to_downloads::add_file_to_downloads;

mod toggle_download_is_paused;
pub use toggle_download_is_paused::toggle_download_is_paused;

mod toggle_all_downloads_are_paused;
pub use toggle_all_downloads_are_paused::toggle_all_downloads_are_paused;

mod remove_file_from_downloads;
pub use remove_file_from_downloads::remove_file_from_downloads;

mod remove_all_files_from_downloads;
pub use remove_all_files_from_downloads::remove_all_files_from_downloads;

mod search_file_downloads;
pub use search_file_downloads::search_file_downloads;

mod set_application_verification_token;
pub use set_application_verification_token::set_application_verification_token;

mod get_message_file_type;
pub use get_message_file_type::get_message_file_type;

mod get_message_import_confirmation_text;
pub use get_message_import_confirmation_text::get_message_import_confirmation_text;

mod import_messages;
pub use import_messages::import_messages;

mod replace_primary_chat_invite_link;
pub use replace_primary_chat_invite_link::replace_primary_chat_invite_link;

mod create_chat_invite_link;
pub use create_chat_invite_link::create_chat_invite_link;

mod create_chat_subscription_invite_link;
pub use create_chat_subscription_invite_link::create_chat_subscription_invite_link;

mod edit_chat_invite_link;
pub use edit_chat_invite_link::edit_chat_invite_link;

mod edit_chat_subscription_invite_link;
pub use edit_chat_subscription_invite_link::edit_chat_subscription_invite_link;

mod get_chat_invite_link;
pub use get_chat_invite_link::get_chat_invite_link;

mod get_chat_invite_link_counts;
pub use get_chat_invite_link_counts::get_chat_invite_link_counts;

mod get_chat_invite_links;
pub use get_chat_invite_links::get_chat_invite_links;

mod get_chat_invite_link_members;
pub use get_chat_invite_link_members::get_chat_invite_link_members;

mod revoke_chat_invite_link;
pub use revoke_chat_invite_link::revoke_chat_invite_link;

mod delete_revoked_chat_invite_link;
pub use delete_revoked_chat_invite_link::delete_revoked_chat_invite_link;

mod delete_all_revoked_chat_invite_links;
pub use delete_all_revoked_chat_invite_links::delete_all_revoked_chat_invite_links;

mod check_chat_invite_link;
pub use check_chat_invite_link::check_chat_invite_link;

mod join_chat_by_invite_link;
pub use join_chat_by_invite_link::join_chat_by_invite_link;

mod get_chat_join_requests;
pub use get_chat_join_requests::get_chat_join_requests;

mod process_chat_join_request;
pub use process_chat_join_request::process_chat_join_request;

mod process_chat_join_requests;
pub use process_chat_join_requests::process_chat_join_requests;

mod approve_suggested_post;
pub use approve_suggested_post::approve_suggested_post;

mod decline_suggested_post;
pub use decline_suggested_post::decline_suggested_post;

mod add_offer;
pub use add_offer::add_offer;

mod create_call;
pub use create_call::create_call;

mod accept_call;
pub use accept_call::accept_call;

mod send_call_signaling_data;
pub use send_call_signaling_data::send_call_signaling_data;

mod discard_call;
pub use discard_call::discard_call;

mod send_call_rating;
pub use send_call_rating::send_call_rating;

mod send_call_debug_information;
pub use send_call_debug_information::send_call_debug_information;

mod send_call_log;
pub use send_call_log::send_call_log;

mod get_video_chat_available_participants;
pub use get_video_chat_available_participants::get_video_chat_available_participants;

mod set_video_chat_default_participant;
pub use set_video_chat_default_participant::set_video_chat_default_participant;

mod create_video_chat;
pub use create_video_chat::create_video_chat;

mod create_group_call;
pub use create_group_call::create_group_call;

mod get_video_chat_rtmp_url;
pub use get_video_chat_rtmp_url::get_video_chat_rtmp_url;

mod replace_video_chat_rtmp_url;
pub use replace_video_chat_rtmp_url::replace_video_chat_rtmp_url;

mod get_live_story_rtmp_url;
pub use get_live_story_rtmp_url::get_live_story_rtmp_url;

mod replace_live_story_rtmp_url;
pub use replace_live_story_rtmp_url::replace_live_story_rtmp_url;

mod get_group_call;
pub use get_group_call::get_group_call;

mod start_scheduled_video_chat;
pub use start_scheduled_video_chat::start_scheduled_video_chat;

mod toggle_video_chat_enabled_start_notification;
pub use toggle_video_chat_enabled_start_notification::toggle_video_chat_enabled_start_notification;

mod join_group_call;
pub use join_group_call::join_group_call;

mod join_video_chat;
pub use join_video_chat::join_video_chat;

mod join_live_story;
pub use join_live_story::join_live_story;

mod start_group_call_screen_sharing;
pub use start_group_call_screen_sharing::start_group_call_screen_sharing;

mod toggle_group_call_screen_sharing_is_paused;
pub use toggle_group_call_screen_sharing_is_paused::toggle_group_call_screen_sharing_is_paused;

mod end_group_call_screen_sharing;
pub use end_group_call_screen_sharing::end_group_call_screen_sharing;

mod set_video_chat_title;
pub use set_video_chat_title::set_video_chat_title;

mod toggle_video_chat_mute_new_participants;
pub use toggle_video_chat_mute_new_participants::toggle_video_chat_mute_new_participants;

mod toggle_group_call_are_messages_allowed;
pub use toggle_group_call_are_messages_allowed::toggle_group_call_are_messages_allowed;

mod get_live_story_streamer;
pub use get_live_story_streamer::get_live_story_streamer;

mod get_live_story_available_message_senders;
pub use get_live_story_available_message_senders::get_live_story_available_message_senders;

mod set_live_story_message_sender;
pub use set_live_story_message_sender::set_live_story_message_sender;

mod send_group_call_message;
pub use send_group_call_message::send_group_call_message;

mod add_pending_live_story_reaction;
pub use add_pending_live_story_reaction::add_pending_live_story_reaction;

mod commit_pending_live_story_reactions;
pub use commit_pending_live_story_reactions::commit_pending_live_story_reactions;

mod remove_pending_live_story_reactions;
pub use remove_pending_live_story_reactions::remove_pending_live_story_reactions;

mod delete_group_call_messages;
pub use delete_group_call_messages::delete_group_call_messages;

mod delete_group_call_messages_by_sender;
pub use delete_group_call_messages_by_sender::delete_group_call_messages_by_sender;

mod get_live_story_top_donors;
pub use get_live_story_top_donors::get_live_story_top_donors;

mod invite_group_call_participant;
pub use invite_group_call_participant::invite_group_call_participant;

mod decline_group_call_invitation;
pub use decline_group_call_invitation::decline_group_call_invitation;

mod ban_group_call_participants;
pub use ban_group_call_participants::ban_group_call_participants;

mod invite_video_chat_participants;
pub use invite_video_chat_participants::invite_video_chat_participants;

mod get_video_chat_invite_link;
pub use get_video_chat_invite_link::get_video_chat_invite_link;

mod revoke_group_call_invite_link;
pub use revoke_group_call_invite_link::revoke_group_call_invite_link;

mod start_group_call_recording;
pub use start_group_call_recording::start_group_call_recording;

mod end_group_call_recording;
pub use end_group_call_recording::end_group_call_recording;

mod toggle_group_call_is_my_video_paused;
pub use toggle_group_call_is_my_video_paused::toggle_group_call_is_my_video_paused;

mod toggle_group_call_is_my_video_enabled;
pub use toggle_group_call_is_my_video_enabled::toggle_group_call_is_my_video_enabled;

mod set_group_call_paid_message_star_count;
pub use set_group_call_paid_message_star_count::set_group_call_paid_message_star_count;

mod set_group_call_participant_is_speaking;
pub use set_group_call_participant_is_speaking::set_group_call_participant_is_speaking;

mod toggle_group_call_participant_is_muted;
pub use toggle_group_call_participant_is_muted::toggle_group_call_participant_is_muted;

mod set_group_call_participant_volume_level;
pub use set_group_call_participant_volume_level::set_group_call_participant_volume_level;

mod toggle_group_call_participant_is_hand_raised;
pub use toggle_group_call_participant_is_hand_raised::toggle_group_call_participant_is_hand_raised;

mod get_group_call_participants;
pub use get_group_call_participants::get_group_call_participants;

mod load_group_call_participants;
pub use load_group_call_participants::load_group_call_participants;

mod leave_group_call;
pub use leave_group_call::leave_group_call;

mod end_group_call;
pub use end_group_call::end_group_call;

mod get_group_call_streams;
pub use get_group_call_streams::get_group_call_streams;

mod get_group_call_stream_segment;
pub use get_group_call_stream_segment::get_group_call_stream_segment;

mod encrypt_group_call_data;
pub use encrypt_group_call_data::encrypt_group_call_data;

mod decrypt_group_call_data;
pub use decrypt_group_call_data::decrypt_group_call_data;

mod set_message_sender_block_list;
pub use set_message_sender_block_list::set_message_sender_block_list;

mod block_message_sender_from_replies;
pub use block_message_sender_from_replies::block_message_sender_from_replies;

mod get_blocked_message_senders;
pub use get_blocked_message_senders::get_blocked_message_senders;

mod add_contact;
pub use add_contact::add_contact;

mod import_contacts;
pub use import_contacts::import_contacts;

mod get_contacts;
pub use get_contacts::get_contacts;

mod search_contacts;
pub use search_contacts::search_contacts;

mod remove_contacts;
pub use remove_contacts::remove_contacts;

mod get_imported_contact_count;
pub use get_imported_contact_count::get_imported_contact_count;

mod change_imported_contacts;
pub use change_imported_contacts::change_imported_contacts;

mod clear_imported_contacts;
pub use clear_imported_contacts::clear_imported_contacts;

mod set_close_friends;
pub use set_close_friends::set_close_friends;

mod get_close_friends;
pub use get_close_friends::get_close_friends;

mod set_user_personal_profile_photo;
pub use set_user_personal_profile_photo::set_user_personal_profile_photo;

mod set_user_note;
pub use set_user_note::set_user_note;

mod suggest_user_profile_photo;
pub use suggest_user_profile_photo::suggest_user_profile_photo;

mod suggest_user_birthdate;
pub use suggest_user_birthdate::suggest_user_birthdate;

mod toggle_bot_can_manage_emoji_status;
pub use toggle_bot_can_manage_emoji_status::toggle_bot_can_manage_emoji_status;

mod set_user_emoji_status;
pub use set_user_emoji_status::set_user_emoji_status;

mod search_user_by_phone_number;
pub use search_user_by_phone_number::search_user_by_phone_number;

mod share_phone_number;
pub use share_phone_number::share_phone_number;

mod get_user_profile_photos;
pub use get_user_profile_photos::get_user_profile_photos;

mod get_user_profile_audios;
pub use get_user_profile_audios::get_user_profile_audios;

mod is_profile_audio;
pub use is_profile_audio::is_profile_audio;

mod add_profile_audio;
pub use add_profile_audio::add_profile_audio;

mod set_profile_audio_position;
pub use set_profile_audio_position::set_profile_audio_position;

mod remove_profile_audio;
pub use remove_profile_audio::remove_profile_audio;

mod get_sticker_outline;
pub use get_sticker_outline::get_sticker_outline;

mod get_sticker_outline_svg_path;
pub use get_sticker_outline_svg_path::get_sticker_outline_svg_path;

mod get_stickers;
pub use get_stickers::get_stickers;

mod get_all_sticker_emojis;
pub use get_all_sticker_emojis::get_all_sticker_emojis;

mod search_stickers;
pub use search_stickers::search_stickers;

mod get_greeting_stickers;
pub use get_greeting_stickers::get_greeting_stickers;

mod get_premium_stickers;
pub use get_premium_stickers::get_premium_stickers;

mod get_installed_sticker_sets;
pub use get_installed_sticker_sets::get_installed_sticker_sets;

mod get_archived_sticker_sets;
pub use get_archived_sticker_sets::get_archived_sticker_sets;

mod get_trending_sticker_sets;
pub use get_trending_sticker_sets::get_trending_sticker_sets;

mod get_attached_sticker_sets;
pub use get_attached_sticker_sets::get_attached_sticker_sets;

mod get_sticker_set;
pub use get_sticker_set::get_sticker_set;

mod get_sticker_set_name;
pub use get_sticker_set_name::get_sticker_set_name;

mod search_sticker_set;
pub use search_sticker_set::search_sticker_set;

mod search_installed_sticker_sets;
pub use search_installed_sticker_sets::search_installed_sticker_sets;

mod search_sticker_sets;
pub use search_sticker_sets::search_sticker_sets;

mod change_sticker_set;
pub use change_sticker_set::change_sticker_set;

mod view_trending_sticker_sets;
pub use view_trending_sticker_sets::view_trending_sticker_sets;

mod reorder_installed_sticker_sets;
pub use reorder_installed_sticker_sets::reorder_installed_sticker_sets;

mod get_recent_stickers;
pub use get_recent_stickers::get_recent_stickers;

mod add_recent_sticker;
pub use add_recent_sticker::add_recent_sticker;

mod remove_recent_sticker;
pub use remove_recent_sticker::remove_recent_sticker;

mod clear_recent_stickers;
pub use clear_recent_stickers::clear_recent_stickers;

mod get_favorite_stickers;
pub use get_favorite_stickers::get_favorite_stickers;

mod add_favorite_sticker;
pub use add_favorite_sticker::add_favorite_sticker;

mod remove_favorite_sticker;
pub use remove_favorite_sticker::remove_favorite_sticker;

mod get_sticker_emojis;
pub use get_sticker_emojis::get_sticker_emojis;

mod search_emojis;
pub use search_emojis::search_emojis;

mod get_keyword_emojis;
pub use get_keyword_emojis::get_keyword_emojis;

mod get_emoji_categories;
pub use get_emoji_categories::get_emoji_categories;

mod get_animated_emoji;
pub use get_animated_emoji::get_animated_emoji;

mod get_emoji_suggestions_url;
pub use get_emoji_suggestions_url::get_emoji_suggestions_url;

mod get_custom_emoji_stickers;
pub use get_custom_emoji_stickers::get_custom_emoji_stickers;

mod get_default_chat_photo_custom_emoji_stickers;
pub use get_default_chat_photo_custom_emoji_stickers::get_default_chat_photo_custom_emoji_stickers;

mod get_default_profile_photo_custom_emoji_stickers;
pub use get_default_profile_photo_custom_emoji_stickers::get_default_profile_photo_custom_emoji_stickers;

mod get_default_background_custom_emoji_stickers;
pub use get_default_background_custom_emoji_stickers::get_default_background_custom_emoji_stickers;

mod get_saved_animations;
pub use get_saved_animations::get_saved_animations;

mod add_saved_animation;
pub use add_saved_animation::add_saved_animation;

mod remove_saved_animation;
pub use remove_saved_animation::remove_saved_animation;

mod get_recent_inline_bots;
pub use get_recent_inline_bots::get_recent_inline_bots;

mod get_owned_bots;
pub use get_owned_bots::get_owned_bots;

mod search_hashtags;
pub use search_hashtags::search_hashtags;

mod remove_recent_hashtag;
pub use remove_recent_hashtag::remove_recent_hashtag;

mod get_link_preview;
pub use get_link_preview::get_link_preview;

mod get_web_page_instant_view;
pub use get_web_page_instant_view::get_web_page_instant_view;

mod set_profile_photo;
pub use set_profile_photo::set_profile_photo;

mod delete_profile_photo;
pub use delete_profile_photo::delete_profile_photo;

mod set_accent_color;
pub use set_accent_color::set_accent_color;

mod set_upgraded_gift_colors;
pub use set_upgraded_gift_colors::set_upgraded_gift_colors;

mod set_profile_accent_color;
pub use set_profile_accent_color::set_profile_accent_color;

mod set_name;
pub use set_name::set_name;

mod set_bio;
pub use set_bio::set_bio;

mod set_username;
pub use set_username::set_username;

mod toggle_username_is_active;
pub use toggle_username_is_active::toggle_username_is_active;

mod reorder_active_usernames;
pub use reorder_active_usernames::reorder_active_usernames;

mod set_birthdate;
pub use set_birthdate::set_birthdate;

mod set_main_profile_tab;
pub use set_main_profile_tab::set_main_profile_tab;

mod set_personal_chat;
pub use set_personal_chat::set_personal_chat;

mod set_emoji_status;
pub use set_emoji_status::set_emoji_status;

mod toggle_has_sponsored_messages_enabled;
pub use toggle_has_sponsored_messages_enabled::toggle_has_sponsored_messages_enabled;

mod set_business_location;
pub use set_business_location::set_business_location;

mod set_business_opening_hours;
pub use set_business_opening_hours::set_business_opening_hours;

mod set_business_greeting_message_settings;
pub use set_business_greeting_message_settings::set_business_greeting_message_settings;

mod set_business_away_message_settings;
pub use set_business_away_message_settings::set_business_away_message_settings;

mod set_business_start_page;
pub use set_business_start_page::set_business_start_page;

mod send_phone_number_code;
pub use send_phone_number_code::send_phone_number_code;

mod send_phone_number_firebase_sms;
pub use send_phone_number_firebase_sms::send_phone_number_firebase_sms;

mod report_phone_number_code_missing;
pub use report_phone_number_code_missing::report_phone_number_code_missing;

mod resend_phone_number_code;
pub use resend_phone_number_code::resend_phone_number_code;

mod check_phone_number_code;
pub use check_phone_number_code::check_phone_number_code;

mod get_business_connected_bot;
pub use get_business_connected_bot::get_business_connected_bot;

mod set_business_connected_bot;
pub use set_business_connected_bot::set_business_connected_bot;

mod delete_business_connected_bot;
pub use delete_business_connected_bot::delete_business_connected_bot;

mod toggle_business_connected_bot_chat_is_paused;
pub use toggle_business_connected_bot_chat_is_paused::toggle_business_connected_bot_chat_is_paused;

mod remove_business_connected_bot_from_chat;
pub use remove_business_connected_bot_from_chat::remove_business_connected_bot_from_chat;

mod get_business_chat_links;
pub use get_business_chat_links::get_business_chat_links;

mod create_business_chat_link;
pub use create_business_chat_link::create_business_chat_link;

mod edit_business_chat_link;
pub use edit_business_chat_link::edit_business_chat_link;

mod delete_business_chat_link;
pub use delete_business_chat_link::delete_business_chat_link;

mod get_business_chat_link_info;
pub use get_business_chat_link_info::get_business_chat_link_info;

mod get_user_link;
pub use get_user_link::get_user_link;

mod search_user_by_token;
pub use search_user_by_token::search_user_by_token;

mod set_commands;
pub use set_commands::set_commands;

mod delete_commands;
pub use delete_commands::delete_commands;

mod get_commands;
pub use get_commands::get_commands;

mod set_menu_button;
pub use set_menu_button::set_menu_button;

mod get_menu_button;
pub use get_menu_button::get_menu_button;

mod set_default_group_administrator_rights;
pub use set_default_group_administrator_rights::set_default_group_administrator_rights;

mod set_default_channel_administrator_rights;
pub use set_default_channel_administrator_rights::set_default_channel_administrator_rights;

mod can_bot_send_messages;
pub use can_bot_send_messages::can_bot_send_messages;

mod allow_bot_to_send_messages;
pub use allow_bot_to_send_messages::allow_bot_to_send_messages;

mod send_web_app_custom_request;
pub use send_web_app_custom_request::send_web_app_custom_request;

mod get_bot_media_previews;
pub use get_bot_media_previews::get_bot_media_previews;

mod get_bot_media_preview_info;
pub use get_bot_media_preview_info::get_bot_media_preview_info;

mod add_bot_media_preview;
pub use add_bot_media_preview::add_bot_media_preview;

mod edit_bot_media_preview;
pub use edit_bot_media_preview::edit_bot_media_preview;

mod reorder_bot_media_previews;
pub use reorder_bot_media_previews::reorder_bot_media_previews;

mod delete_bot_media_previews;
pub use delete_bot_media_previews::delete_bot_media_previews;

mod set_bot_name;
pub use set_bot_name::set_bot_name;

mod get_bot_name;
pub use get_bot_name::get_bot_name;

mod set_bot_profile_photo;
pub use set_bot_profile_photo::set_bot_profile_photo;

mod toggle_bot_username_is_active;
pub use toggle_bot_username_is_active::toggle_bot_username_is_active;

mod reorder_bot_active_usernames;
pub use reorder_bot_active_usernames::reorder_bot_active_usernames;

mod set_bot_info_description;
pub use set_bot_info_description::set_bot_info_description;

mod get_bot_info_description;
pub use get_bot_info_description::get_bot_info_description;

mod set_bot_info_short_description;
pub use set_bot_info_short_description::set_bot_info_short_description;

mod get_bot_info_short_description;
pub use get_bot_info_short_description::get_bot_info_short_description;

mod set_message_sender_bot_verification;
pub use set_message_sender_bot_verification::set_message_sender_bot_verification;

mod remove_message_sender_bot_verification;
pub use remove_message_sender_bot_verification::remove_message_sender_bot_verification;

mod get_active_sessions;
pub use get_active_sessions::get_active_sessions;

mod terminate_session;
pub use terminate_session::terminate_session;

mod terminate_all_other_sessions;
pub use terminate_all_other_sessions::terminate_all_other_sessions;

mod confirm_session;
pub use confirm_session::confirm_session;

mod toggle_session_can_accept_calls;
pub use toggle_session_can_accept_calls::toggle_session_can_accept_calls;

mod toggle_session_can_accept_secret_chats;
pub use toggle_session_can_accept_secret_chats::toggle_session_can_accept_secret_chats;

mod set_inactive_session_ttl;
pub use set_inactive_session_ttl::set_inactive_session_ttl;

mod get_connected_websites;
pub use get_connected_websites::get_connected_websites;

mod disconnect_website;
pub use disconnect_website::disconnect_website;

mod disconnect_all_websites;
pub use disconnect_all_websites::disconnect_all_websites;

mod set_supergroup_username;
pub use set_supergroup_username::set_supergroup_username;

mod toggle_supergroup_username_is_active;
pub use toggle_supergroup_username_is_active::toggle_supergroup_username_is_active;

mod disable_all_supergroup_usernames;
pub use disable_all_supergroup_usernames::disable_all_supergroup_usernames;

mod reorder_supergroup_active_usernames;
pub use reorder_supergroup_active_usernames::reorder_supergroup_active_usernames;

mod set_supergroup_sticker_set;
pub use set_supergroup_sticker_set::set_supergroup_sticker_set;

mod set_supergroup_custom_emoji_sticker_set;
pub use set_supergroup_custom_emoji_sticker_set::set_supergroup_custom_emoji_sticker_set;

mod set_supergroup_unrestrict_boost_count;
pub use set_supergroup_unrestrict_boost_count::set_supergroup_unrestrict_boost_count;

mod set_supergroup_main_profile_tab;
pub use set_supergroup_main_profile_tab::set_supergroup_main_profile_tab;

mod toggle_supergroup_sign_messages;
pub use toggle_supergroup_sign_messages::toggle_supergroup_sign_messages;

mod toggle_supergroup_join_to_send_messages;
pub use toggle_supergroup_join_to_send_messages::toggle_supergroup_join_to_send_messages;

mod toggle_supergroup_join_by_request;
pub use toggle_supergroup_join_by_request::toggle_supergroup_join_by_request;

mod toggle_supergroup_is_all_history_available;
pub use toggle_supergroup_is_all_history_available::toggle_supergroup_is_all_history_available;

mod toggle_supergroup_can_have_sponsored_messages;
pub use toggle_supergroup_can_have_sponsored_messages::toggle_supergroup_can_have_sponsored_messages;

mod toggle_supergroup_has_automatic_translation;
pub use toggle_supergroup_has_automatic_translation::toggle_supergroup_has_automatic_translation;

mod toggle_supergroup_has_hidden_members;
pub use toggle_supergroup_has_hidden_members::toggle_supergroup_has_hidden_members;

mod toggle_supergroup_has_aggressive_anti_spam_enabled;
pub use toggle_supergroup_has_aggressive_anti_spam_enabled::toggle_supergroup_has_aggressive_anti_spam_enabled;

mod toggle_supergroup_is_forum;
pub use toggle_supergroup_is_forum::toggle_supergroup_is_forum;

mod toggle_supergroup_is_broadcast_group;
pub use toggle_supergroup_is_broadcast_group::toggle_supergroup_is_broadcast_group;

mod report_supergroup_spam;
pub use report_supergroup_spam::report_supergroup_spam;

mod report_supergroup_anti_spam_false_positive;
pub use report_supergroup_anti_spam_false_positive::report_supergroup_anti_spam_false_positive;

mod get_supergroup_members;
pub use get_supergroup_members::get_supergroup_members;

mod close_secret_chat;
pub use close_secret_chat::close_secret_chat;

mod get_chat_event_log;
pub use get_chat_event_log::get_chat_event_log;

mod get_time_zones;
pub use get_time_zones::get_time_zones;

mod get_payment_form;
pub use get_payment_form::get_payment_form;

mod validate_order_info;
pub use validate_order_info::validate_order_info;

mod send_payment_form;
pub use send_payment_form::send_payment_form;

mod get_payment_receipt;
pub use get_payment_receipt::get_payment_receipt;

mod get_saved_order_info;
pub use get_saved_order_info::get_saved_order_info;

mod delete_saved_order_info;
pub use delete_saved_order_info::delete_saved_order_info;

mod delete_saved_credentials;
pub use delete_saved_credentials::delete_saved_credentials;

mod set_gift_settings;
pub use set_gift_settings::set_gift_settings;

mod get_available_gifts;
pub use get_available_gifts::get_available_gifts;

mod can_send_gift;
pub use can_send_gift::can_send_gift;

mod send_gift;
pub use send_gift::send_gift;

mod get_gift_auction_state;
pub use get_gift_auction_state::get_gift_auction_state;

mod get_gift_auction_acquired_gifts;
pub use get_gift_auction_acquired_gifts::get_gift_auction_acquired_gifts;

mod open_gift_auction;
pub use open_gift_auction::open_gift_auction;

mod close_gift_auction;
pub use close_gift_auction::close_gift_auction;

mod place_gift_auction_bid;
pub use place_gift_auction_bid::place_gift_auction_bid;

mod increase_gift_auction_bid;
pub use increase_gift_auction_bid::increase_gift_auction_bid;

mod sell_gift;
pub use sell_gift::sell_gift;

mod toggle_gift_is_saved;
pub use toggle_gift_is_saved::toggle_gift_is_saved;

mod set_pinned_gifts;
pub use set_pinned_gifts::set_pinned_gifts;

mod toggle_chat_gift_notifications;
pub use toggle_chat_gift_notifications::toggle_chat_gift_notifications;

mod get_gift_upgrade_preview;
pub use get_gift_upgrade_preview::get_gift_upgrade_preview;

mod get_upgraded_gift_variants;
pub use get_upgraded_gift_variants::get_upgraded_gift_variants;

mod upgrade_gift;
pub use upgrade_gift::upgrade_gift;

mod buy_gift_upgrade;
pub use buy_gift_upgrade::buy_gift_upgrade;

mod craft_gift;
pub use craft_gift::craft_gift;

mod transfer_gift;
pub use transfer_gift::transfer_gift;

mod drop_gift_original_details;
pub use drop_gift_original_details::drop_gift_original_details;

mod send_resold_gift;
pub use send_resold_gift::send_resold_gift;

mod send_gift_purchase_offer;
pub use send_gift_purchase_offer::send_gift_purchase_offer;

mod process_gift_purchase_offer;
pub use process_gift_purchase_offer::process_gift_purchase_offer;

mod get_received_gifts;
pub use get_received_gifts::get_received_gifts;

mod get_received_gift;
pub use get_received_gift::get_received_gift;

mod get_gifts_for_crafting;
pub use get_gifts_for_crafting::get_gifts_for_crafting;

mod get_upgraded_gift;
pub use get_upgraded_gift::get_upgraded_gift;

mod get_upgraded_gift_value_info;
pub use get_upgraded_gift_value_info::get_upgraded_gift_value_info;

mod get_upgraded_gift_withdrawal_url;
pub use get_upgraded_gift_withdrawal_url::get_upgraded_gift_withdrawal_url;

mod get_upgraded_gifts_promotional_animation;
pub use get_upgraded_gifts_promotional_animation::get_upgraded_gifts_promotional_animation;

mod set_gift_resale_price;
pub use set_gift_resale_price::set_gift_resale_price;

mod search_gifts_for_resale;
pub use search_gifts_for_resale::search_gifts_for_resale;

mod get_gift_collections;
pub use get_gift_collections::get_gift_collections;

mod create_gift_collection;
pub use create_gift_collection::create_gift_collection;

mod reorder_gift_collections;
pub use reorder_gift_collections::reorder_gift_collections;

mod delete_gift_collection;
pub use delete_gift_collection::delete_gift_collection;

mod set_gift_collection_name;
pub use set_gift_collection_name::set_gift_collection_name;

mod add_gift_collection_gifts;
pub use add_gift_collection_gifts::add_gift_collection_gifts;

mod remove_gift_collection_gifts;
pub use remove_gift_collection_gifts::remove_gift_collection_gifts;

mod reorder_gift_collection_gifts;
pub use reorder_gift_collection_gifts::reorder_gift_collection_gifts;

mod create_invoice_link;
pub use create_invoice_link::create_invoice_link;

mod refund_star_payment;
pub use refund_star_payment::refund_star_payment;

mod get_support_user;
pub use get_support_user::get_support_user;

mod get_background_url;
pub use get_background_url::get_background_url;

mod search_background;
pub use search_background::search_background;

mod set_default_background;
pub use set_default_background::set_default_background;

mod delete_default_background;
pub use delete_default_background::delete_default_background;

mod get_installed_backgrounds;
pub use get_installed_backgrounds::get_installed_backgrounds;

mod remove_installed_background;
pub use remove_installed_background::remove_installed_background;

mod reset_installed_backgrounds;
pub use reset_installed_backgrounds::reset_installed_backgrounds;

mod get_localization_target_info;
pub use get_localization_target_info::get_localization_target_info;

mod get_language_pack_info;
pub use get_language_pack_info::get_language_pack_info;

mod get_language_pack_strings;
pub use get_language_pack_strings::get_language_pack_strings;

mod synchronize_language_pack;
pub use synchronize_language_pack::synchronize_language_pack;

mod add_custom_server_language_pack;
pub use add_custom_server_language_pack::add_custom_server_language_pack;

mod set_custom_language_pack;
pub use set_custom_language_pack::set_custom_language_pack;

mod edit_custom_language_pack_info;
pub use edit_custom_language_pack_info::edit_custom_language_pack_info;

mod set_custom_language_pack_string;
pub use set_custom_language_pack_string::set_custom_language_pack_string;

mod delete_language_pack;
pub use delete_language_pack::delete_language_pack;

mod register_device;
pub use register_device::register_device;

mod process_push_notification;
pub use process_push_notification::process_push_notification;

mod get_push_receiver_id;
pub use get_push_receiver_id::get_push_receiver_id;

mod get_recently_visited_t_me_urls;
pub use get_recently_visited_t_me_urls::get_recently_visited_t_me_urls;

mod set_user_privacy_setting_rules;
pub use set_user_privacy_setting_rules::set_user_privacy_setting_rules;

mod get_user_privacy_setting_rules;
pub use get_user_privacy_setting_rules::get_user_privacy_setting_rules;

mod set_read_date_privacy_settings;
pub use set_read_date_privacy_settings::set_read_date_privacy_settings;

mod get_read_date_privacy_settings;
pub use get_read_date_privacy_settings::get_read_date_privacy_settings;

mod set_new_chat_privacy_settings;
pub use set_new_chat_privacy_settings::set_new_chat_privacy_settings;

mod get_new_chat_privacy_settings;
pub use get_new_chat_privacy_settings::get_new_chat_privacy_settings;

mod get_paid_message_revenue;
pub use get_paid_message_revenue::get_paid_message_revenue;

mod allow_unpaid_messages_from_user;
pub use allow_unpaid_messages_from_user::allow_unpaid_messages_from_user;

mod set_chat_paid_message_star_count;
pub use set_chat_paid_message_star_count::set_chat_paid_message_star_count;

mod can_send_message_to_user;
pub use can_send_message_to_user::can_send_message_to_user;

mod get_option;
pub use get_option::get_option;

mod set_option;
pub use set_option::set_option;

mod set_account_ttl;
pub use set_account_ttl::set_account_ttl;

mod get_account_ttl;
pub use get_account_ttl::get_account_ttl;

mod delete_account;
pub use delete_account::delete_account;

mod set_default_message_auto_delete_time;
pub use set_default_message_auto_delete_time::set_default_message_auto_delete_time;

mod get_default_message_auto_delete_time;
pub use get_default_message_auto_delete_time::get_default_message_auto_delete_time;

mod remove_chat_action_bar;
pub use remove_chat_action_bar::remove_chat_action_bar;

mod report_chat;
pub use report_chat::report_chat;

mod report_chat_photo;
pub use report_chat_photo::report_chat_photo;

mod report_message_reactions;
pub use report_message_reactions::report_message_reactions;

mod get_chat_revenue_statistics;
pub use get_chat_revenue_statistics::get_chat_revenue_statistics;

mod get_chat_revenue_withdrawal_url;
pub use get_chat_revenue_withdrawal_url::get_chat_revenue_withdrawal_url;

mod get_chat_revenue_transactions;
pub use get_chat_revenue_transactions::get_chat_revenue_transactions;

mod get_ton_transactions;
pub use get_ton_transactions::get_ton_transactions;

mod get_star_revenue_statistics;
pub use get_star_revenue_statistics::get_star_revenue_statistics;

mod get_star_withdrawal_url;
pub use get_star_withdrawal_url::get_star_withdrawal_url;

mod get_star_ad_account_url;
pub use get_star_ad_account_url::get_star_ad_account_url;

mod get_ton_revenue_statistics;
pub use get_ton_revenue_statistics::get_ton_revenue_statistics;

mod get_ton_withdrawal_url;
pub use get_ton_withdrawal_url::get_ton_withdrawal_url;

mod get_chat_statistics;
pub use get_chat_statistics::get_chat_statistics;

mod get_message_statistics;
pub use get_message_statistics::get_message_statistics;

mod get_message_public_forwards;
pub use get_message_public_forwards::get_message_public_forwards;

mod get_story_statistics;
pub use get_story_statistics::get_story_statistics;

mod get_statistical_graph;
pub use get_statistical_graph::get_statistical_graph;

mod get_storage_statistics;
pub use get_storage_statistics::get_storage_statistics;

mod get_storage_statistics_fast;
pub use get_storage_statistics_fast::get_storage_statistics_fast;

mod get_database_statistics;
pub use get_database_statistics::get_database_statistics;

mod optimize_storage;
pub use optimize_storage::optimize_storage;

mod set_network_type;
pub use set_network_type::set_network_type;

mod get_network_statistics;
pub use get_network_statistics::get_network_statistics;

mod add_network_statistics;
pub use add_network_statistics::add_network_statistics;

mod reset_network_statistics;
pub use reset_network_statistics::reset_network_statistics;

mod get_auto_download_settings_presets;
pub use get_auto_download_settings_presets::get_auto_download_settings_presets;

mod set_auto_download_settings;
pub use set_auto_download_settings::set_auto_download_settings;

mod get_autosave_settings;
pub use get_autosave_settings::get_autosave_settings;

mod set_autosave_settings;
pub use set_autosave_settings::set_autosave_settings;

mod clear_autosave_settings_exceptions;
pub use clear_autosave_settings_exceptions::clear_autosave_settings_exceptions;

mod get_bank_card_info;
pub use get_bank_card_info::get_bank_card_info;

mod get_passport_element;
pub use get_passport_element::get_passport_element;

mod get_all_passport_elements;
pub use get_all_passport_elements::get_all_passport_elements;

mod set_passport_element;
pub use set_passport_element::set_passport_element;

mod delete_passport_element;
pub use delete_passport_element::delete_passport_element;

mod set_passport_element_errors;
pub use set_passport_element_errors::set_passport_element_errors;

mod get_preferred_country_language;
pub use get_preferred_country_language::get_preferred_country_language;

mod send_email_address_verification_code;
pub use send_email_address_verification_code::send_email_address_verification_code;

mod resend_email_address_verification_code;
pub use resend_email_address_verification_code::resend_email_address_verification_code;

mod check_email_address_verification_code;
pub use check_email_address_verification_code::check_email_address_verification_code;

mod get_passport_authorization_form;
pub use get_passport_authorization_form::get_passport_authorization_form;

mod get_passport_authorization_form_available_elements;
pub use get_passport_authorization_form_available_elements::get_passport_authorization_form_available_elements;

mod send_passport_authorization_form;
pub use send_passport_authorization_form::send_passport_authorization_form;

mod set_bot_updates_status;
pub use set_bot_updates_status::set_bot_updates_status;

mod upload_sticker_file;
pub use upload_sticker_file::upload_sticker_file;

mod get_suggested_sticker_set_name;
pub use get_suggested_sticker_set_name::get_suggested_sticker_set_name;

mod check_sticker_set_name;
pub use check_sticker_set_name::check_sticker_set_name;

mod create_new_sticker_set;
pub use create_new_sticker_set::create_new_sticker_set;

mod add_sticker_to_set;
pub use add_sticker_to_set::add_sticker_to_set;

mod replace_sticker_in_set;
pub use replace_sticker_in_set::replace_sticker_in_set;

mod set_sticker_set_thumbnail;
pub use set_sticker_set_thumbnail::set_sticker_set_thumbnail;

mod set_custom_emoji_sticker_set_thumbnail;
pub use set_custom_emoji_sticker_set_thumbnail::set_custom_emoji_sticker_set_thumbnail;

mod set_sticker_set_title;
pub use set_sticker_set_title::set_sticker_set_title;

mod delete_sticker_set;
pub use delete_sticker_set::delete_sticker_set;

mod set_sticker_position_in_set;
pub use set_sticker_position_in_set::set_sticker_position_in_set;

mod remove_sticker_from_set;
pub use remove_sticker_from_set::remove_sticker_from_set;

mod set_sticker_emojis;
pub use set_sticker_emojis::set_sticker_emojis;

mod set_sticker_keywords;
pub use set_sticker_keywords::set_sticker_keywords;

mod set_sticker_mask_position;
pub use set_sticker_mask_position::set_sticker_mask_position;

mod get_owned_sticker_sets;
pub use get_owned_sticker_sets::get_owned_sticker_sets;

mod get_map_thumbnail_file;
pub use get_map_thumbnail_file::get_map_thumbnail_file;

mod get_premium_limit;
pub use get_premium_limit::get_premium_limit;

mod get_premium_features;
pub use get_premium_features::get_premium_features;

mod get_premium_sticker_examples;
pub use get_premium_sticker_examples::get_premium_sticker_examples;

mod get_premium_info_sticker;
pub use get_premium_info_sticker::get_premium_info_sticker;

mod view_premium_feature;
pub use view_premium_feature::view_premium_feature;

mod click_premium_subscription_button;
pub use click_premium_subscription_button::click_premium_subscription_button;

mod get_premium_state;
pub use get_premium_state::get_premium_state;

mod get_premium_gift_payment_options;
pub use get_premium_gift_payment_options::get_premium_gift_payment_options;

mod get_premium_giveaway_payment_options;
pub use get_premium_giveaway_payment_options::get_premium_giveaway_payment_options;

mod check_premium_gift_code;
pub use check_premium_gift_code::check_premium_gift_code;

mod apply_premium_gift_code;
pub use apply_premium_gift_code::apply_premium_gift_code;

mod gift_premium_with_stars;
pub use gift_premium_with_stars::gift_premium_with_stars;

mod launch_prepaid_giveaway;
pub use launch_prepaid_giveaway::launch_prepaid_giveaway;

mod get_giveaway_info;
pub use get_giveaway_info::get_giveaway_info;

mod get_star_payment_options;
pub use get_star_payment_options::get_star_payment_options;

mod get_star_gift_payment_options;
pub use get_star_gift_payment_options::get_star_gift_payment_options;

mod get_star_giveaway_payment_options;
pub use get_star_giveaway_payment_options::get_star_giveaway_payment_options;

mod get_star_transactions;
pub use get_star_transactions::get_star_transactions;

mod get_star_subscriptions;
pub use get_star_subscriptions::get_star_subscriptions;

mod can_purchase_from_store;
pub use can_purchase_from_store::can_purchase_from_store;

mod assign_store_transaction;
pub use assign_store_transaction::assign_store_transaction;

mod edit_star_subscription;
pub use edit_star_subscription::edit_star_subscription;

mod edit_user_star_subscription;
pub use edit_user_star_subscription::edit_user_star_subscription;

mod reuse_star_subscription;
pub use reuse_star_subscription::reuse_star_subscription;

mod set_chat_affiliate_program;
pub use set_chat_affiliate_program::set_chat_affiliate_program;

mod search_chat_affiliate_program;
pub use search_chat_affiliate_program::search_chat_affiliate_program;

mod search_affiliate_programs;
pub use search_affiliate_programs::search_affiliate_programs;

mod connect_affiliate_program;
pub use connect_affiliate_program::connect_affiliate_program;

mod disconnect_affiliate_program;
pub use disconnect_affiliate_program::disconnect_affiliate_program;

mod get_connected_affiliate_program;
pub use get_connected_affiliate_program::get_connected_affiliate_program;

mod get_connected_affiliate_programs;
pub use get_connected_affiliate_programs::get_connected_affiliate_programs;

mod get_business_features;
pub use get_business_features::get_business_features;

mod accept_terms_of_service;
pub use accept_terms_of_service::accept_terms_of_service;

mod search_strings_by_prefix;
pub use search_strings_by_prefix::search_strings_by_prefix;

mod send_custom_request;
pub use send_custom_request::send_custom_request;

mod answer_custom_query;
pub use answer_custom_query::answer_custom_query;

mod set_alarm;
pub use set_alarm::set_alarm;

mod get_countries;
pub use get_countries::get_countries;

mod get_country_code;
pub use get_country_code::get_country_code;

mod get_phone_number_info;
pub use get_phone_number_info::get_phone_number_info;

mod get_phone_number_info_sync;
pub use get_phone_number_info_sync::get_phone_number_info_sync;

mod get_collectible_item_info;
pub use get_collectible_item_info::get_collectible_item_info;

mod get_deep_link_info;
pub use get_deep_link_info::get_deep_link_info;

mod get_application_config;
pub use get_application_config::get_application_config;

mod save_application_log_event;
pub use save_application_log_event::save_application_log_event;

mod get_application_download_link;
pub use get_application_download_link::get_application_download_link;

mod add_proxy;
pub use add_proxy::add_proxy;

mod edit_proxy;
pub use edit_proxy::edit_proxy;

mod enable_proxy;
pub use enable_proxy::enable_proxy;

mod disable_proxy;
pub use disable_proxy::disable_proxy;

mod remove_proxy;
pub use remove_proxy::remove_proxy;

mod get_proxies;
pub use get_proxies::get_proxies;

mod ping_proxy;
pub use ping_proxy::ping_proxy;

mod set_log_stream;
pub use set_log_stream::set_log_stream;

mod get_log_stream;
pub use get_log_stream::get_log_stream;

mod set_log_verbosity_level;
pub use set_log_verbosity_level::set_log_verbosity_level;

mod get_log_verbosity_level;
pub use get_log_verbosity_level::get_log_verbosity_level;

mod get_log_tags;
pub use get_log_tags::get_log_tags;

mod set_log_tag_verbosity_level;
pub use set_log_tag_verbosity_level::set_log_tag_verbosity_level;

mod get_log_tag_verbosity_level;
pub use get_log_tag_verbosity_level::get_log_tag_verbosity_level;

mod add_log_message;
pub use add_log_message::add_log_message;

mod get_user_support_info;
pub use get_user_support_info::get_user_support_info;

mod set_user_support_info;
pub use set_user_support_info::set_user_support_info;

mod get_support_name;
pub use get_support_name::get_support_name;

mod test_call_empty;
pub use test_call_empty::test_call_empty;

mod test_call_string;
pub use test_call_string::test_call_string;

mod test_call_bytes;
pub use test_call_bytes::test_call_bytes;

mod test_call_vector_int;
pub use test_call_vector_int::test_call_vector_int;

mod test_call_vector_int_object;
pub use test_call_vector_int_object::test_call_vector_int_object;

mod test_call_vector_string;
pub use test_call_vector_string::test_call_vector_string;

mod test_call_vector_string_object;
pub use test_call_vector_string_object::test_call_vector_string_object;

mod test_square_int;
pub use test_square_int::test_square_int;

mod test_network;
pub use test_network::test_network;

mod test_proxy;
pub use test_proxy::test_proxy;

mod test_get_difference;
pub use test_get_difference::test_get_difference;

mod test_use_update;
pub use test_use_update::test_use_update;

mod test_return_error;
pub use test_return_error::test_return_error;
