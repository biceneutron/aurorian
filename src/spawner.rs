use crate::{BuildingDetail, ConstructionManifest};

use super::{components::*, Map, Rect, ResourceType};
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
        "Farm" => spawn_farm,
        "Food Factory" => spawn_food_factory,
        "Army" => spawn_army,
        "Lumber Camp" => spawn_lumber_camp,
        "Mining Camp" => spawn_mining_camp,
        _ => panic!("Unmatched building name"),
    };

    (detail, func)
}

pub fn spawn_farm(ecs: &mut World, detail: BuildingDetail, x: i32, y: i32) {
    {
        let mut map = ecs.write_resource::<Map>();
        let idx = map.xy_idx(x, y);
        map.occupied[idx] = true;
    }

    let rect = Rect::new(x, y, detail.width, detail.height);
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('☼'),
            fg: RGB::named(rltk::WHEAT3),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Building { rect, level: 0 })
        .with(Generator {
            rate: detail.levels.get(&0).unwrap().rate.unwrap(),
            resource_type: ResourceType::Food,
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
            glyph: rltk::to_cp437('o'),
            fg: RGB::named(rltk::LIME),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Building { rect, level: 0 })
        .with(Generator {
            rate: detail.levels.get(&0).unwrap().rate.unwrap(),
            resource_type: ResourceType::Food,
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
            glyph: rltk::to_cp437('x'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Building { rect, level: 0 })
        .with(Name {
            name: detail.name.to_string(),
        })
        .build();
}

pub fn spawn_lumber_camp(ecs: &mut World, detail: BuildingDetail, x: i32, y: i32) {
    {
        let mut map = ecs.write_resource::<Map>();
        let idx = map.xy_idx(x, y);
        map.occupied[idx] = true;
    }

    let rect = Rect::new(x, y, detail.width, detail.height);
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('╣'),
            fg: RGB::named(rltk::TAN4),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Building { rect, level: 0 })
        .with(Generator {
            rate: detail.levels.get(&0).unwrap().rate.unwrap(),
            resource_type: ResourceType::Wood,
        })
        .with(Name {
            name: detail.name.to_string(),
        })
        .build();
}

pub fn spawn_mining_camp(ecs: &mut World, detail: BuildingDetail, x: i32, y: i32) {
    {
        let mut map = ecs.write_resource::<Map>();
        let idx = map.xy_idx(x, y);
        map.occupied[idx] = true;
    }

    let rect = Rect::new(x, y, detail.width, detail.height);
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('■'),
            fg: RGB::named(rltk::GRAY60),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Building { rect, level: 0 })
        .with(Generator {
            rate: detail.levels.get(&0).unwrap().rate.unwrap(),
            resource_type: ResourceType::Stone,
        })
        .with(Name {
            name: detail.name.to_string(),
        })
        .build();
}
