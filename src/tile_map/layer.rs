use std::collections::HashMap;

use bevy::prelude::*;

use super::tile::{Pos, Tile, TileBuilder};

pub struct Layer {
    pub entity: Entity,
    pub tiles: HashMap<Pos, Entity>,
}

impl Layer {
    fn new(entity: Entity) -> Layer {
        Layer {
            entity,
            tiles: HashMap::new(),
        }
    }

    fn insert_tile(&mut self, commands: &mut Commands, tile: TileBuilder) {
        let tile_entity = commands
            .spawn()
            .insert(Tile {
                pos: tile.pos,
                height: tile.height,
            })
            .id();

        self.tiles.insert(tile.pos, tile_entity);
    }
}
