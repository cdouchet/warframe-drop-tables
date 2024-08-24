use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Title(pub String);
#[derive(Serialize, Deserialize)]
pub struct Subtitle(pub String);
