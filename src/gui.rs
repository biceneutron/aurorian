use rltk::{Console, Point, Rltk, VirtualKeyCode, RGB};
use serde::de;
use specs::prelude::*;

use crate::{utils, BuildingDetail, ConstructionManifest, MAP_COUNT, MAP_WIDTH};

use super::{
    components::*, Map, Rect, ResourceType, RunState, State, MAP_HEIGHT, MAP_PADDING_BOTTOM,
    MAP_PADDING_LEFT, MAP_PADDING_UP, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use std::cmp::{max, min};

pub const UIBOX_X: usize = 0;
pub const UIBOX_Y: usize = MAP_PADDING_UP + MAP_HEIGHT + 1;
pub const UIBOX_WIDTH: usize = WINDOW_WIDTH - 1;
pub const UIBOX_HEIGHT: usize = WINDOW_HEIGHT - MAP_PADDING_UP - MAP_HEIGHT - 2;
pub const CONSTRUCTION_INFO_WIDTH: usize = 20;
pub const CONSTRUCTION_INFO_HEIGHT: usize = 20;

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
    let wood_stats;
    let stone_stats;
    if runstate == RunState::PreRun {
        food_stats = format!("Food:  - / - (-/sec)");
        wood_stats = format!("Wood:  - / - (-/sec)");
        stone_stats = format!("Stone: - / - (-/sec)");
    } else {
        food_stats = format!(
            "Food:  {} / {} (+{}/sec)",
            player_stats.food.amount, player_stats.food.max_amount, player_stats.food.rate
        );
        wood_stats = format!(
            "Wood:  {} / {} (+{}/sec)",
            player_stats.wood.amount, player_stats.wood.max_amount, player_stats.wood.rate
        );
        stone_stats = format!(
            "Stone: {} / {} (+{}/sec)",
            player_stats.stone.amount, player_stats.stone.max_amount, player_stats.stone.rate
        );
    }

    ctx.print_color(
        UIBOX_X + 1,
        UIBOX_Y + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        &food_stats,
    );
    ctx.print_color(
        UIBOX_X + 1,
        UIBOX_Y + 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        &wood_stats,
    );
    ctx.print_color(
        UIBOX_X + 1,
        UIBOX_Y + 3,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        &stone_stats,
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
    let runstate = *ecs.fetch::<RunState>();
    if let RunState::ConstructionMenu { mut selected_idx } = runstate {
        let construction_manifest = ecs.fetch::<ConstructionManifest>();
        let player = *ecs.fetch::<Entity>();
        let stats_storage = ecs.read_storage::<PlayerStats>();
        let player_stats = stats_storage.get(player).unwrap();

        ctx.draw_box(
            CONSTRUCTION_MENU_X,
            CONSTRUCTION_MENU_Y,
            CONSTRUCTION_MENU_WIDTH,
            CONSTRUCTION_MENU_HEIGHT,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );

        let separate_vertical_line_x = CONSTRUCTION_MENU_X as i32 + 50;
        ctx.draw_box(
            separate_vertical_line_x,
            CONSTRUCTION_MENU_Y,
            0,
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
        for (idx, detail) in construction_manifest.buildings.iter().enumerate() {
            if selected_idx == idx {
                ctx.print_color(
                    CONSTRUCTION_MENU_X + 2,
                    CONSTRUCTION_MENU_Y + 2 + idx,
                    RGB::named(rltk::MAGENTA),
                    RGB::named(rltk::BLACK),
                    &detail.name,
                );

                print_building_requirements(
                    ctx,
                    player_stats,
                    detail,
                    0,
                    separate_vertical_line_x + 1,
                    CONSTRUCTION_MENU_Y as i32 + 2,
                );
            } else {
                ctx.print_color(
                    CONSTRUCTION_MENU_X + 2,
                    CONSTRUCTION_MENU_Y + 2 + idx,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::BLACK),
                    &detail.name,
                );
            }
        }

        // control
        let construction_manifest = ecs.fetch::<ConstructionManifest>();
        let detail = &construction_manifest.buildings[selected_idx];
        match ctx.key {
            None => return ConstructionMenuResult::NoSelection { selected_idx },
            Some(key) => match key {
                VirtualKeyCode::Escape => return ConstructionMenuResult::Escape,
                VirtualKeyCode::Return => {
                    // #TODO requirements check
                    if utils::requirements_check(player_stats, None, detail, 0) {
                        return ConstructionMenuResult::Selected { selected_idx };
                    }

                    return ConstructionMenuResult::NoSelection { selected_idx };
                }
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
                    // for i in x..x + detail.width {
                    //     for j in y..y + detail.height {
                    //         ctx.set_bg(i, j, RGB::named(rltk::BLACK));
                    //     }
                    // }

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

pub enum ConstructionSelectingResult {
    Escape,
    NoSelection { x: i32, y: i32 },
    Selected { selected_idx: usize, x: i32, y: i32 },
}

pub fn draw_construction_selecting(ecs: &mut World, ctx: &mut Rltk) -> ConstructionSelectingResult {
    let runstate = *ecs.fetch::<RunState>();

    if let RunState::ConstructionSelecting { mut x, mut y } = runstate {
        let map = ecs.fetch::<Map>();
        if x == 0 && y == 0 {
            // searching for the first occupied tile
            let mut idx = 0;
            while idx < MAP_COUNT && !map.occupied[idx] {
                idx += 1;
            }

            if idx == MAP_COUNT {
                return ConstructionSelectingResult::Escape;
            }

            (x, y) = map.idx_xy(idx);
        }

        let mut building_storage = ecs.write_storage::<Building>();
        let name_storage = ecs.read_storage::<Name>();
        let mut generator_storage = ecs.write_storage::<Generator>();
        let entities = ecs.entities();
        let building_manifest = ecs.fetch::<ConstructionManifest>();
        let player = *ecs.fetch::<Entity>();
        let mut stats_storage = ecs.write_storage::<PlayerStats>();
        let player_stats = stats_storage.get_mut(player).unwrap();
        for (building, name, entity) in (&mut building_storage, &name_storage, &entities).join() {
            if x == building.rect.x1 && y == building.rect.y1 {
                // draw the spot
                for i in building.rect.y1..building.rect.y2 {
                    for j in building.rect.x1..building.rect.x2 {
                        ctx.set_bg(j, i, RGB::named(rltk::SKY_BLUE));
                    }
                }

                // draw building info and action menu
                let mut detail = None;
                for b in &building_manifest.buildings {
                    if b.name == name.name {
                        detail = Some(b);
                    }
                }
                let generator = generator_storage.get_mut(entity);
                draw_construction_info(
                    ctx,
                    player_stats,
                    building,
                    name,
                    generator,
                    detail.expect("Building must have detail"),
                );
            }
        }

        // control
        match ctx.key {
            None => return ConstructionSelectingResult::NoSelection { x, y },
            Some(key) => match key {
                VirtualKeyCode::Escape => return ConstructionSelectingResult::Escape,
                //  VirtualKeyCode::Return => {
                //      for i in building.rect.y1..building.rect.y2 + 1 {
                //          for j in building.rect.x1..building.rect.x2 + 1 {
                //              ctx.set_bg(i, j, RGB::named(rltk::BLACK));
                //          }
                //      }

                //      return ConstructionSpotSelectingResult::Selected { selected_idx, x, y };
                //  }
                VirtualKeyCode::K => {
                    if y == MAP_PADDING_UP as i32 {
                        return ConstructionSelectingResult::NoSelection { x, y };
                    }

                    let mut idx = map.xy_idx(x, y - 1);

                    loop {
                        if map.occupied[idx] {
                            let (x, y) = map.idx_xy(idx);
                            return ConstructionSelectingResult::NoSelection { x, y };
                        }

                        if idx == 0 {
                            break;
                        }

                        idx -= 1
                    }

                    return ConstructionSelectingResult::NoSelection { x, y };
                }
                VirtualKeyCode::J => {
                    let map = ecs.fetch::<Map>();
                    if y == (MAP_PADDING_UP + MAP_HEIGHT - 1) as i32 {
                        return ConstructionSelectingResult::NoSelection { x, y };
                    }

                    let mut idx = map.xy_idx(x, y + 1 as i32);

                    while idx < MAP_COUNT {
                        if map.occupied[idx] {
                            let (x, y) = map.idx_xy(idx);
                            return ConstructionSelectingResult::NoSelection { x, y };
                        }
                        idx += 1;
                    }

                    return ConstructionSelectingResult::NoSelection { x, y };
                }
                VirtualKeyCode::H => {
                    let map = ecs.fetch::<Map>();
                    let mut idx = map.xy_idx(x, y);
                    if idx == 0 {
                        return ConstructionSelectingResult::NoSelection { x, y };
                    }
                    idx -= 1;

                    loop {
                        if map.occupied[idx] {
                            let (x, y) = map.idx_xy(idx);
                            return ConstructionSelectingResult::NoSelection { x, y };
                        }

                        if idx == 0 {
                            break;
                        }

                        idx -= 1;
                    }

                    return ConstructionSelectingResult::NoSelection { x, y };
                }
                VirtualKeyCode::L => {
                    let map = ecs.fetch::<Map>();
                    let mut idx = map.xy_idx(x, y) + 1;
                    while idx < MAP_COUNT {
                        if map.occupied[idx] {
                            let (x, y) = map.idx_xy(idx);
                            return ConstructionSelectingResult::NoSelection { x, y };
                        }
                        idx += 1;
                    }

                    return ConstructionSelectingResult::NoSelection { x, y };
                }
                _ => return ConstructionSelectingResult::NoSelection { x, y },
            },
        }
    }

    ConstructionSelectingResult::Escape
}

fn draw_construction_info(
    ctx: &mut Rltk,
    player_stats: &mut PlayerStats,
    building: &mut Building,
    name: &Name,
    generator: Option<&mut Generator>,
    detail: &BuildingDetail,
) {
    let mut info_x = building.rect.x2;
    if info_x + CONSTRUCTION_INFO_WIDTH as i32 >= (MAP_PADDING_LEFT + MAP_WIDTH) as i32 {
        info_x = building.rect.x1 - 1 - CONSTRUCTION_INFO_WIDTH as i32;
    }
    let mut info_y = building.rect.y1;
    if info_y + CONSTRUCTION_INFO_HEIGHT as i32 >= (WINDOW_HEIGHT - MAP_PADDING_BOTTOM) as i32 {
        info_y = info_y
            - (info_y + CONSTRUCTION_INFO_HEIGHT as i32 - WINDOW_HEIGHT as i32
                + MAP_PADDING_BOTTOM as i32);
    }
    ctx.draw_box(
        info_x,
        info_y,
        CONSTRUCTION_INFO_WIDTH,
        CONSTRUCTION_INFO_HEIGHT,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    // construction name
    ctx.print_color(
        info_x + 1,
        info_y + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        name.name.clone(),
    );
    ctx.draw_hollow_box(
        info_x,
        info_y + 2,
        CONSTRUCTION_INFO_WIDTH,
        0,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    // level
    ctx.print_color(
        info_x + 1,
        info_y + 3,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        format!("Level: {}", building.level),
    );

    // rate
    if let Some(gen) = generator.as_ref() {
        let rate_info;
        match gen.resource_type {
            ResourceType::Food => {
                rate_info = format!("Food: +{}/sec", gen.rate);
            }
            ResourceType::Wood => {
                rate_info = format!("Wood: +{}/sec", gen.rate);
            }
            ResourceType::Stone => {
                rate_info = format!("Stone: +{}/sec", gen.rate);
            }
        }

        ctx.print_color(
            info_x + 1,
            info_y + 4,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rate_info,
        );
    }

    // actions
    let next_level = building.level + 1;
    let action_line_y = info_y + CONSTRUCTION_INFO_HEIGHT as i32 - 3;
    ctx.draw_hollow_box(
        info_x,
        action_line_y,
        CONSTRUCTION_INFO_WIDTH,
        0,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    if next_level < detail.levels.len() as i32 {
        ctx.print_color(
            info_x + 1,
            action_line_y + 1,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            format!("[u] Upgrade"),
        );
    }
    ctx.print_color(
        info_x + 1,
        action_line_y + 2,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        format!("[t] Tear down"),
    );

    // upgrade requirements
    if next_level < detail.levels.len() as i32 {
        let requirement_line_y = action_line_y - 5;
        ctx.draw_hollow_box(
            info_x,
            requirement_line_y,
            CONSTRUCTION_INFO_WIDTH,
            0,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );

        print_building_requirements(
            ctx,
            player_stats,
            detail,
            next_level,
            info_x + 1,
            requirement_line_y,
        );
    }

    // player control
    match ctx.key {
        Some(key) => match key {
            VirtualKeyCode::U => {
                // upgrade
                if utils::requirements_check(player_stats, Some(building), detail, next_level) {
                    utils::consume_resource(player_stats, detail, next_level);
                    utils::upgrade_building(detail, building, generator, next_level);
                }
            }
            VirtualKeyCode::T => {
                // tear down
            }
            _ => {}
        },
        None => {}
    }
}

fn print_building_requirements(
    ctx: &mut Rltk,
    player_stats: &PlayerStats,
    detail: &BuildingDetail,
    next_level: i32,
    x: i32,
    y: i32,
) {
    let mut y_offset = 0;

    // next level info
    if let Some(rate) = detail.levels[&next_level].rate {
        let info;
        if next_level == 0 {
            info = format!(
                "Generates {:?} at the rate +{} / sec.",
                detail.resource_type.unwrap(),
                rate
            );
        } else {
            info = format!("Next Level:+{}/sec", rate);
        }

        ctx.print_color(
            x,
            y + y_offset,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            info,
        );
        y_offset += 1;
    }

    // requirements
    if let Some(requirements) = detail.levels[&next_level].requirements {
        if next_level == 0 {
            y_offset += 1;
            ctx.print_color(
                x,
                y + y_offset,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "Requirements:",
            );
            y_offset += 1;
        }

        let level_req;
        let mut color = RGB::named(rltk::WHITE);
        if let Some(cur_player_level) = requirements.current_player_level {
            level_req = format!("Player Level: {}", cur_player_level);

            // #TODO
            // check the current player level and decide the color
            //
        } else {
            level_req = format!("Player Level:-");
        }
        ctx.print_color(x, y + y_offset, color, RGB::named(rltk::BLACK), level_req);
        y_offset += 1;

        let food_req;
        color = RGB::named(rltk::WHITE);
        if let Some(food) = requirements.food {
            food_req = format!("Food: {}", food);
            if player_stats.food.amount < food {
                color = *utils::MORANDI_RED;
            }
        } else {
            food_req = format!("Food:-");
        }
        ctx.print_color(x, y + y_offset, color, RGB::named(rltk::BLACK), food_req);
        y_offset += 1;

        let wood_req;
        color = RGB::named(rltk::WHITE);
        if let Some(wood) = requirements.wood {
            wood_req = format!("Wood: {}", wood);
            if player_stats.wood.amount < wood {
                color = RGB::named(rltk::RED);
            }
        } else {
            wood_req = format!("Wood:-");
        }
        ctx.print_color(x, y + y_offset, color, RGB::named(rltk::BLACK), wood_req);
        y_offset += 1;

        let stone_req;
        color = RGB::named(rltk::WHITE);
        if let Some(stone) = requirements.stone {
            stone_req = format!("Stone: {}", stone);
            if player_stats.stone.amount < stone {
                color = RGB::named(rltk::RED);
            }
        } else {
            stone_req = format!("Stone:-");
        }
        ctx.print_color(x, y + y_offset, color, RGB::named(rltk::BLACK), stone_req);
        y_offset += 1;
    }
}
