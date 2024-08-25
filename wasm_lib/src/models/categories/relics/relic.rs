use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, UnwrapThrowExt};

use crate::models::item::Item;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct Relic {
    name: String,
    items: Vec<Item>,
}

#[wasm_bindgen]
impl Relic {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Vec<Item> {
        self.items.clone()
    }
}

impl Relic {
    pub fn parse(input: &str) -> Self {
        let mut spl = input
            .split("><")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        spl.iter_mut().for_each(|e| {
            e.remove_matches("<th colspan=\"2\"");
            e.remove_matches("th colspan=\"2\"");
            e.remove_matches("td>");
            e.remove_matches("</td");
            e.remove_matches("</");
            e.remove_matches(">");
            e.remove_matches("th");
        });
        let mut iter = spl.iter();
        let name = iter.next().unwrap();
        let mut items: Vec<Item> = Vec::new();
        while let Some(raw_name) = iter.next() {
            let raw_chance = iter.next().expect_throw("Failed to parse item chance");
            items.push(Item::from_name_and_chance(raw_name, raw_chance));
        }
        Self {
            name: name.to_string(),
            items,
        }
    }
}
