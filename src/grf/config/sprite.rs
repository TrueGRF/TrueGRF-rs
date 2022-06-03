use serde::{Serialize, Deserialize};

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
