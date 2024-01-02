use chrono::prelude::*;
use map::draw_map;
use render::draw_buildings;
use resource_system::ResourceSystem;
use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use std::collections::{hash_map, HashMap};
use std::time::Duration;

use crate::map::Map;
use rand::Rng;

// serde
use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod components;
mod map;
pub use components::*;
pub use map::*;

mod gui;
mod rect;
pub use rect::*;
mod control;
mod render;
mod resource_system;
mod spawner;
mod utils;

pub const WINDOW_HEIGHT: usize = 100;
pub const WINDOW_WIDTH: usize = 150;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    PreRun,
    Idle,
    ConstructionMenu { selected_idx: usize },
    ConstructionSpotSelecting { selected_idx: usize, x: i32, y: i32 },
    ConstructionSelecting { x: i32, y: i32 },
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut resource = ResourceSystem {};

        resource.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        draw_map(&self.ecs, ctx);
        draw_buildings(&self.ecs, ctx);
        gui::draw_ui(&self.ecs, ctx);

        // state machine
        let mut new_runstate = *self.ecs.fetch::<RunState>();
        match new_runstate {
            RunState::PreRun => {
                self.run_systems();
                new_runstate = RunState::Idle;
            }
            RunState::Idle => {
                self.run_systems();
                new_runstate = control::player_input(&mut self.ecs, ctx);
            }
            RunState::ConstructionMenu { .. } => {
                self.run_systems();

                // show construction menu
                let result = gui::draw_construction_menu(&mut self.ecs, ctx);
                match result {
                    gui::ConstructionMenuResult::Selected { selected_idx } => {
                        // build the selected building
                        // let x = rand::thread_rng().gen_range(1..=120);
                        // let y = rand::thread_rng().gen_range(1..=70);
                        // let rect = Rect::new(x, y, 4, 4);
                        // spawner::spawn_mill(&mut self.ecs, rect);
                        // new_runstate = RunState::Idle
                        new_runstate = RunState::ConstructionSpotSelecting {
                            selected_idx,
                            x: WINDOW_WIDTH as i32 / 2,
                            y: WINDOW_HEIGHT as i32 / 2,
                        };
                    }
                    gui::ConstructionMenuResult::NoSelection { selected_idx } => {
                        new_runstate = RunState::ConstructionMenu { selected_idx }
                    }
                    gui::ConstructionMenuResult::Escape => new_runstate = RunState::Idle,
                }
            }
            RunState::ConstructionSpotSelecting { selected_idx, .. } => {
                self.run_systems();
                let result = gui::draw_construction_spot(&mut self.ecs, ctx);
                match result {
                    gui::ConstructionSpotSelectingResult::Selected { selected_idx, x, y } => {
                        let (detail, spawner_fn) =
                            spawner::get_spawner(&mut self.ecs, selected_idx);
                        spawner_fn(&mut self.ecs, detail, x, y);
                        new_runstate = RunState::Idle
                    }
                    gui::ConstructionSpotSelectingResult::NoSelection { selected_idx, x, y } => {
                        new_runstate = RunState::ConstructionSpotSelecting { selected_idx, x, y }
                    }
                    gui::ConstructionSpotSelectingResult::Escape => {
                        new_runstate = RunState::ConstructionMenu { selected_idx }
                    }
                }
            }
            RunState::ConstructionSelecting { x, y } => {
                self.run_systems();
                let result = gui::draw_construction_selecting(&mut self.ecs, ctx);
                match result {
                    gui::ConstructionSelectingResult::Selected { .. } => {
                        new_runstate = RunState::Idle;
                    }
                    gui::ConstructionSelectingResult::NoSelection { x, y } => {
                        new_runstate = RunState::ConstructionSelecting { x, y };
                    }
                    gui::ConstructionSelectingResult::Escape => {
                        new_runstate = RunState::Idle;
                    }
                }
            }
        }

        let mut runstate_writer = self.ecs.write_resource::<RunState>();
        *runstate_writer = new_runstate;
    }
}

// Constrction list
#[derive(Deserialize, Debug)]
pub struct ConstructionManifest {
    pub buildings: Vec<BuildingDetail>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BuildingDetail {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub fg: String,
    pub bg: String,
    pub glyph: char,
    pub levels: HashMap<i32, LevelDetail>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct LevelDetail {
    pub rate: Option<i32>,
    pub requirements: Option<ConstructionRequirment>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct ConstructionRequirment {
    pub current_player_level: Option<i32>,
    pub current_building_level: Option<i32>,
    pub food: Option<i32>,
    pub wood: Option<i32>,
    pub stone: Option<i32>,
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(WINDOW_WIDTH, WINDOW_HEIGHT)
        .expect("Failed creating window")
        .with_title("Aurorian")
        .build()?;
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<PlayerStats>();
    gs.ecs.register::<Generator>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Building>();
    gs.ecs.register::<Name>();

    let map = Map::new();

    // create player
    let current: DateTime<Local> = Local::now();
    let player_stats = gs
        .ecs
        .create_entity()
        .with(PlayerStats {
            food_amount: 0,
            food_amount_max: 10000,
            food_generation_rate: 0,
            next_refresh: current.timestamp() - 1,
        })
        .build();

    // mill
    // gs.ecs
    //     .create_entity()
    //     .with(Renderable {
    //         glyph: rltk::to_cp437('☼'),
    //         fg: RGB::named(rltk::GOLD2),
    //         bg: RGB::named(rltk::BLACK),
    //         render_order: 0,
    //     })
    //     .with(Building {
    //         rect: Rect::new(40, 40, 4, 4),
    //     })
    //     .with(FoodGenerator { rate: 2 })
    //     .build();

    let file = File::open("src/constructions.json")?;
    let reader = BufReader::new(file);
    let construction_manifest = serde_json::from_reader::<_, ConstructionManifest>(reader)?;

    gs.ecs.insert(construction_manifest);
    gs.ecs.insert(map);
    gs.ecs.insert(player_stats);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    rltk::main_loop(context, gs)
}
