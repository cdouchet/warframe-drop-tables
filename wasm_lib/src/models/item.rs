use std::str::FromStr;

use serde::Serialize;
use wasm_bindgen::{throw_str, UnwrapThrowExt};

use crate::console_log;

use super::rarity::Rarity;

#[derive(Serialize)]
pub struct Item {
    name: String,
    rarity: Rarity,
    drop_chance: f32,
}

impl Item {
    pub fn from_name_and_chance(name: &String, chance: &String) -> Self {
        let spl = chance
            .split(" (")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        let rarity = Rarity::from_str(spl.get(0).expect_throw("Failed to parsed item rarity"))
            .expect_throw("Invalid item rarity given");
        let mut raw_chance = spl
            .get(1)
            .expect_throw("Failed to parse item chance")
            .to_string();
        // raw_chance.remove_matches("(");
        raw_chance.remove_matches(")");
        raw_chance.remove_matches("%");
        raw_chance.remove_matches(" ");
        let raw_chance: String = raw_chance
            .chars()
            .filter(|&c| c != '\u{000D}' && c != '\u{000A}')
            .collect();
        // let raw_chance = raw_chance.trim_matches(char::from(0));
        let chance = match raw_chance.parse::<f32>() {
            Ok(e) => e,
            Err(err) => {
                throw_str(&format!(
                    "Raw chance should have been a floating point. Err: {:?}",
                    err
                ));
            }
        };
        // let chance = raw_chance
        //     .parse::<f32>()
        //     .expect_throw("Raw chance should have been a floating point");
        Item {
            name: name.to_string(),
            rarity,
            drop_chance: chance,
        }
    }
}
