use bevy::prelude::*;

mod camera;
mod tile_map;

use camera::CameraPlugin;
use tile_map::TileMapPluginGroup;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum PreStartupSystemLabels {
    LoadGraphics,
    SpawnEntities,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugins(TileMapPluginGroup)
        .run()
}
