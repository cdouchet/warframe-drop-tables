use serde::Serialize;
use wasm_bindgen::throw_str;

use crate::models::item::Item;

#[derive(Serialize)]
pub struct Rotations {
    pub a: Vec<Item>,
    pub b: Vec<Item>,
    pub c: Vec<Item>,
}

impl Rotations {
    pub fn name_from_counter(c: &u8) -> String {
        match c {
            0 => "Rotation A",
            1 => "Rotation B",
            2 => "Rotation C",
            _ => throw_str("Invalid counter in rotation parsing"),
        }
        .to_string()
    }
}
