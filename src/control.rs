use rltk::{console, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

use super::RunState;

pub fn player_input(ecs: &mut World, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::Idle, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::C => return RunState::ConstructionMenu { selected_idx: 0 },

            _ => return RunState::Idle,
        },
    }

    RunState::Idle
}
