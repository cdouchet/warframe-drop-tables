#![feature(string_remove_matches)]
#![feature(let_chains)]
use core::str;

use models::categories::missions::Missions;
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use wasm_bindgen::{prelude::*, throw_str};

pub mod macros;
pub mod models;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn parse_warframe_data(data: &str) -> JsValue {
    let mut reader = Reader::from_str(data);
    reader.config_mut().trim_text(true);

    let spl = data.split("</ul>").collect::<Vec<&str>>();

    let body = match spl.iter().nth(2) {
        Some(body) => body,
        None => {
            console_log!("Big bug");
            warframe_parse_error!();
            throw_str("Failed to parse warframe base data");
        }
    };

    // let tables_content = body
    //     .split("</table>")
    //     .map(|e| {
    //         let mut splitted = e.split("<table>");
    //         let el = &splitted.nth(1);
    //         console_log!(
    //             "Exists Len: {}",
    //             splitted.clone().collect::<Vec<&str>>().len()
    //         );
    //         let exists = el.is_some();
    //         console_log!("Exists: {}", &exists);
    //         // if exists {
    //         //     console_log!("Exists data: {}", el.unwrap());
    //         // }
    //         el.unwrap_or_else(|| splitted.nth(0).unwrap()).to_string()
    //     })
    //     .collect::<Vec<String>>();

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

    // for (i, el) in tables_content.iter().enumerate() {
    //     console_log!("Index is : {}", i);
    //     if i % 2 == 1 {
    //         console_log!("{}", el);
    //     }
    // }

    let missions = tables_content.first().unwrap();
    let missions = Missions::parse(missions);
    serde_wasm_bindgen::to_value(&missions).expect_throw("Failed to send final data")

    // loop {
    //     match reader.read_event() {
    //         Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
    //         Ok(Event::Eof) => break,
    //         Ok(Event::Start(e)) => {
    //             let name = str::from_utf8(e.name().0).unwrap();
    //             console_log!("{}", name);
    //         }
    //         _ => (),
    //     }
    // }
}
