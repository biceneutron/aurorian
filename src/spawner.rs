use super::{components::*, Rect};
use rltk::{RandomNumberGenerator, RGB};
use specs::{
    prelude::*,
    saveload::{MarkedBuilder, SimpleMarker},
};
use std::collections::HashMap;

pub fn spawn_mill(ecs: &mut World, rect: Rect) {
    ecs.create_entity()
        .with(Renderable {
            glyph: rltk::to_cp437('☼'),
            fg: RGB::named(rltk::GOLD2),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(Building { rect })
        .with(FoodGenerator { rate: 2 })
        .with(Name {
            name: "Mill".to_string(),
        })
        .build();
}
