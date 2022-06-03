use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFGeneral {
    pub version: u32,
    pub grfid: String,
    pub name: String,
    pub url: String,
    pub description: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFSprite {
    pub filename: String,
    pub left: i16,
    pub top: i16,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFDefaultSprite {
    pub id: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NewGRFSpriteContainer {
    Sprite(NewGRFSprite),
    DefaultSprite(NewGRFDefaultSprite),
}
impl Default for NewGRFSpriteContainer {
    fn default() -> Self { NewGRFSpriteContainer::DefaultSprite(NewGRFDefaultSprite { id: 0 } ) }
}

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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NewGRFOptions {
    pub general: NewGRFGeneral,
    pub cargoes: Vec<NewGRFCargo>,
    pub industries: Vec<NewGRFIndustry>,
}
