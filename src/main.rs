use bevy::prelude::*;

mod camera;
mod tile_map;

use camera::CameraPlugin;
use tile_map::TileMapPluginGroup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugins(TileMapPluginGroup)
        .run()
}
