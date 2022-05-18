use super::{graphics::MapSprites, pos::Pos};
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

pub(crate) const TILE_SIZE: f32 = 32.0;

#[derive(Copy, Clone, PartialEq)]
pub enum TileHeight {
    Full,
    Half,
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
        layer_index: usize,
    ) -> Entity {
        let sprite = match self.height {
            TileHeight::Full => graphics
                .full_tile
                .choose(&mut thread_rng())
                .expect("no tile sprites")
                .clone(),
            TileHeight::Half => graphics.half_tile.clone(),
        };

        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                texture: sprite,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(
                    self.pos.to_isometric(layer_index, self.height),
                ),
                ..default()
            })
            .insert(*self)
            .id()
    }
}
