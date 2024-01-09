use super::components::*;
use rltk::RGB;
use serde::{Deserialize, Serialize};
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

use lazy_static::lazy_static;
use std::error::Error;

use crate::{BuildingDetail, PlayerStats, PlayerStatsSaveloadData};

lazy_static! {
    pub static ref MORANDI_RED: RGB = RGB::from_u8(185, 87, 86);
}

pub fn requirements_check(
    stats: &PlayerStats,
    building: Option<&Building>,
    detail: &BuildingDetail,
    next_level: i32,
) -> bool {
    if next_level >= detail.levels.len() as i32 {
        return false;
    }

    if let Some(req) = detail.levels[&next_level].requirements {
        if let Some(req_building_level) = req.current_building_level {
            if let Some(b) = building {
                if b.level != req_building_level {
                    return false;
                }
            } else {
                return false;
            }
        }

        // #TODO
        // if let Some(req_player_level) = req.current_player_level {
        // }

        if let Some(req_food) = req.food {
            if stats.food.amount < req_food {
                return false;
            }
        }
        if let Some(req_wood) = req.wood {
            if stats.wood.amount < req_wood {
                return false;
            }
        }
        if let Some(req_stone) = req.stone {
            if stats.stone.amount < req_stone {
                return false;
            }
        }
    }

    true
}

pub fn consume_resource(stats: &mut PlayerStats, detail: &BuildingDetail, next_level: i32) {
    if next_level >= detail.levels.len() as i32 {
        panic!("next level is out of range");
    }

    if let Some(req) = detail.levels[&next_level].requirements {
        if let Some(req_food) = req.food {
            stats.food.amount -= req_food;
        }
        if let Some(req_wood) = req.wood {
            stats.wood.amount -= req_wood;
        }
        if let Some(req_stone) = req.stone {
            stats.stone.amount -= req_stone;
        }
    }
}

pub fn upgrade_building(
    detail: &BuildingDetail,
    building: &mut Building,
    generator: Option<&mut Generator>,
    next_level: i32,
) {
    building.level = next_level;
    if let Some(gen) = generator {
        (*gen).rate = detail.levels[&next_level].rate.unwrap();
    }
}
