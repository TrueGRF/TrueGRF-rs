use serde::{Serialize, Deserialize};

mod cargo;
mod general;
mod industry;
mod sprite;

pub use sprite::{
    NewGRFSprite,
    NewGRFSpriteContainer,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFConfig {
    pub general: general::NewGRFGeneral,
    pub cargoes: Vec<cargo::NewGRFCargo>,
    pub industries: Vec<industry::NewGRFIndustry>,
}
