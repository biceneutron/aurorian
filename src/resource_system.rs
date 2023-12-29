use super::components;
use chrono::prelude::*;
use rltk::console;
use specs::prelude::*;
use std::cmp::min;

pub struct ResourceSystem {}

impl<'a> System<'a> for ResourceSystem {
    type SystemData = (
        ReadStorage<'a, components::FoodGenerator>,
        WriteStorage<'a, components::PlayerStats>,
        WriteExpect<'a, Entity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (food_generators, mut stats, player) = data;

        let player_stats = stats.get_mut(*player).expect("Player must have stats");
        let current = Local::now().timestamp();
        let time_elapsed = (current - player_stats.next_refresh) as i32;

        if time_elapsed > 0 {
            let mut rate_sum = 0;
            for food_generator in food_generators.join() {
                rate_sum += food_generator.rate;
            }

            player_stats.food_amount = min(
                player_stats.food_amount_max,
                player_stats.food_amount + rate_sum * time_elapsed,
            );
            player_stats.food_generation_rate = rate_sum;
            player_stats.next_refresh = current;
        }
    }
}
