use missions::{mission::Mission, Missions};
use relics::{relic::Relic, Relics};
use wasm_bindgen::{prelude::*, throw_str};

use crate::{console_log, warframe_parse_error};

pub mod missions;
pub mod relics;

#[wasm_bindgen]
pub struct WarframeData {
    missions: Missions,
    relics: Relics,
}

#[wasm_bindgen]
impl WarframeData {
    #[wasm_bindgen(constructor)]
    pub fn new(data: &str) -> WarframeData {
        let spl = data.split("</ul>").collect::<Vec<&str>>();

        let body = match spl.iter().nth(2) {
            Some(body) => body,
            None => {
                console_log!("Big bug");
                warframe_parse_error!();
                throw_str("Failed to parse warframe base data");
            }
        };

        let tables_content = body
            .split("</table>")
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        let tables_content = tables_content
            .iter()
            .map(|e| {
                let spl = e.split("<table>").collect::<Vec<&str>>();
                let el = spl.iter().nth(1);
                el.unwrap_or(spl.iter().nth(0).unwrap()).to_string()
            })
            .collect::<Vec<String>>();

        let missions = tables_content
            .first()
            .expect_throw("Failed to get raw mission data");
        let missions = Missions::parse(missions);
        let relics = tables_content
            .get(1)
            .expect_throw("Failed to get raw relics data");
        let relics = Relics::parse(relics);
        let mut warframe_data = WarframeData { missions, relics };
        warframe_data.reconstruct_missing_names();
        warframe_data
    }

    #[wasm_bindgen]
    pub fn filter(&self, input: &str) -> WarframeData {
        let missions = self.missions.filter(input);
        let relics = self.relics.filter(input);
        WarframeData { missions, relics }
    }

    #[wasm_bindgen(getter)]
    pub fn missions(&self) -> Vec<Mission> {
        self.missions.inner()
    }

    #[wasm_bindgen(setter)]
    pub fn set_missions(&mut self, missions: Missions) {
        self.missions = missions;
    }

    #[wasm_bindgen(getter)]
    pub fn relics(&self) -> Vec<Relic> {
        self.relics.inner()
    }

    #[wasm_bindgen(setter)]
    pub fn set_relics(&mut self, relics: Relics) {
        self.relics = relics;
    }
}

impl WarframeData {
    fn reconstruct_missing_names(&mut self) {
        self.missions.reconstruct_missing_names();
        self.relics.reconstruct_item_name();
    }
}
