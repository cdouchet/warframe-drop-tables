use core::str;

use mission::Mission;
use quick_xml::{events::Event, Reader};
use rotations::Rotations;
use serde::Serialize;

use crate::{console_log, parse_xml_name};

pub mod mission;
pub mod rotations;

use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct Missions(Vec<Mission>);

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

        // console_log!("Raw missions length: {}", raw_missions.len());
        // for r in &raw_missions {
        //     console_log!("Raw mission: {}", &r);
        // }
        let mut missions = Vec::<Mission>::new();

        for raw_mission in raw_missions {
            missions.push(Mission::parse(&raw_mission));
        }

        Self(missions)

        // 'outer: loop {
        //     match reader.read_event() {
        //         Err(err) => panic!("Failed to read xml in Missions::parse"),
        //         Ok(Event::Eof) => break,
        //         Ok(Event::Start(s)) => {
        //             let mut counter: u32 = 0;
        //             let mission: Mission;
        //             'inner: loop {
        //                 match reader.read_event() {
        //                     Ok(Event::Start(t)) => {
        //                         if t.attributes().any(|e| {
        //                             let e = e.unwrap();
        //                             e.value.as_ref() == b"blank-row"
        //                         }) {
        //                             loop {
        //                                 match reader.read_event() {
        //                                     Err(err) => panic!("Parsing error third step"),
        //                                     Ok(Event::Eof) => break 'outer,
        //                                     Ok(Event::End(t)) => {
        //                                         if t.name().as_ref() == b"tr" {
        //                                             break 'inner;
        //                                         }
        //                                     }
        //                                     _ => {}
        //                                 }
        //                             }
        //                         } else if t.name().as_ref() == b"tr" {
        //                             continue 'inner;
        //                         } else {
        //                             let name = t.name().as_ref();
        //                             if counter == 0 {
        //                                 if let Event::Text(t) = reader.read_event().unwrap() {}
        //                             }
        //                         }
        //                     }
        //                     Ok(Event::Eof) => break 'outer,
        //                     Err(err) => panic!("Parsing error second step"),
        //                     _ => {}
        //                 }
        //                 console_log!("Start: {}", parse_xml_name!(s));
        //             }
        //         }
        //         Ok(Event::End(s)) => {
        //             // console_log!("End: {}", parse_xml_name!(s));
        //         }
        //         Ok(Event::Text(t)) => {
        //             // console_log!("Some text: {:?}", t);
        //         }
        //         _ => {}
        //     }
        // }
        // Self(Vec::new())
    }
}
