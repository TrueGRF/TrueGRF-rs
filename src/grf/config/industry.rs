use serde::{Serialize, Deserialize};

use super::sprite::NewGRFSpriteContainer;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFIndustryTileSprite {
    pub sprite: NewGRFSpriteContainer,
    pub drawType: String,
    pub alwaysDraw: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFIndustryTile {
    pub sprites: Vec<NewGRFIndustryTileSprite>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFIndustry {
    pub id: u8,
    pub available: bool,
    pub name: String,
    pub r#type: String,
    pub fundCostMultiplier: u8,
    pub probabilityMapGen: u8,
    pub probabilityInGame: u8,
    pub colour: u8,
    pub prospectChance: u8,  // Scale from 0 to 100
    pub layout: Vec<Vec<Vec<i32>>>,
    pub cargoAcceptance: Vec<String>,
    pub cargoProduction: Vec<String>,
    pub placement: String,
    pub tiles: Vec<NewGRFIndustryTile>,
    pub callbacks: String,
}
