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
    pub level: i32,
}

#[derive(Component, ConvertSaveload, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, ConvertSaveload)]
pub struct PlayerStats {
    pub food: ResourceInfo,
    pub wood: ResourceInfo,
    pub stone: ResourceInfo,
    pub next_refresh: i64, // second
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceInfo {
    pub amount: i32,
    pub max_amount: i32,
    pub rate: i32, // per sec
}

#[derive(PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum ResourceType {
    Food,
    Stone,
    Wood,
}

#[derive(Component, ConvertSaveload)]
pub struct Generator {
    pub rate: i32, // per sec
    pub resource_type: ResourceType,
}
