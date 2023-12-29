use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

use super::Rect;

#[derive(Component, ConvertSaveload)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component, ConvertSaveload)]
pub struct Building {
    pub rect: Rect,
}

#[derive(Component, ConvertSaveload, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, ConvertSaveload)]
pub struct PlayerStats {
    pub food_amount: i32,
    pub food_amount_max: i32,
    pub food_generation_rate: i32, // total food/sec
    pub next_refresh: i64,         // second
}

#[derive(Component, ConvertSaveload)]
pub struct FoodGenerator {
    pub rate: i32, // food/sec
}
