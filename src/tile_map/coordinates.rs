use super::tile::Pos;
use super::tile::TILE_SIZE;
use bevy::prelude::{Vec2, Vec3};

const A: f32 = 0.5 * TILE_SIZE;
const B: f32 = -(0.5 * TILE_SIZE);
const C: f32 = 0.25 * TILE_SIZE;
const D: f32 = 0.25 * TILE_SIZE;

pub trait Coordinates {
    fn to_isometric(&self) -> Vec3;
}

impl Coordinates for Pos {
    fn to_isometric(&self) -> Vec3 {
        let x_transform = Vec2::new(A, C);
        let y_transform = Vec2::new(B, D);

        // transform x + z into 2d isometric coord
        let mut coords = (self.0 as f32 * x_transform) + (self.1 as f32 * y_transform);

        // bevy y axis is in the opposite direction
        coords.y = -coords.y;
        // TODO: account for layer here — higher layer - higher y
        // coords.y += self.1 as f32 * TILE_SIZE / 2.0;

        // account for layer in this maths - higher layer = higher z
        let z = (self.0 as f32 * 0.0001) + (self.1 as f32 * 0.001); 

        Vec3::new(coords.x, coords.y, z)
    }
}
