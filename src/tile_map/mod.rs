use crate::{PreStartupSystemLabels, TILE_SIZE};
use bevy::prelude::*;

mod graphics;
mod map;
mod picking;
mod pos;
mod tile;

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
        let mut map = Map::new(
            commands.spawn().id(),
            TILE_SIZE,
            Vec3::new(0.0, -100.0, 0.0),
        );

        for x in 0..10 {
            for z in 0..10 {
                let x = x as f32;
                let z = z as f32;
                let pos = Pos::new(x, 0.0, z);

                let tile = Tile {
                    height: TileHeight::Full,
                    size: map.tile_size,
                };

                map.insert_tile(&mut commands, pos, tile, &graphics);
            }
        }

        let pos = Pos::new(0.0, 1.0, 0.0);

        let tile = Tile {
            height: TileHeight::Full,
            size: map.tile_size,
        };

        map.insert_tile(&mut commands, pos, tile, &graphics);

        let pos = Pos::new(1.0, 0.5, 0.0);
        let tile = Tile {
            height: TileHeight::Half,
            size: map.tile_size,
        };

        map.insert_tile(&mut commands, pos, tile, &graphics);

        let pos = Pos::new(0.0, 1.5, 0.0);
        let tile = Tile {
            height: TileHeight::Half,
            size: map.tile_size,
        };

        map.insert_tile(&mut commands, pos, tile, &graphics);

        let pos = Pos::new(5.0, 1.0, 5.0);
        let tile = Tile {
            height: TileHeight::Full,
            size: map.tile_size,
        };

        map.insert_tile(&mut commands, pos, tile, &graphics);

        let pos = Pos::new(5.0, 0.5, 4.0);
        let tile = Tile {
            height: TileHeight::Half,
            size: map.tile_size,
        };

        map.insert_tile(&mut commands, pos, tile, &graphics);

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
