use bevy::prelude::*;

mod camera;

use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .run()
}
