use crate::PreStartupSystemLabels;
use bevy::prelude::*;

#[derive(Default)]
pub(crate) struct MapSprites {
    pub full_tile: Vec<Handle<Image>>,
    pub half_tile: Handle<Image>,
    pub tile_hover_overlay: Handle<Image>,
    pub tile_valid_move_overlay: Handle<Image>,
}

pub(crate) struct MapGraphicsPlugin;

impl Plugin for MapGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapSprites::default())
            .add_startup_system_to_stage(
                StartupStage::PreStartup,
                Self::load_graphics.label(PreStartupSystemLabels::LoadGraphics),
            );
    }
}

impl MapGraphicsPlugin {
    fn load_graphics(asset_server: Res<AssetServer>, mut graphics: ResMut<MapSprites>) {
        let tile1_handle = asset_server.load::<Image, &str>("tiles/full/tile1.png");
        let tile2_handle = asset_server.load::<Image, &str>("tiles/full/tile2.png");
        let tile3_handle = asset_server.load::<Image, &str>("tiles/full/tile3.png");
        let tile4_handle = asset_server.load::<Image, &str>("tiles/full/tile4.png");
        let tile5_handle = asset_server.load::<Image, &str>("tiles/full/tile5.png");
        let half_tile_handle = asset_server.load::<Image, &str>("tiles/half-tile.png");
        let tile_hover_handle = asset_server.load::<Image, &str>("tile_hover.png");

        graphics.full_tile = vec![
            tile1_handle,
            tile2_handle,
            tile3_handle,
            tile4_handle,
            tile5_handle,
        ];
        graphics.tile_hover_overlay = tile_hover_handle;
        graphics.half_tile = half_tile_handle;
    }
}
