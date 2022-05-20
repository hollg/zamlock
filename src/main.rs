use bevy::prelude::*;

mod camera;
mod tile_map;
mod units;

use camera::CameraPlugin;
use tile_map::TileMapPluginGroup;
use units::UnitPlugin;

const TILE_SIZE: f32 = 32.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum PreStartupSystemLabels {
    LoadGraphics,
    SpawnEntities,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Zamlock".to_string(),
            width: 640.,
            height: 360.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(TileMapPluginGroup)
        .add_plugin(UnitPlugin)
        .add_plugin(CameraPlugin)
        .run()
}
