use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::cmp::{max, min};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub const MAP_PADDING_LEFT: usize = 1;
pub const MAP_PADDING_RIGHT: usize = 1;
pub const MAP_PADDING_UP: usize = 1;
pub const MAP_PADDING_BOTTOM: usize = 15;
pub const MAP_WIDTH: usize = WINDOW_WIDTH - MAP_PADDING_LEFT - MAP_PADDING_RIGHT;
pub const MAP_HEIGHT: usize = WINDOW_HEIGHT - MAP_PADDING_UP - MAP_PADDING_BOTTOM;
pub const MAP_COUNT: usize = MAP_HEIGHT * MAP_WIDTH;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    /// the coordinates of the left-top point of buildings on the map are
    // marked as true
    pub occupied: Vec<bool>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; MAP_COUNT],
            width: MAP_WIDTH as i32,
            height: MAP_HEIGHT as i32,
            occupied: vec![false; MAP_COUNT],
            tile_content: vec![Vec::new(); MAP_COUNT],
        }
    }

    pub fn xy_idx(&self, mut x: i32, mut y: i32) -> usize {
        y -= MAP_PADDING_UP as i32;
        x -= MAP_PADDING_LEFT as i32;

        if x < 0 || y < 0 {
            panic!("Coordinates out of bound");
        }

        (y as usize * self.width as usize) + x as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx % MAP_WIDTH;
        let y = idx / MAP_WIDTH;

        (
            x as i32 + MAP_PADDING_LEFT as i32,
            y as i32 + MAP_PADDING_UP as i32,
        )
    }

    // fn is_exit_valid(&self, x: i32, y: i32) -> bool {
    //     if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
    //         return false;
    //     }
    //     let idx = self.xy_idx(x, y);
    //     !self.blocked[idx]
    // }

    // pub fn populate_blocked(&mut self) {
    //     for (i, tile) in self.tiles.iter_mut().enumerate() {
    //         self.blocked[i] = *tile == TileType::Wall;
    //     }
    // }

    // pub fn clear_content_index(&mut self) {
    //     for content in self.tile_content.iter_mut() {
    //         content.clear();
    //     }
    // }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    // fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
    //     let mut exits = rltk::SmallVec::new();
    //     let x = idx as i32 % self.width;
    //     let y = idx as i32 / self.width;
    //     let w = self.width as usize;

    //     // Cardinal directions
    //     if self.is_exit_valid(x - 1, y) {
    //         exits.push((idx - 1, 1.0))
    //     };
    //     if self.is_exit_valid(x + 1, y) {
    //         exits.push((idx + 1, 1.0))
    //     };
    //     if self.is_exit_valid(x, y - 1) {
    //         exits.push((idx - w, 1.0))
    //     };
    //     if self.is_exit_valid(x, y + 1) {
    //         exits.push((idx + w, 1.0))
    //     };

    //     // Diagonals
    //     if self.is_exit_valid(x - 1, y - 1) {
    //         exits.push(((idx - w) - 1, 1.45));
    //     }
    //     if self.is_exit_valid(x + 1, y - 1) {
    //         exits.push(((idx - w) + 1, 1.45));
    //     }
    //     if self.is_exit_valid(x - 1, y + 1) {
    //         exits.push(((idx + w) - 1, 1.45));
    //     }
    //     if self.is_exit_valid(x + 1, y + 1) {
    //         exits.push(((idx + w) + 1, 1.45));
    //     }

    //     exits
    // }

    // fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
    //     let w = self.width as usize;
    //     let p1 = Point::new(idx1 % w, idx1 / w);
    //     let p2 = Point::new(idx2 % w, idx2 / w);
    //     rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    // }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    ctx.draw_box(
        0,
        0,
        MAP_WIDTH + MAP_PADDING_LEFT,
        MAP_HEIGHT + MAP_PADDING_UP,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let map = ecs.fetch::<Map>();

    let mut y = MAP_PADDING_UP;
    let mut x = MAP_PADDING_LEFT;
    for (idx, tile) in map.tiles.iter().enumerate() {
        // Render a tile depending upon the tile type

        let glyph;
        let fg;
        match tile {
            TileType::Floor => {
                glyph = rltk::to_cp437(' ');
                fg = RGB::from_f32(0.0, 0.5, 0.5);
            }
            TileType::Wall => {
                glyph = rltk::to_cp437('#');
                fg = RGB::from_f32(0., 1.0, 0.);
            }
        }
        ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

        // Move the coordinates
        x += 1;
        if x >= MAP_PADDING_LEFT + MAP_WIDTH {
            x = MAP_PADDING_LEFT;
            y += 1;
        }
    }
}
