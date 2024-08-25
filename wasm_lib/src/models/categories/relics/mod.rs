pub mod relic;

use wasm_bindgen::prelude::*;

use relic::Relic;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct Relics {
    inner: Vec<Relic>,
}

impl Relics {
    pub fn inner(&self) -> Vec<Relic> {
        self.inner.clone()
    }

    pub fn parse(input: &str) -> Self {
        let mut input = String::from(input);
        input.remove_matches("<tr>");
        input.remove_matches("</tr>");
        let raw_relics = input
            .split("<tr class=\"blank-row\"><td class=\"blank-row\" colspan=\"2\"></td>")
            .collect::<Vec<&str>>();
        let raw_relics = raw_relics
            .iter()
            .filter(|e| e.contains("(Radiant)"))
            .map(|e| *e)
            .collect::<Vec<&str>>();
        let raw_relics = raw_relics
            .iter()
            .map(|e| {
                e.chars()
                    .filter(|&c| c != '\u{000D}' && c != '\u{000A}')
                    .collect::<String>()
            })
            .collect::<Vec<String>>();

        let relics = raw_relics.iter().map(|e| Relic::parse(e)).collect();
        Self { inner: relics }
    }

    pub fn filter(&self, f: &str) -> Relics {
        let result = self
            .inner
            .iter()
            .filter(|e| {
                e.items()
                    .iter()
                    .any(|item| item.name().to_lowercase().contains(f))
            })
            .map(|e| e.clone())
            .collect::<Vec<Relic>>();
        Relics { inner: result }
    }
}
