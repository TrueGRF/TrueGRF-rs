use serde::{Serialize, Deserialize};

use super::sprite::NewGRFSprite;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFCargo {
    pub id: u8,
    pub available: bool,
    pub name: String,
    pub longName: String,
    pub unitName: String,
    pub label: String,
    pub abbreviation: String,
    pub classes: u16,
    pub weight: u8,
    pub colour: u8,
    pub penaltyLowerBound: u8,
    pub penaltyLength: u8,
    pub price: u32,
    pub sprite: NewGRFSprite,
}
