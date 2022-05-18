use std::collections::HashMap;

use bevy::prelude::*;

use super::{graphics::MapSprites, pos::Pos, tile::Tile};

#[derive(Component, Clone)]
pub struct Layer {
    pub(crate) entity: Entity,
    /// 0 is the lowest layer and higher layers are stacked above
    pub(crate) index: usize,
    pub(crate) tiles: HashMap<Pos, Entity>,
}

impl Layer {
    pub(crate) fn new(index: usize, commands: &mut Commands) -> Layer {
        let entity = commands
            .spawn()
            .insert_bundle(TransformBundle::default())
            .id();

        let layer = Layer {
            entity,
            index,
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
        let tile_entity = tile.spawn(commands.spawn().id(), commands, graphics, self.index);
        commands.entity(self.entity).add_child(tile_entity);
        self.tiles.insert(tile.pos, tile_entity);
    }
}
