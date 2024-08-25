#![feature(string_remove_matches)]
#![feature(let_chains)]
use core::str;

use wasm_bindgen::prelude::*;

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

// #[wasm_bindgen]
// pub fn parse_warframe_data(data: &str) -> JsValue {
//     let spl = data.split("</ul>").collect::<Vec<&str>>();

//     let body = match spl.iter().nth(2) {
//         Some(body) => body,
//         None => {
//             console_log!("Big bug");
//             warframe_parse_error!();
//             throw_str("Failed to parse warframe base data");
//         }
//     };

//     let tables_content = body
//         .split("</table>")
//         .map(|e| e.to_string())
//         .collect::<Vec<String>>();

//     let tables_content = tables_content
//         .iter()
//         .map(|e| {
//             let spl = e.split("<table>").collect::<Vec<&str>>();
//             let el = spl.iter().nth(1);
//             el.unwrap_or(spl.iter().nth(0).unwrap()).to_string()
//         })
//         .collect::<Vec<String>>();

//     let missions = tables_content
//         .first()
//         .expect_throw("Failed to get raw mission data");
//     let missions = Missions::parse(missions);
//     let relics = tables_content
//         .get(1)
//         .expect_throw("Failed to get raw relics data");
//     let relics = Relics::parse(relics);

//     let warframe_data = WarframeData { missions, relics };

//     serde_wasm_bindgen::to_value(&warframe_data).expect_throw("Failed to send final data")
// }

// #[wasm_bindgen]
// pub fn filter_warframe_missions(missions: JsValue, filter: &str) -> JsValue {
//     let warframe_data: WarframeData =
//         serde_wasm_bindgen::from_value(missions).expect_throw("Expected an array of missions");
//     let result = &missions.filter(&filter.to_lowercase());
//     serde_wasm_bindgen::to_value(&result).expect_throw("Failed to send filtered data")
// }
