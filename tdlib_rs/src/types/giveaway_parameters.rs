#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


/// Describes parameters of a giveaway
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct GiveawayParameters {
    /// Identifier of the supergroup or channel chat, which will be automatically boosted by the winners of the giveaway for duration of the Telegram Premium subscription,
    /// or for the specified time. If the chat is a channel, then can_post_messages administrator right is required in the channel, otherwise, the user must be an administrator in the supergroup
    pub boosted_chat_id: i64,
    /// Identifiers of other supergroup or channel chats that must be subscribed by the users to be eligible for the giveaway. There can be up to getOption("giveaway_additional_chat_count_max") additional chats
    pub additional_chat_ids: Vec<i64>,
    /// Point in time (Unix timestamp) when the giveaway is expected to be performed; must be 60-getOption("giveaway_duration_max") seconds in the future in scheduled giveaways
    pub winners_selection_date: i32,
    /// True, if only new members of the chats will be eligible for the giveaway
    pub only_new_members: bool,
    /// True, if the list of winners of the giveaway will be available to everyone
    pub has_public_winners: bool,
    /// The list of two-letter ISO 3166-1 alpha-2 codes of countries, users from which will be eligible for the giveaway. If empty, then all users can participate in the giveaway.
    /// There can be up to getOption("giveaway_country_count_max") chosen countries. Users with phone number that was bought at https:fragment.com can participate in any giveaway and the country code "FT" must not be specified in the list
    pub country_codes: Vec<String>,
    /// Additional description of the giveaway prize; 0-128 characters
    pub prize_description: String,
}
