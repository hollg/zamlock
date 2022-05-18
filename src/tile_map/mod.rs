use std::collections::HashMap;

use bevy::prelude::*;

use layer::Layer;
use map::Map;
use tile::{Pos, Tile, TileHeight};

use self::graphics::MapSprites;

mod coordinates;
mod graphics;
mod layer;
mod map;
mod tile;

struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, Self::spawn_map);
    }
}

impl TileMapPlugin {
    fn spawn_map(mut commands: Commands, graphics: Res<MapSprites>) {
        let mut map = Map {
            entity: commands.spawn().id(),
            layers: vec![],
        };

        let mut layer = Layer {
            entity: commands.spawn().id(),
            tiles: HashMap::new(),
        };

        for x in 0..10 {
            for y in 0..10 {
                let pos = Pos(x, y);
                let tile = Tile {
                    pos,
                    height: TileHeight::Full,
                };

                let tile_entity = commands.spawn().insert(tile).id();

                tile.spawn(tile_entity, &mut commands, &graphics);

                layer.tiles.insert(pos, tile_entity);
                // commands.entity(layer.entity).add_child(tile_entity); 
            }
        }

        map.insert_layer(&mut commands, layer);

        map.spawn(&mut commands, &graphics);
    }
}

pub struct TileMapPluginGroup;

impl PluginGroup for TileMapPluginGroup {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(graphics::MapGraphicsPlugin).add(TileMapPlugin);
    }
}
