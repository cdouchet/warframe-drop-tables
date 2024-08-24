use core::str;

use mission::Mission;
use quick_xml::{events::Event, Reader};
use rotations::Rotations;
use serde::{Deserialize, Serialize};

use crate::{models::item::Item, parse_xml_name};

pub mod mission;
pub mod rotations;

use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Missions(pub Vec<Mission>);

impl Missions {
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

        Self(missions)
    }

    pub fn filter(&self, f: &str) -> Vec<&Mission> {
        let mut result = self
            .0
            .iter()
            .filter(|e| match e {
                Mission::Classic { name, items } => items
                    .iter()
                    .any(|item| item.name.to_lowercase().contains(f)),
                Mission::Rotation { name, rotations } => rotations
                    .a
                    .iter()
                    .chain(rotations.b.iter())
                    .chain(rotations.c.iter())
                    .any(|item| item.name.to_lowercase().contains(f)),
            })
            .collect::<Vec<&Mission>>();
        result.sort_by(|a, b| {
            let item_a = find_first_item(a, f);
            let item_b = find_first_item(b, f);
            item_b.drop_chance.total_cmp(&item_a.drop_chance)
        });
        result
    }
}

fn find_first_item<'a>(mission: &&'a Mission, f: &str) -> &'a Item {
    match mission {
        Mission::Classic { name, items } => items
            .iter()
            .find(|e| e.name.to_lowercase().contains(f))
            .expect_throw("Fail"),
        Mission::Rotation { name, rotations } => rotations
            .a
            .iter()
            .chain(rotations.b.iter())
            .chain(rotations.c.iter())
            .find(|e| e.name.to_lowercase().contains(f))
            .expect_throw("Failed"),
    }
}
