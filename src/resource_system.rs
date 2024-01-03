use crate::ResourceType;

use super::components;
use chrono::prelude::*;
use rltk::console;
use specs::prelude::*;
use std::cmp::min;

pub struct ResourceSystem {}

impl<'a> System<'a> for ResourceSystem {
    type SystemData = (
        ReadStorage<'a, components::Generator>,
        WriteStorage<'a, components::PlayerStats>,
        WriteExpect<'a, Entity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (generators, mut stats, player) = data;

        let player_stats = stats.get_mut(*player).expect("Player must have stats");
        let current = Local::now().timestamp();
        let time_elapsed = (current - player_stats.next_refresh) as i32;

        if time_elapsed > 0 {
            let mut food_rate_sum = 0;
            let mut wood_rate_sum = 0;
            let mut stone_rate_sum = 0;
            for generator in generators.join() {
                match generator.resource_type {
                    ResourceType::Food => {
                        food_rate_sum += generator.rate;
                    }
                    ResourceType::Wood => {
                        wood_rate_sum += generator.rate;
                    }
                    ResourceType::Stone => {
                        stone_rate_sum += generator.rate;
                    }
                }
            }

            player_stats.food.amount = min(
                player_stats.food.max_amount,
                player_stats.food.amount + food_rate_sum * time_elapsed,
            );
            player_stats.food.rate = food_rate_sum;

            player_stats.wood.amount = min(
                player_stats.wood.max_amount,
                player_stats.wood.amount + wood_rate_sum * time_elapsed,
            );
            player_stats.wood.rate = wood_rate_sum;

            player_stats.stone.amount = min(
                player_stats.stone.max_amount,
                player_stats.stone.amount + stone_rate_sum * time_elapsed,
            );
            player_stats.stone.rate = stone_rate_sum;

            player_stats.next_refresh = current;
        }
    }
}
