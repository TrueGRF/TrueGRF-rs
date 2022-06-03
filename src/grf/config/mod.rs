use serde::{Serialize, Deserialize};

mod cargo;
mod general;
mod industry;
mod sprite;

pub use general::NewGRFGeneral;
pub use sprite::{
    NewGRFSprite,
    NewGRFSpriteContainer,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewGRFConfigIndustry {
    pub general: NewGRFGeneral,
    pub cargoes: Vec<cargo::NewGRFCargo>,
    pub industries: Vec<industry::NewGRFIndustry>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NewGRFConfig {
    industry(NewGRFConfigIndustry),
}
