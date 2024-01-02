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
            for generator in generators.join() {
                match generator.resource_type {
                    ResourceType::Food => {
                        food_rate_sum += generator.rate;
                    }

                    // #TODO more resource types here
                    _ => {}
                }
            }

            player_stats.food_amount = min(
                player_stats.food_amount_max,
                player_stats.food_amount + food_rate_sum * time_elapsed,
            );
            player_stats.food_generation_rate = food_rate_sum;
            player_stats.next_refresh = current;
        }
    }
}
