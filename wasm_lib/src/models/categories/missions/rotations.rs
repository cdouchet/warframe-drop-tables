use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, throw_str};

use crate::models::item::Item;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct Rotations {
    a: Vec<Item>,
    b: Vec<Item>,
    c: Vec<Item>,
}

#[wasm_bindgen]
impl Rotations {
    #[wasm_bindgen(constructor)]
    pub fn new(a: Vec<Item>, b: Vec<Item>, c: Vec<Item>) -> Rotations {
        Rotations { a, b, c }
    }

    #[wasm_bindgen(getter)]
    pub fn a(&self) -> Vec<Item> {
        self.a.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn b(&self) -> Vec<Item> {
        self.b.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn c(&self) -> Vec<Item> {
        self.c.clone()
    }
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
