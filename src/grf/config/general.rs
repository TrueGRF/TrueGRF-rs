use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFGeneral {
    pub version: u32,
    pub grfid: String,
    pub name: String,
    pub url: String,
    pub description: String,
}
