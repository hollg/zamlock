use std::collections::HashMap;

use bevy::prelude::*;
use nalgebra::{Matrix1x2, Matrix2};

use super::{
    graphics::MapSprites,
    pos::{Pos, UnorderedPos},
    tile::Tile,
};

#[derive(Component, Clone)]
/// Contains hashmap of all Tiles and provides utilities for
/// translating between world and screen coordinates etc
pub struct Map {
    pub(crate) entity: Entity,
    /// tile size must be square; this is both height and width
    pub(crate) tile_size: f32,
    /// Pos places tiles in 3d world grid space.
    ///
    /// x runs SouthWest - NorthEast
    ///
    /// y runs groud - sky
    ///
    /// z runs SouthEast - NorthWest
    pub(crate) tiles: HashMap<Pos, Entity>,
    /// Positions the map on the screen. This value is important when mapping screen coordinates
    /// to world/grid coordinates
    pub translation: Vec3,
}

impl Map {
    pub(crate) fn new(entity: Entity, tile_size: f32, translation: Vec3) -> Map {
        Map {
            entity,
            tile_size,
            translation,
            tiles: HashMap::new(),
        }
    }

    pub(crate) fn insert_tile(
        &mut self,
        commands: &mut Commands,
        pos: Pos,
        tile: Tile,
        graphics: &Res<MapSprites>,
    ) {
        let tile_entity = commands.spawn().id();
        tile.spawn(tile_entity, commands, graphics, self.world_pos_to_screen_pos(pos));

        commands.entity(self.entity).add_child(tile_entity);
        self.tiles.insert(pos, tile_entity);
    }

    /// This should only be called once, in startup. If you call it on the Map you get from a query
    /// then things will break!
    pub(crate) fn spawn(self, commands: &mut Commands) {
        commands
            .entity(self.entity)
            .insert(self.clone())
            .insert_bundle(TransformBundle {
                local: Transform::from_translation(self.translation),
                ..default()
            });
    }

    /// translate screen space coords into isometric grid pos with y = 0.0
    pub(crate) fn screen_pos_to_world_pos(&self, screen_pos: Vec2) -> Pos {
        let screen_to_world_transform_matrix = self
            .screen_to_world_transform_matrix()
            .expect("Can't inverse matrix");

        let offset_screen_pos = screen_pos - self.translation.truncate();
        let screen_pos_matrix = Matrix1x2::new(offset_screen_pos.x, offset_screen_pos.y);
        let world_pos_matrix = screen_pos_matrix * screen_to_world_transform_matrix;

        Pos::new(world_pos_matrix.x.floor(), 0.0, world_pos_matrix.y.floor())
    }

    /// translate grid pos into screen space coords
    pub(crate) fn world_pos_to_screen_pos(&self, world_pos: Pos) -> Vec3 {
        let world_to_screen_transform_matrix = self.world_to_screen_transform_matrix();
        let UnorderedPos { x, y, z } = world_pos.into();

        let pos_as_matrix = Matrix1x2::new(x, z);
        let mut screen_pos = pos_as_matrix * world_to_screen_transform_matrix;
        screen_pos.y += y * self.tile_size / 2.0;

        let z_index = -(x * 0.001) + -(z * 0.01) + (y * 0.01);
        Vec3::new(screen_pos.x, screen_pos.y, z_index)
    }

    fn world_to_screen_transform_matrix(&self) -> Matrix2<f32> {
        let a = 0.5 * self.tile_size;
        let b = -(0.5 * self.tile_size);
        let c = 0.25 * self.tile_size;
        let d = 0.25 * self.tile_size;

        Matrix2::new(a, c, b, d)
    }

    fn screen_to_world_transform_matrix(&self) -> Option<Matrix2<f32>> {
        self.world_to_screen_transform_matrix().try_inverse()
    }

    /// Get distance between centre of tile sprite and centre of top face
    pub(crate) fn tile_y_offset(&self) -> f32 {
        self.tile_size / 2.0
    }
}
