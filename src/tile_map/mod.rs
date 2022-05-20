use crate::{PreStartupSystemLabels, TILE_SIZE};
use bevy::prelude::*;

mod graphics;
mod layer;
mod map;
mod picking;
mod pos;
mod tile;

use layer::Layer;
use map::Map;
use picking::TilePickingPlugin;
use pos::Pos;
use tile::{Tile, TileHeight};

use self::graphics::MapSprites;

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
        let mut map = Map::new(commands.spawn().id(), TILE_SIZE, -100.0);
        let mut layer = Layer::new(0, &mut commands);

        for y in 0..10 {
            for x in 0..10 {
                let pos = Pos(x, y);
                let tile = Tile {
                    pos,
                    height: TileHeight::Half,
                    size: map.tile_size,
                };

                layer.insert_tile(&mut commands, tile, &graphics);
            }
        }

        let mut layer2 = Layer::new(1, &mut commands);

        layer2.insert_tile(
            &mut commands,
            Tile {
                pos: Pos(0, 0),
                height: TileHeight::Full,
                size: map.tile_size,
            },
            &graphics,
        );

        layer2.insert_tile(
            &mut commands,
            Tile {
                pos: Pos(0, 1),
                height: TileHeight::Half,
                size: map.tile_size,
            },
            &graphics,
        );

        map.insert_layers(&mut commands, &[layer, layer2]);

        map.spawn(&mut commands);
    }
}

pub struct TileMapPluginGroup;

impl PluginGroup for TileMapPluginGroup {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(graphics::MapGraphicsPlugin)
            .add(TileMapPlugin)
            .add(TilePickingPlugin);
    }
}
