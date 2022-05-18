use crate::PreStartupSystemLabels;

use bevy::prelude::*;

use layer::Layer;
use map::Map;
use pos::Pos;
use tile::{Tile, TileHeight};

use self::graphics::MapSprites;

mod graphics;
mod layer;
mod map;
mod pos;
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
        let mut layer = Layer::new(0, &mut commands);

        for x in 0..10 {
            for y in 0..10 {
                let pos = Pos(x, y);
                let tile = Tile {
                    pos,
                    height: TileHeight::Full,
                };

                layer.insert_tile(&mut commands, tile, &graphics);
            }
        }
        map.insert_layer(&mut commands, layer);

        let mut layer2 = Layer::new(1, &mut commands);

        for x in 0..10 {
            for y in 0..10 {
                if x % 2 == 0 && y % 3 == 0 {
                    let pos = Pos(x, y);
                    let tile = Tile {
                        pos,
                        height: TileHeight::Half,
                    };

                    layer2.insert_tile(&mut commands, tile, &graphics);
                }
            }
        }
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
