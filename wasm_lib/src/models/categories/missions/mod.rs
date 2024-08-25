use core::str;

use mission::{Mission, MissionType};

use crate::models::item::Item;

pub mod mission;
pub mod rotations;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Missions {
    inner: Vec<Mission>,
}

impl Missions {
    pub fn inner(&self) -> Vec<Mission> {
        self.inner.clone()
    }

    pub fn parse(input: &str) -> Self {
        let mut input = String::from(input);
        input.remove_matches("<tr>");
        input.remove_matches("</tr>");
        let raw_missions = input
            .split("<tr class=\"blank-row\"><td class=\"blank-row\" colspan=\"2\"></td>")
            .collect::<Vec<&str>>();
        let raw_missions = raw_missions
            .iter()
            .map(|e| {
                e.chars()
                    .filter(|&c| c != '\u{000D}' && c != '\u{000A}')
                    .collect::<String>()
            })
            .collect::<Vec<String>>();

        let mut missions = Vec::<Mission>::new();

        for raw_mission in raw_missions {
            missions.push(Mission::parse(&raw_mission));
        }

        Self { inner: missions }
    }

    pub fn filter(&self, f: &str) -> Missions {
        let mut result = self
            .inner
            .iter()
            .filter(|e| match e.mission_type() {
                MissionType::Classic => e
                    .items()
                    .iter()
                    .any(|item| item.name().to_lowercase().contains(f)),
                MissionType::Rotation => {
                    let rotations = e.rotations();
                    rotations
                        .a()
                        .iter()
                        .chain(rotations.b().iter())
                        .chain(rotations.c().iter())
                        .any(|item| item.name().to_lowercase().contains(f))
                }
            })
            .map(|e| e.clone())
            .collect::<Vec<Mission>>();
        result.sort_by(|a, b| {
            let item_a = find_first_item(a, f);
            let item_b = find_first_item(b, f);
            item_b.drop_chance().total_cmp(&item_a.drop_chance())
        });
        Missions { inner: result }
    }
}

fn find_first_item<'a>(mission: &'a Mission, f: &str) -> Item {
    match mission.mission_type() {
        MissionType::Classic => mission
            .items()
            .iter()
            .find(|e| e.name().to_lowercase().contains(f))
            .expect_throw("Failed")
            .clone(),
        MissionType::Rotation => {
            let rotations = mission.rotations();
            rotations
                .a()
                .iter()
                .chain(rotations.b().iter())
                .chain(rotations.c().iter())
                .find(|e| e.name().to_lowercase().contains(f))
                .expect_throw("Fail")
                .clone()
        }
    }
}
