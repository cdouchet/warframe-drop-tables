use std::str::FromStr;

use serde::Serialize;
use wasm_bindgen::{throw_str, UnwrapThrowExt};

use crate::{console_log, models::item::Item};

use super::rotations::Rotations;

#[derive(Serialize)]
pub enum Mission {
    Rotation { name: String, rotations: Rotations },
    Classic { name: String, items: Vec<Item> },
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
        Self::Rotation {
            name: name.to_string(),
            rotations: Rotations { a, b, c },
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
        Self::Classic {
            name: name.to_string(),
            items,
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
}
