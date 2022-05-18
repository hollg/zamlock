use crate::PreStartupSystemLabels;

use bevy::prelude::*;

use pos::Pos;
use layer::Layer;
use map::Map;
use tile::{Tile, TileHeight};

use self::graphics::MapSprites;

mod pos;
mod graphics;
mod layer;
mod map;
mod tile;

struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(
            StartupStage::PreStartup,
            Self::spawn_map.after(PreStartupSystemLabels::LoadGraphics),
        );
    }
}

impl TileMapPlugin {
    fn spawn_map(mut commands: Commands, graphics: Res<MapSprites>) {
        let mut map = Map::new(commands.spawn().id());
        let mut layer = Layer::new(&mut commands);

        for x in 0..10 {
            for y in 0..10 {
                let pos = Pos(x, y);
                let tile = Tile {
                    pos,
                    height: TileHeight::Full,
                };

                let tile_entity = commands.spawn().insert(tile).id();

                // tile.spawn(tile_entity, &mut commands, &graphics);
                layer.insert_tile(&mut commands, tile, &graphics);
                layer.tiles.insert(pos, tile_entity);
                // commands.entity(layer.entity).add_child(tile_entity);
            }
        }
        let mut layer2 = layer.clone();

        map.insert_layer(&mut commands, layer);
        map.insert_layer(&mut commands, layer2);

        map.spawn(&mut commands);
    }
}

pub struct TileMapPluginGroup;

impl PluginGroup for TileMapPluginGroup {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(graphics::MapGraphicsPlugin).add(TileMapPlugin);
    }
}
