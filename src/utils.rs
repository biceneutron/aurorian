use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

use std::error::Error;

pub fn str_to_rgb(color: &str) -> Option<RGB> {
    let result = match color {
        "GOLD2" => Some(RGB::named(rltk::GOLD2)),
        "LIME" => Some(RGB::named(rltk::LIME)),
        "RED" => Some(RGB::named(rltk::RED)),
        "BLACK" => Some(RGB::named(rltk::BLACK)),
        _ => None,
    };

    result
}
