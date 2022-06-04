use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewGRFTownnameName {
    pub name: String,
    pub probability: u8,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewGRFTownnamePart {
    pub names: Vec<NewGRFTownnameName>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewGRFTownnameSet {
    pub name: String,
    pub parts: Vec<NewGRFTownnamePart>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewGRFTownname {
    pub available: bool,
    pub name: String,
    pub mainset: Vec<NewGRFTownnamePart>,
    pub subsets: Vec<NewGRFTownnameSet>,
}
