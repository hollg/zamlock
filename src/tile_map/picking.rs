use bevy::prelude::*;

use crate::camera::{mouse_pos_to_screen_pos, MainCamera};

use super::{
    graphics::MapSprites,
    map::Map,
    tile::{Tile, ToWorld},
};

pub(crate) struct TilePickingPlugin;

struct ActiveTile(Option<Entity>);

#[derive(Component)]
struct Highlight;

impl Plugin for TilePickingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveTile(None))
            .add_system_to_stage(CoreStage::PreUpdate, Self::set_active_tile)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                Self::hover_tile.after(Self::set_active_tile),
            )
            .add_system(Self::handle_tile_click.after(Self::set_active_tile));
    }
}

impl TilePickingPlugin {
    fn set_active_tile(
        wnds: Res<Windows>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        tile_query: Query<(&Tile, Entity)>,
        mut active_tile: ResMut<ActiveTile>,
        map_query: Query<&Map>,
    ) {
        let map = map_query.get_single().expect("No map!");
        let mut new_active_tile: ActiveTile = ActiveTile(None);

        if let Some(screen_pos) = mouse_pos_to_screen_pos(wnds, q_camera) {
            let world_pos = map.screen_pos_to_world_pos(screen_pos);

            if let Some((_tile, tile_entity)) = tile_query
                .iter()
                .find(|(tile, _entity)| tile.pos == world_pos)
            {
                new_active_tile = ActiveTile(Some(tile_entity));
            }
        }

        *active_tile = new_active_tile;
    }

    fn hover_tile(
        mut commands: Commands,
        tile_query: Query<&Tile>,
        highlight_query: Query<Entity, With<Highlight>>,
        graphics: Res<MapSprites>,
        active_tile: ResMut<ActiveTile>,
    ) {
        if active_tile.0 == None {
            if let Ok(highlight) = highlight_query.get_single() {
                commands.entity(highlight).despawn();
            }
            return;
        }

        let tile_entity = active_tile.0.expect("No active tile entity");
        let tile = tile_query
            .get(tile_entity)
            .expect("No tile for active tile entity");

        if let Ok(highlight) = highlight_query.get_single() {
            if highlight == tile_entity {
                return;
            } else {
                commands.entity(highlight).despawn();
            }
        }

        let highlight = commands
            .spawn_bundle(SpriteBundle {
                texture: graphics.tile_hover_overlay.clone(),
                transform: Transform::from_xyz(0.0, tile.size / 4.0, 0.01),
                ..default()
            })
            .insert(Highlight)
            .id();

        commands.entity(tile_entity).add_child(highlight);
    }

    fn handle_tile_click(
        mut commands: Commands,
        mouse_input: Res<Input<MouseButton>>,
        active_tile: Res<ActiveTile>,
        tile_query: Query<&Tile>,
    ) {
        if active_tile.0 == None {
            return;
        }

        if !mouse_input.just_pressed(MouseButton::Left) {
            return;
        }

        let tile_entity = active_tile.0.expect("No active tile entity");
        let tile = tile_query
            .get(tile_entity)
            .expect("No tile for active tile entity");

        let thing = commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0.0, tile.get_y_offset(), 20.0)),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(2.0)),
                    color: Color::rgb(0., 0., 0.),
                    ..default()
                },
                ..default()
            })
            .id();

        commands.entity(tile_entity).add_child(thing);
    }
}
