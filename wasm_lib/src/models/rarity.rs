use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Rarity {
    VeryCommon,
    Common,
    Uncommon,
    Rare,
    UltraRare,
    Legendary,
    BeyondLegendary,
}

impl FromStr for Rarity {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Very Common" => Self::VeryCommon,
            "Common" => Self::Common,
            "Uncommon" => Self::Uncommon,
            "Rare" => Self::Rare,
            "Ultra Rare" => Self::UltraRare,
            "Legendary" => Self::Legendary,
            "Beyond Legendary" => Self::BeyondLegendary,
            _ => return Err("Invalid Input for rarity".into()),
        })
    }
}
