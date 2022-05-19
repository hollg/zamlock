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
        let a = 0.5 * self.size;
        let b = -(0.5 * self.size);
        let c = 0.25 * self.size;
        let d = 0.25 * self.size;

        let Pos(x, y) = self.pos;
        let world_to_screen_transform_matrix = Matrix2::new(a, c, b, d);

        let pos_as_matrix = Matrix1x2::new(x as f32, y as f32);

        let mut screen_pos = pos_as_matrix * world_to_screen_transform_matrix;

        let z = -(x as f32 * 0.0001) + -(y as f32 * 0.01) + (layer_index as f32 * 0.01);

        let height_offset = match tile_height {
            TileHeight::Full => layer_index as f32,
            TileHeight::Half => layer_index as f32 * 0.5,
        };

        screen_pos.y += height_offset * self.size / 2.0;

        Vec3::new(screen_pos.x, screen_pos.y, z)
    }

    /// returns y coord offset to from sprite origin to centre of top face
    pub(crate) fn get_y_offset(&self) -> f32 {
        self.size / 4.0
    }
}
