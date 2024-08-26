use std::str::FromStr;

use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, throw_str, UnwrapThrowExt};

use super::rarity::Rarity;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    name: String,
    rarity: Rarity,
    drop_chance: f32,
}

#[wasm_bindgen]
impl Item {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn rarity(&self) -> Rarity {
        self.rarity.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn drop_chance(&self) -> f32 {
        self.drop_chance
    }
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

    const RECONSTRUCTION_NAMES: [&str; 2] = ["Sevago", "Li"];

    pub fn reconstruct_item_name(&mut self) {
        let el = Item::RECONSTRUCTION_NAMES
            .iter()
            .find(|e| self.name.contains(*e));
        if el.is_some() {
            let el = el.expect_throw("Should have not throw");
            let idx = self.name.find(el).expect_throw("Should have not throw") + el.len();
            let (first, last) = self.name.split_at(idx);
            let result = format!("{}{}{}", first, "th", last);
            self.name = result;
        }
    }
}
