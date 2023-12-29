use super::components::*;
use rltk::Rltk;
use specs::prelude::*;

pub fn draw_buildings(ecs: &World, ctx: &mut Rltk) {
    let building_storage = ecs.read_storage::<Building>();
    let renderable_storage = ecs.read_storage::<Renderable>();

    for (building, renderable) in (&building_storage, &renderable_storage).join() {
        for x in building.rect.x1..building.rect.x2 {
            for y in building.rect.y1..building.rect.y2 {
                ctx.set(x, y, renderable.fg, renderable.bg, renderable.glyph);
            }
        }
    }
}
