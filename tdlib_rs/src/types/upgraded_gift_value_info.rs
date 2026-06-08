#[allow(clippy::all)]
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// Contains information about value of an upgraded gift
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct UpgradedGiftValueInfo {
    /// ISO 4217 currency code of the currency in which the prices are represented
    pub currency: String,
    /// Estimated value of the gift; in the smallest units of the currency
    pub value: i64,
    /// True, if the value is calculated as average value of similar sold gifts. Otherwise, it is based on the sale price of the gift
    pub is_value_average: bool,
    /// Point in time (Unix timestamp) when the corresponding regular gift was originally purchased
    pub initial_sale_date: i32,
    /// The Telegram Star amount that was paid for the gift
    pub initial_sale_star_count: i64,
    /// Initial price of the gift; in the smallest units of the currency
    pub initial_sale_price: i64,
    /// Point in time (Unix timestamp) when the upgraded gift was purchased last time; 0 if never
    pub last_sale_date: i32,
    /// Last purchase price of the gift; in the smallest units of the currency; 0 if the gift has never been resold
    pub last_sale_price: i64,
    /// True, if the last sale was completed on Fragment
    pub is_last_sale_on_fragment: bool,
    /// The current minimum price of gifts upgraded from the same gift; in the smallest units of the currency; 0 if there are no such gifts
    pub minimum_price: i64,
    /// The average sale price in the last month of gifts upgraded from the same gift; in the smallest units of the currency; 0 if there were no such sales
    pub average_sale_price: i64,
    /// Number of gifts upgraded from the same gift being resold on Telegram
    pub telegram_listed_gift_count: i32,
    /// Number of gifts upgraded from the same gift being resold on Fragment
    pub fragment_listed_gift_count: i32,
    /// The HTTPS link to the Fragment for the gift; may be empty if there are no such gifts being sold on Fragment
    pub fragment_url: String,
}
