use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

use lazy_static::lazy_static;
use std::error::Error;

lazy_static! {
    pub static ref MORANDI_RED: RGB = RGB::from_u8(185, 87, 86);
}
