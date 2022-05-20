use std::collections::HashMap;

use bevy::prelude::*;
use nalgebra::{Matrix1x2, Matrix2};

use super::{graphics::MapSprites, pos::Pos, tile::Tile};

#[derive(Component, Clone)]
pub struct Map {
    pub(crate) entity: Entity,

    pub(crate) tile_size: f32,
    pub(crate) tiles: HashMap<Pos, Entity>,
    pub y_offset: f32,
}

impl Map {
    pub(crate) fn new(entity: Entity, tile_size: f32, y_offset: f32) -> Map {
        Map {
            entity,
            tile_size,
            y_offset,
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
        tile.spawn(tile_entity, commands, graphics);

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
                local: Transform::from_xyz(0.0, self.y_offset, 0.0),
                ..default()
            });
    }

    pub(crate) fn screen_pos_to_world_pos(&self, screen_pos: Vec2) -> Pos {
        let a = 0.5 * self.tile_size;
        let b = -(0.5 * self.tile_size);
        let c = 0.25 * self.tile_size;
        let d = 0.25 * self.tile_size;

        let world_to_screen_transform_matrix = Matrix2::new(a, c, b, d);
        let screen_to_world_transform_matrix = world_to_screen_transform_matrix
            .try_inverse()
            .expect("Can't inverse matrix");

        let screen_pos_matrix =
            Matrix1x2::new(screen_pos.x as f32, screen_pos.y - self.y_offset as f32);

        let world_pos_matrix = screen_pos_matrix * screen_to_world_transform_matrix;

        Pos::new(world_pos_matrix.x.floor(), 0.0, world_pos_matrix.y.floor())
    }
}
