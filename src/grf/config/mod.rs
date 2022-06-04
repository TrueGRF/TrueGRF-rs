use serde::{Serialize, Deserialize};

mod cargo;
mod general;
mod industry;
mod sprite;
mod townname;

pub use general::NewGRFGeneral;
pub use sprite::{
    NewGRFSprite,
    NewGRFSpriteContainer,
};
pub use townname::NewGRFTownnamePart;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewGRFConfigIndustry {
    pub general: NewGRFGeneral,
    pub cargoes: Vec<cargo::NewGRFCargo>,
    pub industries: Vec<industry::NewGRFIndustry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewGRFConfigTownname {
    pub general: NewGRFGeneral,
    pub townnames: Vec<townname::NewGRFTownname>,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NewGRFConfig {
    industry(NewGRFConfigIndustry),
    townname(NewGRFConfigTownname),
}
