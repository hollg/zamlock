use super::{coordinates::Coordinates, graphics::MapSprites};
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

pub(crate) const TILE_SIZE: f32 = 32.0;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileHeight {
    Full,
    Half,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Pos(pub(crate) u32, pub(crate) u32);

pub struct TileBuilder {
    pub accessible: bool,
    /// cartesian, in relation to layer
    pub(crate) pos: Pos,
    pub(crate) height: TileHeight,
}

#[derive(Component, Copy, Clone)]
pub struct Tile {
    /// cartesian, in relation to layer
    pub(crate) pos: Pos,
    pub(crate) height: TileHeight,
}

impl Tile {
    pub(crate) fn spawn(
        &self,
        entity: Entity,
        commands: &mut Commands,
        graphics: &Res<MapSprites>,
    ) -> Entity {
        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                texture: graphics
                    .full_tile
                    .choose(&mut thread_rng())
                    .expect("no tile sprites")
                    .clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(self.pos.to_isometric()),
                ..default()
            })
            .insert(*self)
            .id()
    }
}
