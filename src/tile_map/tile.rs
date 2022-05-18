use super::{graphics::MapSprites, pos::Pos};
use bevy::prelude::*;

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
                transform: Transform::from_translation(self.isometric(layer_index, self.height)),
                ..default()
            })
            .insert(*self)
            .id()
    }

    /// get world coords for isometric grid
    pub(crate) fn isometric(self, layer_index: usize, tile_height: TileHeight) -> Vec3 {
        let a = 0.5 * self.size;
        let b = -(0.5 * self.size);
        let c = 0.25 * self.size;
        let d = 0.25 * self.size;
        let x_transform = Vec2::new(a, c);
        let y_transform = Vec2::new(b, d);

        let Pos(x, y) = self.pos;

        // transform x + z into 2d isometric coord
        let mut coords = (x as f32 * x_transform) + (y as f32 * y_transform);

        // bevy y axis is in the opposite direction
        coords.y = -coords.y;
        let y_offset = match tile_height {
            TileHeight::Full => layer_index as f32,
            TileHeight::Half => layer_index as f32 * 0.5,
        };
        coords.y += y_offset as f32 * self.size / 2.0;

        let z = (x as f32 * 0.0001) + (y as f32 * 0.001) + (layer_index as f32 * 0.01);

        Vec3::new(coords.x, coords.y, z)
    }
}
