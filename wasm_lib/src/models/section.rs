use serde::Serialize;

use super::{item::Item, title::Subtitle};

#[derive(Serialize)]
pub struct Section {
    pub title: Subtitle,
    pub items: Vec<Item>,
}
