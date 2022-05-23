use super::{graphics::MapSprites, Pos};
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileHeight {
    Full,
    Half,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Tile {
    pub(crate) height: TileHeight,
    /// height/width in pixels (tile must be square!)
    pub(crate) size: f32,
    pub(crate) pos: Pos,
}

impl Tile {
    pub(crate) fn spawn(
        &self,
        entity: Entity,
        commands: &mut Commands,
        graphics: &Res<MapSprites>,
        translation: Vec3,
    ) -> Entity {
        commands
            .entity(entity)
            .insert_bundle(SpriteBundle {
                texture: graphics.get_tile(self.height),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(self.size)),
                    ..default()
                },
                transform: Transform::from_translation(translation),
                ..default()
            })
            .insert(*self)
            .id()
    }
}
