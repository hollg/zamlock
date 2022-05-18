use std::collections::HashMap;

use bevy::prelude::*;

use super::{pos::Pos, graphics::MapSprites, tile::Tile};

#[derive(Component, Clone)]
pub struct Layer {
    pub entity: Entity,
    pub tiles: HashMap<Pos, Entity>,
}

impl Layer {
    pub(crate) fn new(commands: &mut Commands) -> Layer {
        let entity = commands
            .spawn()
            .insert_bundle(TransformBundle::default())
            .id();

        let layer = Layer {
            entity,
            tiles: HashMap::new(),
        };

        commands.entity(entity).insert(layer.clone());

        layer
    }

    pub(crate) fn insert_tile(
        &mut self,
        commands: &mut Commands,
        tile: Tile,
        graphics: &Res<MapSprites>,
    ) {
        let tile_entity = tile.spawn(commands.spawn().id(), commands, graphics);
        commands.entity(self.entity).add_child(tile_entity);
        self.tiles.insert(tile.pos, tile_entity);
    }
}
