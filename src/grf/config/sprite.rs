use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewGRFSprite {
    pub filename: String,
    pub left: i16,
    pub top: i16,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
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
