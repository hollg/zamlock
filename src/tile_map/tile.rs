use super::{graphics::MapSprites, pos::Pos};
use bevy::prelude::*;
use nalgebra::{Matrix1x2, Matrix2};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileHeight {
    Full,
    Half,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Tile {
    /// cartesian, in relation to layer
    pub(crate) pos: Pos,
    pub(crate) height: TileHeight,
    /// height/width in pixels (tile must be square!)
    pub(crate) size: f32,
}

impl Tile {
    pub(crate) fn spawn(
        &self,
        entity: Entity,
        commands: &mut Commands,
        graphics: &Res<MapSprites>,

        layer_index: usize,
    ) -> Entity {
        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                texture: graphics.get_tile(self.height),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(self.size)),
                    ..default()
                },
                transform: Transform::from_translation(
                    self.to_screen_space(layer_index, self.height),
                ),
                ..default()
            })
            .insert(*self)
            .id()
    }

    /// get world coords for isometric grid
    pub(crate) fn to_screen_space(self, layer_index: usize, tile_height: TileHeight) -> Vec3 {
        let x_transform = Vec2::new(0.5 * self.size, 0.25 * self.size);
        let y_transform = Vec2::new(-(0.5 * self.size), 0.25 * self.size);

        let Pos(x, y) = self.pos;

        // transform x + z into 2d screen space coord
        let mut coords = (x as f32 * x_transform) + (y as f32 * y_transform);

        // bevy y axis is in the opposite direction
        coords.y = -coords.y;

        // TODO: I'm pretty sure this is wrong for layer indexes greater than 1
        // — maybe it's layer_index - 0.5 in those cases?

        let height_offset = match tile_height {
            TileHeight::Full => layer_index as f32,
            TileHeight::Half => layer_index as f32 * 0.5,
        };
        coords.y += height_offset * self.size / 2.0;

        let z = (x as f32 * 0.0001) + (y as f32 * 0.001) + (layer_index as f32 * 0.01);

        Vec3::new(coords.x, coords.y, z)
    }

    /// returns y coord offset to from sprite origin to centre of top face
    pub(crate) fn get_y_offset(&self) -> f32 {
        self.size / 4.0
    }
}

pub(crate) trait ToWorld {
    fn to_world(&self, size: f32) -> Pos;
}

impl ToWorld for Vec2 {
    fn to_world(&self, size: f32) -> Pos {
        let a = 0.5 * size;
        let b = -(0.5 * size);
        let c = 0.25 * size;
        let d = 0.25 * size;

        let world_to_screen_transform_matrix = Matrix2::new(a, b, c, d);
        let screen_to_world_transform_matrix = world_to_screen_transform_matrix
            .try_inverse()
            .expect("Can't inverse matrix");

        let screen_pos_matrix = Matrix1x2::new(self.x as f32, self.y as f32);

        let world_pos_matrix = screen_pos_matrix * screen_to_world_transform_matrix;

        let mut world_pos = Pos(world_pos_matrix.x as u32, -world_pos_matrix.y as u32);

        world_pos
    }
}
