use serde::Serialize;

use super::{section::Section, title::Title};

#[derive(Serialize)]
pub struct Sheet {
    pub title: Title,
    pub sections: Vec<Section>,
}
