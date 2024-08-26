use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, throw_str, UnwrapThrowExt};

use crate::models::item::Item;

use super::rotations::Rotations;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub enum MissionType {
    Rotation,
    Classic,
    // Rotation { name: String, rotations: Rotations },
    // Classic { name: String, items: Vec<Item> },
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Mission {
    mission_type: MissionType,
    name: String,
    rotation_data: Option<Rotations>,
    classic_data: Option<Vec<Item>>,
}

#[wasm_bindgen]
impl Mission {
    /// Get classic mission data. Panics if mission type is Rotations
    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Vec<Item> {
        self.classic_data
            .clone()
            .expect_throw("Unwraped classic data on a rotation mission")
    }

    /// Get rotations mission data. Panics if mission type is classic
    #[wasm_bindgen(getter)]
    pub fn rotations(&self) -> Rotations {
        self.rotation_data
            .clone()
            .expect_throw("Unwraped rotations data on a classic mission")
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn mission_type(&self) -> MissionType {
        self.mission_type.clone()
    }
}

impl Mission {
    fn parse_rotation(input: Vec<String>) -> Self {
        let mut iter = input.iter().peekable();
        let name = iter
            .next()
            .expect_throw("Failed to parse mission name in rotation");
        let mut counter: u8 = 0;
        let mut a: Vec<Item> = Vec::new();
        let mut b: Vec<Item> = Vec::new();
        let mut c: Vec<Item> = Vec::new();
        while let Some(rot) = iter.next() {
            if rot.contains("Rotation") {
                let mut items: Vec<Item> = Vec::new();
                while let Some(el) = iter.peek()
                    && !el.contains("Rotation")
                {
                    let item_name = iter.next().expect_throw("Failed to get item name");
                    let item_chance = iter.next().expect_throw("Failed to get item chance");

                    // let el = iter.next().unwrap();
                    // let mut spl = el
                    //     .split("</td>")
                    //     .map(|e| e.to_string())
                    //     .collect::<Vec<String>>();
                    // spl.iter_mut().for_each(|e| {
                    //     e.remove_matches("<td>");
                    // });
                    // let raw_name = spl.get(0).expect_throw("Failed to parse item name");
                    // let raw_chance = spl.get(1).expect_throw("Failed to parse item chance");
                    items.push(Item::from_name_and_chance(item_name, item_chance));
                }
                match counter {
                    0 => a = items,
                    1 => b = items,
                    2 => c = items,
                    _ => throw_str(&format!("Invalid counter: {counter}")),
                }
                counter += 1;
            }
        }
        Self {
            mission_type: MissionType::Rotation,
            name: name.to_string(),
            classic_data: None,
            rotation_data: Some(Rotations::new(a, b, c)),
        }
    }

    fn parse_classic(input: Vec<String>) -> Self {
        let mut iter = input.iter();
        let name = iter.next().unwrap();
        let mut items: Vec<Item> = Vec::new();
        while let Some(raw_name) = iter.next() {
            let raw_chance = iter.next().expect_throw("Failed to parse item chance");
            items.push(Item::from_name_and_chance(raw_name, raw_chance));
        }
        // while let Some(el) = iter.next() {
        //     let mut spl = el
        //         .split("</td>")
        //         .map(|e| e.to_string())
        //         .collect::<Vec<String>>();
        //     spl.iter_mut().for_each(|e| {
        //         e.remove_matches("<td>");
        //     });
        //     let raw_name = spl.get(0).expect_throw("Failed to parse item name");
        //     let raw_chance = spl.get(1).expect_throw("Failed to parse item chance");
        //     items.push(Item::from_name_and_chance(raw_name, raw_chance));
        // }
        Self {
            mission_type: MissionType::Classic,
            name: name.to_string(),
            rotation_data: None,
            classic_data: Some(items),
        }
    }

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
        match input.contains("Rotation") {
            true => Self::parse_rotation(spl),
            false => Self::parse_classic(spl),
        }
    }

    pub fn reconstruct_missing_names(&mut self) {
        match self.mission_type {
            MissionType::Classic => {
                if let Some(ref mut data) = self.classic_data {
                    for item in data {
                        (*item).reconstruct_item_name();
                    }
                }
                // for data in self
                //     .classic_data
                //     .expect_throw("Failed to unwrap classic data in name reconstruction")
                // {
                //     data.reconstruct_item_name();
                // }
            }
            MissionType::Rotation => {
                if let Some(ref mut rotations) = self.rotation_data {
                    rotations.reconstruct_item_name();
                }
            }
        }
    }
}
