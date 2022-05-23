use bevy::prelude::*;
use std::collections::VecDeque;

use crate::{
    tile_map::Map,
    units::unit::{SelectedUnit, Unit},
};

#[derive(Component)]
pub(super) struct Moving {
    pub(super) path: VecDeque<Vec3>,
}

#[derive(Default)]
struct ValidMoveGraphics {
    overlay: Handle<Image>,
}
#[derive(Component)]
struct ValidMoveOverlay;

#[derive(Component)]
pub struct ValidMove;
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ValidMoveGraphics::default())
            .add_startup_system(Self::load_overlay_graphic)
            .add_system(Self::highlight_valid_moves.after("click_tile"));
    }
}

impl MovementPlugin {
    fn load_overlay_graphic(assets: Res<AssetServer>, mut graphics: ResMut<ValidMoveGraphics>) {
        let handle = assets.load("tiles/tile_overlay.png");

        graphics.overlay = handle;
    }

    fn highlight_valid_moves(
        mut commands: Commands,
        selected_unit: Res<SelectedUnit>,
        unit_query: Query<&Unit>,
        map_query: Query<&Map>,
        overlay_query: Query<Entity, With<ValidMoveOverlay>>,
        valid_move_query: Query<Entity, With<ValidMove>>,
        graphics: Res<ValidMoveGraphics>,
    ) {
        if selected_unit.is_none() {
            for entity in overlay_query.iter() {
                commands.entity(entity).despawn();
            }

            for entity in valid_move_query.iter() {
                commands.entity(entity).remove::<ValidMove>();
            }

            return;
        }
        let map = map_query.get_single().expect("Not exactly one map!");

        if let SelectedUnit::Some { entity, mode: _ } = *selected_unit {
            let unit = unit_query.get(entity).expect("No entity for selected unit");
            let valid_moves = unit.get_valid_moves(map);

            for pos in valid_moves.iter() {
                let tile_entity = map.tiles.get(pos).expect("No tile for entity");

                let overlay = commands
                    .spawn_bundle(SpriteBundle {
                        texture: graphics.overlay.clone(),
                        transform: Transform::from_xyz(0.0, 8.0, 0.0001),
                        ..default()
                    })
                    .insert(ValidMoveOverlay)
                    .id();

                commands
                    .entity(*tile_entity)
                    .insert(ValidMove)
                    .add_child(overlay);
            }
        }
    }
}
