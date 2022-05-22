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
        tile.spawn(
            tile_entity,
            commands,
            graphics,
            self.world_pos_to_screen_pos(pos),
        );

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
    ///
    /// Returns translation relative to `self.translation`, i.e. suitable for child entitites.
    ///
    /// For a non-relative result, use `map.world_pos_to_screen_pos_absolute()`
    pub(crate) fn world_pos_to_screen_pos(&self, world_pos: Pos) -> Vec3 {
        let world_to_screen_transform_matrix = self.world_to_screen_transform_matrix();
        let UnorderedPos { x, y, z } = world_pos.into();

        let pos_as_matrix = Matrix1x2::new(x, z);
        let mut screen_pos = pos_as_matrix * world_to_screen_transform_matrix;
        screen_pos.y += y * self.tile_size / 2.0;

        let z_index = -(x * 0.001) + -(z * 0.01) + (y * 0.01);
        Vec3::new(screen_pos.x, screen_pos.y, z_index)
    }

    pub(crate) fn world_pos_to_screen_pos_absolute(&self, world_pos: Pos) -> Vec3 {
        self.world_pos_to_screen_pos(world_pos) + self.translation
    }

    // maths courtesy of https://www.youtube.com/watch?v=04oQ2jOUjkU
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

    pub(crate) fn tile_height(&self) -> f32 {
        self.tile_size / 2.0
    }

    /// Returns distance between centre of tile sprite ("closest" vertex to camera)
    /// and centre of top face
    pub fn tile_top_y_offset(&self) -> f32 {
        self.tile_height() / 2.0
    }

    // Returns the screen coords for the centre of the top face of a tile in the provided
    // grid position. Use this to place things "on" tiles
    pub fn world_pos_to_unit_screen_pos_absolute(&self, world_pos: Pos) -> Vec3 {
        let mut coords = self.world_pos_to_screen_pos_absolute(world_pos);
        coords.y += self.tile_top_y_offset();
        // in front of tile highlight sprite, behind taller tiles
        coords.z += 0.005;
        coords
    }

    /// Return `Pos` of all existent `Tile`s reachable in one stop from given `Po`.
    ///
    /// N.B. movement in the y axis happens 0.5 at a time
    pub fn get_frontier(&self, pos: Pos) -> Vec<Pos> {
        let UnorderedPos { x, y, z } = pos.into();

        let mut frontier: Vec<Pos> = vec![];

        // Current elevation
        let level_plus_x = Pos::new(x + 1.0, y, z);
        if self.tiles.get(&level_plus_x).is_some() && !self.is_pos_covered(level_plus_x) {
            frontier.push(level_plus_x);
        }
        let level_minus_x = Pos::new(x - 1.0, y, z);
        if self.tiles.get(&level_minus_x).is_some() && !self.is_pos_covered(level_minus_x) {
            frontier.push(level_minus_x);
        }

        let level_plus_z = Pos::new(x, y, z + 1.0);
        if self.tiles.get(&level_plus_z).is_some() && !self.is_pos_covered(level_plus_z) {
            frontier.push(level_plus_z);
        }
        let level_minus_z = Pos::new(x, y, z - 1.0);
        if self.tiles.get(&level_minus_z).is_some() && !self.is_pos_covered(level_minus_z) {
            frontier.push(level_minus_z);
        }

        // Higher elevation
        let higher_plus_x = Pos::new(x + 1.0, y + 0.5, z);
        if self.tiles.get(&higher_plus_x).is_some() && !self.is_pos_covered(higher_plus_x) {
            frontier.push(higher_plus_x);
        }
        let higher_minus_x = Pos::new(x - 1.0, y + 0.5, z);
        if self.tiles.get(&higher_minus_x).is_some() && !self.is_pos_covered(higher_minus_x) {
            frontier.push(higher_minus_x);
        }

        let higher_plus_z = Pos::new(x, y + 0.5, z + 1.0);
        if self.tiles.get(&higher_plus_z).is_some() && !self.is_pos_covered(higher_plus_z) {
            frontier.push(higher_plus_z);
        }
        let higher_minus_z = Pos::new(x, y + 0.5, z - 1.0);
        if self.tiles.get(&higher_minus_z).is_some() && !self.is_pos_covered(higher_minus_z) {
            frontier.push(higher_minus_z);
        }

        // Lower elevation
        let lower_plus_x = Pos::new(x + 1.0, y - 0.5, z);
        if self.tiles.get(&lower_plus_x).is_some() && !self.is_pos_covered(lower_plus_x) {
            frontier.push(lower_plus_x);
        }
        let lower_minus_x = Pos::new(x - 1.0, y - 0.5, z);
        if self.tiles.get(&lower_minus_x).is_some() && !self.is_pos_covered(lower_minus_x) {
            frontier.push(lower_minus_x);
        }

        let lower_plus_z = Pos::new(x, y - 0.5, z + 1.0);
        if self.tiles.get(&lower_plus_z).is_some() && !self.is_pos_covered(lower_plus_z) {
            frontier.push(lower_plus_z);
        }
        let lower_minus_z = Pos::new(x, y - 0.5, z - 1.0);
        if self.tiles.get(&lower_minus_z).is_some() && !self.is_pos_covered(lower_minus_z) {
            frontier.push(lower_minus_z);
        }

        frontier
    }

    /// return `true` if there is a `Tile` directly above provided `Pos` on y axis,
    /// else return `false
    fn is_pos_covered(&self, pos: Pos) -> bool {
        self.tiles
            .get(&Pos::new(pos.x, pos.y + 0.5, pos.z))
            .is_some()
            || self
                .tiles
                .get(&Pos::new(pos.x, pos.y + 1.0, pos.z))
                .is_some()
    }
}
