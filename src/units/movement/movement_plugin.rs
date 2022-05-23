use bevy::prelude::*;
use std::collections::VecDeque;

use crate::{
    tile_map::{Map, Pos, SetPathEvent},
    units::unit::{SelectedUnit, Unit},
};

use super::super::Direction;

#[derive(Component)]
pub(super) struct Moving {
    pub(super) path: VecDeque<Pos>,
}

pub struct ChangeFacingEvent(pub Entity, pub Direction);
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
            .add_event::<ChangeFacingEvent>()
            .add_startup_system(Self::load_overlay_graphic)
            .add_system(Self::highlight_valid_moves.after("click_tile"))
            .add_system(Self::set_unit_path)
            .add_system(Self::move_units);
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

    fn set_unit_path(
        mut commands: Commands,
        mut events: EventReader<SetPathEvent>,
        mut unit_query: Query<(&mut Unit, Entity)>,
        map_query: Query<&Map>,
    ) {
        let map = map_query.get_single().expect("Not exactly one map");

        for SetPathEvent(unit_entity, target_pos) in events.iter() {
            let (unit, entity) = unit_query.get_mut(*unit_entity).unwrap();

            commands.entity(entity).insert(Moving {
                path: VecDeque::from(unit.get_path(*target_pos, map)),
            });
        }
    }

    fn move_units(
        mut commands: Commands,
        mut moving_unit_query: Query<(&mut Unit, &mut Transform, &mut Moving, Entity)>,
        map_query: Query<&Map>,
        mut event: EventWriter<ChangeFacingEvent>,
    ) {
        let map = map_query.get_single().expect("Not exactly one map");
        for (mut unit, mut transform, mut moving, entity) in moving_unit_query.iter_mut() {
            let next = moving.path[0];
            let next_tile_entity = map.tiles.get(&next).expect("No tile at next pos");

            let facing = get_facing(unit.pos, next);
            event.send(ChangeFacingEvent(entity, facing));

            let next_pos_isometric = map.world_pos_to_unit_screen_pos_absolute(next);
            let move_vector =
                (next_pos_isometric - transform.translation).normalize() * unit.move_speed;
            let next_translation = transform.translation + move_vector;

            *transform.translation = *next_translation;

            // when unit reaches next tile,
            if next_translation.distance(next_pos_isometric) < 0.5 {
                // pop it from path
                moving.path.pop_front();
                // update current tile for facing calculation on next frame
                unit.tile = *next_tile_entity;
                unit.pos = next;
            }

            // when unit reaches last tile, stop moving
            if moving.path.is_empty() {
                commands.entity(entity).remove::<Moving>();
            }
        }
    }
}

fn get_facing(current: Pos, next: Pos) -> Direction {
    if next.x > current.x && next.z == current.z {
        return Direction::NorthEast;
    }
    if next.x < current.x && next.z == current.z {
        return Direction::SouthWest;
    }
    if next.z > current.z && next.x == current.x {
        return Direction::NorthWest;
    }
    if next.z < current.z && next.x == current.x {
        return Direction::SouthEast;
    }

    Direction::SouthWest
}
