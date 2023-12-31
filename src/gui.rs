use rltk::{Console, Point, Rltk, VirtualKeyCode, RGB};
use serde::de;
use specs::prelude::*;

use crate::{BuildingDetail, ConstructionManifest, MAP_WIDTH};

use super::{
    components::*, Map, Rect, RunState, State, MAP_HEIGHT, MAP_PADDING_LEFT, MAP_PADDING_UP,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};
use std::cmp::{max, min};

pub const UIBOX_X: usize = 0;
pub const UIBOX_Y: usize = MAP_PADDING_UP + MAP_HEIGHT + 1;
pub const UIBOX_WIDTH: usize = WINDOW_WIDTH - 1;
pub const UIBOX_HEIGHT: usize = WINDOW_HEIGHT - MAP_PADDING_UP - MAP_HEIGHT - 2;

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(
        UIBOX_X,
        UIBOX_Y,
        UIBOX_WIDTH,
        UIBOX_HEIGHT,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let stats_storage = ecs.read_storage::<PlayerStats>();
    let player = *ecs.read_resource::<Entity>();
    let player_stats = stats_storage.get(player).expect("Player must have stats");

    let runstate = *ecs.read_resource::<RunState>();
    let food_stats;
    if runstate == RunState::PreRun {
        food_stats = format!("Food: - / - (-/sec)");
    } else {
        food_stats = format!(
            "Food: {} / {} (+{}/sec)",
            player_stats.food_amount,
            player_stats.food_amount_max,
            player_stats.food_generation_rate
        );
    }

    ctx.print_color(
        UIBOX_X + 1,
        UIBOX_Y + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        &food_stats,
    );
}

pub const CONSTRUCTION_MENU_X: usize = 15;
pub const CONSTRUCTION_MENU_Y: usize = 10;
pub const CONSTRUCTION_MENU_WIDTH: usize = 120;
pub const CONSTRUCTION_MENU_HEIGHT: usize = 60;
#[derive(PartialEq, Copy, Clone)]
pub enum ConstructionMenuResult {
    Escape,
    NoSelection { selected_idx: usize },
    Selected { selected_idx: usize },
}

pub fn draw_construction_menu(ecs: &mut World, ctx: &mut Rltk) -> ConstructionMenuResult {
    let construction_manifest = ecs.fetch::<ConstructionManifest>();

    let runstate = *ecs.fetch::<RunState>();
    if let RunState::ConstructionMenu { mut selected_idx } = runstate {
        ctx.draw_box(
            CONSTRUCTION_MENU_X,
            CONSTRUCTION_MENU_Y,
            CONSTRUCTION_MENU_WIDTH,
            CONSTRUCTION_MENU_HEIGHT,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );
        ctx.print_color(
            CONSTRUCTION_MENU_X + 1,
            CONSTRUCTION_MENU_Y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "Construction Menu",
        );
        ctx.print_color(
            CONSTRUCTION_MENU_X + 1,
            CONSTRUCTION_MENU_Y + CONSTRUCTION_MENU_HEIGHT,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "ESCAPE to cancel",
        );

        if construction_manifest.buildings.is_empty() {
            // control
            match ctx.key {
                None => return ConstructionMenuResult::NoSelection { selected_idx },
                Some(key) => match key {
                    VirtualKeyCode::Escape => return ConstructionMenuResult::Escape,
                    _ => return ConstructionMenuResult::NoSelection { selected_idx },
                },
            }
        }

        // draw construction options
        for (idx, building) in construction_manifest.buildings.iter().enumerate() {
            if selected_idx == idx {
                ctx.print_color(
                    CONSTRUCTION_MENU_X + 2,
                    CONSTRUCTION_MENU_Y + 2 + idx,
                    RGB::named(rltk::MAGENTA),
                    RGB::named(rltk::BLACK),
                    &building.name,
                );
            } else {
                ctx.print_color(
                    CONSTRUCTION_MENU_X + 2,
                    CONSTRUCTION_MENU_Y + 2 + idx,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::BLACK),
                    &building.name,
                );
            }
        }

        // control
        match ctx.key {
            None => return ConstructionMenuResult::NoSelection { selected_idx },
            Some(key) => match key {
                VirtualKeyCode::Escape => return ConstructionMenuResult::Escape,
                VirtualKeyCode::Return => return ConstructionMenuResult::Selected { selected_idx },
                VirtualKeyCode::K => {
                    if selected_idx == 0 {
                        selected_idx = construction_manifest.buildings.len() - 1;
                    } else {
                        selected_idx -= 1;
                    }
                    return ConstructionMenuResult::NoSelection { selected_idx };
                }
                VirtualKeyCode::J => {
                    selected_idx = (selected_idx + 1) % construction_manifest.buildings.len();
                    return ConstructionMenuResult::NoSelection { selected_idx };
                }
                _ => return ConstructionMenuResult::NoSelection { selected_idx },
            },
        }
    }

    ConstructionMenuResult::NoSelection { selected_idx: 0 }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ConstructionSpotSelectingResult {
    Escape,
    NoSelection { selected_idx: usize, x: i32, y: i32 },
    Selected { selected_idx: usize, x: i32, y: i32 },
}

