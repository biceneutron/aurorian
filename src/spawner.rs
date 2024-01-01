use crate::{BuildingDetail, ConstructionManifest};

use super::{components::*, utils, Map, Rect, State};
use rltk::{RandomNumberGenerator, RGB};
use specs::{
    prelude::*,
    saveload::{MarkedBuilder, SimpleMarker},
};

pub fn get_spawner(
    ecs: &mut World,
    idx: usize,
) -> (
    BuildingDetail,
    impl Fn(&mut World, BuildingDetail, i32, i32),
) {
    let detail = ecs.fetch::<ConstructionManifest>().buildings[idx].clone();

    let func = match detail.name.as_str() {
        "Mill" => spawn_mill,
        "Food Factory" => spawn_food_factory,
        "Army" => spawn_army,
        _ => panic!("Unmatched building name"),
    };

    (detail, func)
}

pub fn spawn_mill(ecs: &mut World, detail: BuildingDetail, x: i32, y: i32) {
    {
        let mut map = ecs.write_resource::<Map>();
        let idx = map.xy_idx(x, y);
        map.occupied[idx] = true;
    }

    let rect = Rect::new(x, y, detail.width, detail.height);
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437(detail.glyph),
            fg: utils::str_to_rgb(detail.fg.as_str()).unwrap(),
            bg: utils::str_to_rgb(detail.bg.as_str()).unwrap(),
            render_order: 0,
        })
        .with(Building { rect, level: 0 })
        .with(FoodGenerator {
            rate: detail.levels.get(&0).unwrap().rate.unwrap(),
        })
        .with(Name {
            name: detail.name.to_string(),
        })
        .build();
}

pub fn spawn_food_factory(ecs: &mut World, detail: BuildingDetail, x: i32, y: i32) {
    {
        let mut map = ecs.write_resource::<Map>();
        let idx = map.xy_idx(x, y);
        map.occupied[idx] = true;
    }

    let rect = Rect::new(x, y, detail.width, detail.height);
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437(detail.glyph),
            fg: utils::str_to_rgb(detail.fg.as_str()).unwrap(),
            bg: utils::str_to_rgb(detail.bg.as_str()).unwrap(),
            render_order: 0,
        })
        .with(Building { rect, level: 0 })
        .with(FoodGenerator {
            rate: detail.levels.get(&0).unwrap().rate.unwrap(),
        })
        .with(Name {
            name: detail.name.to_string(),
        })
        .build();
}

pub fn spawn_army(ecs: &mut World, detail: BuildingDetail, x: i32, y: i32) {
    {
        let mut map = ecs.write_resource::<Map>();
        let idx = map.xy_idx(x, y);
        map.occupied[idx] = true;
    }

    let rect = Rect::new(x, y, detail.width, detail.height);
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437(detail.glyph),
            fg: utils::str_to_rgb(detail.fg.as_str()).unwrap(),
            bg: utils::str_to_rgb(detail.bg.as_str()).unwrap(),
            render_order: 0,
        })
        .with(Building { rect, level: 0 })
        .with(Name {
            name: detail.name.to_string(),
        })
        .build();
}
