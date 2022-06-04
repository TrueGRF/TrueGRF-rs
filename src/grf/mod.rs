mod actions;
mod config;
mod industry;
mod rpn;
mod townname;

pub use config::{
    NewGRFConfig,
    NewGRFConfigIndustry,
    NewGRFConfigTownname,
    NewGRFGeneral,
    NewGRFSprite,
    NewGRFSpriteContainer,
    NewGRFTownnamePart,
};

use industry::write_industry_segments;
use townname::write_townname_segments;

pub struct Output<'a> {
    buffer: Vec<u8>,
    string_counter: u16,
    sprites: Vec<Vec<u8>>,
    load_sprite_bytes: &'a dyn Fn(&str) -> Vec<u8>,
}

pub fn write_grf(options: NewGRFConfig, load_sprite_bytes: &dyn Fn(&str) -> Vec<u8>) -> Result<Vec<u8>, String> {
    let mut output = Output { buffer: Vec::new(), string_counter: 0xdc00, sprites: Vec::new(), load_sprite_bytes };

    match options {
        NewGRFConfig::industry(options) => write_industry_segments(&mut output, options)?,
        NewGRFConfig::townname(options) => write_townname_segments(&mut output, options)?,
    };

    let mut grf = Vec::new();
    /* Write GRF container version 2 header. */
    grf.extend(b"\x00\x00GRF\x82\r\n\x1a\n");
    /* Sprite section offset (+ 9 for initial sprite and + 4 end-of-data-section marker). */
    grf.extend((output.buffer.len() as u32 + 1 + 9 + 4).to_le_bytes());
    /* Compression. OpenTTD currently only support no-compression (= 0). */
    grf.extend(b"\x00");

    /* Initial sprite; should be 4 in length, ignored by OpenTTD. */
    grf.extend(b"\x04\x00\x00\x00\xff\x02\x00\x00\x00");
    /* Add data-section. */
    grf.extend(output.buffer);
    /* End-of-data-section marker. */
    grf.extend(b"\x00\x00\x00\x00");

    /* Add all sprites to sprite-section. */
    for sprite in output.sprites {
        grf.extend(sprite);
    }
    /* End-of-sprite-section marker. */
    grf.extend(b"\x00\x00\x00\x00");

    Ok(grf)
}
