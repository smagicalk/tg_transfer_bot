#[allow(clippy::all)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum AffiliateProgramSortOrder {
    /// The affiliate programs must be sorted by the profitability
    #[serde(rename(
        serialize = "affiliateProgramSortOrderProfitability",
        deserialize = "affiliateProgramSortOrderProfitability"
    ))]
    Profitability,
    /// The affiliate programs must be sorted by creation date
    #[serde(rename(
        serialize = "affiliateProgramSortOrderCreationDate",
        deserialize = "affiliateProgramSortOrderCreationDate"
    ))]
    CreationDate,
    /// The affiliate programs must be sorted by the expected revenue
    #[serde(rename(
        serialize = "affiliateProgramSortOrderRevenue",
        deserialize = "affiliateProgramSortOrderRevenue"
    ))]
    Revenue,
}
