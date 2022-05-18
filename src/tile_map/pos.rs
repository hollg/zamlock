use super::tile::{TileHeight, TILE_SIZE};
use bevy::prelude::{Vec2, Vec3};

const A: f32 = 0.5 * TILE_SIZE;
const B: f32 = -(0.5 * TILE_SIZE);
const C: f32 = 0.25 * TILE_SIZE;
const D: f32 = 0.25 * TILE_SIZE;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Pos(pub(crate) u32, pub(crate) u32);

impl Pos {
    pub(crate) fn to_isometric(self, layer_index: usize, tile_height: TileHeight) -> Vec3 {
        let x_transform = Vec2::new(A, C);
        let y_transform = Vec2::new(B, D);

        // transform x + z into 2d isometric coord
        let mut coords = (self.0 as f32 * x_transform) + (self.1 as f32 * y_transform);

        // bevy y axis is in the opposite direction
        coords.y = -coords.y;
        let y_offset = match tile_height{
            TileHeight::Full => layer_index as f32,
            TileHeight::Half => layer_index as f32 * 0.5,

        };
        coords.y += y_offset as f32 * TILE_SIZE / 2.0;

        let z = (self.0 as f32 * 0.0001) + (self.1 as f32 * 0.001) + (layer_index as f32 * 0.01);

        Vec3::new(coords.x, coords.y, z)
    }
}
