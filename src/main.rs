use bevy::prelude::*;

mod camera;
mod tile_map;

use camera::CameraPlugin;
use tile_map::TileMapPluginGroup;
const TILE_SIZE: f32 = 32.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum PreStartupSystemLabels {
    LoadGraphics,
    SpawnEntities,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TileMapPluginGroup)
        .add_plugin(CameraPlugin)
        .run()
}