pub fn draw_construction_spot(ecs: &mut World, ctx: &mut Rltk) -> ConstructionSpotSelectingResult {
    let runstate = *ecs.fetch::<RunState>();

    if let RunState::ConstructionSpotSelecting { selected_idx, x, y } = runstate {
        let detail = &ecs.fetch::<ConstructionManifest>().buildings[selected_idx];

        let mut valid = true;
        let target_spot = Rect::new(x, y, detail.width, detail.height);
        let buildings_storage = ecs.read_storage::<Building>();
        for building in buildings_storage.join() {
            if target_spot.intersect(&building.rect) {
                valid = false;
                break;
            }
        }

        // draw the spot
        let spot_color = if valid {
            RGB::named(rltk::GREEN)
        } else {
            RGB::named(rltk::RED)
        };
        for i in x..x + detail.width {
            for j in y..y + detail.height {
                ctx.set_bg(i, j, spot_color);
            }
        }

        // control
        match ctx.key {
            None => return ConstructionSpotSelectingResult::NoSelection { selected_idx, x, y },
            Some(key) => match key {
                VirtualKeyCode::Escape => return ConstructionSpotSelectingResult::Escape,
                VirtualKeyCode::Return => {
                    if !valid {
                        return ConstructionSpotSelectingResult::NoSelection { selected_idx, x, y };
                    }
                    for i in x..x + detail.width {
                        for j in y..y + detail.height {
                            ctx.set_bg(i, j, RGB::named(rltk::BLACK));
                        }
                    }

                    return ConstructionSpotSelectingResult::Selected { selected_idx, x, y };
                }
                VirtualKeyCode::K => {
                    return ConstructionSpotSelectingResult::NoSelection {
                        selected_idx,
                        x,
                        y: max(y - 1, MAP_PADDING_UP as i32),
                    }
                }
                VirtualKeyCode::J => {
                    return ConstructionSpotSelectingResult::NoSelection {
                        selected_idx,
                        x,
                        y: min(
                            y + 1,
                            MAP_PADDING_UP as i32 + MAP_HEIGHT as i32 - detail.height,
                        ),
                    }
                }
                VirtualKeyCode::H => {
                    return ConstructionSpotSelectingResult::NoSelection {
                        selected_idx,
                        x: max(x - 1, MAP_PADDING_LEFT as i32),
                        y,
                    }
                }
                VirtualKeyCode::L => {
                    return ConstructionSpotSelectingResult::NoSelection {
                        selected_idx,
                        x: min(
                            x + 1,
                            MAP_PADDING_LEFT as i32 + MAP_WIDTH as i32 - detail.width,
                        ),
                        y,
                    }
                }
                _ => return ConstructionSpotSelectingResult::NoSelection { selected_idx, x, y },
            },
        }
    }

    ConstructionSpotSelectingResult::Escape
}
