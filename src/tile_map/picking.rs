use bevy::prelude::*;

use crate::camera::{mouse_pos_to_screen_pos, MainCamera};

use super::{graphics::MapSprites, map::Map, pos::Pos, tile::Tile};

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
            );
    }
}

impl TilePickingPlugin {
    fn set_active_tile(
        wnds: Res<Windows>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        mut active_tile: ResMut<ActiveTile>,
        map_query: Query<&Map>,
    ) {
        let map = map_query.get_single().expect("No map!");
        let mut new_active_tile: ActiveTile = ActiveTile(None);

        if let Some(screen_pos) = mouse_pos_to_screen_pos(wnds, q_camera) {
            // let mut picked: Option<(Pos, Entity)> = None;
            let world_pos = map.screen_pos_to_world_pos(screen_pos);

            if let Some(entity) = map.tiles.get(&world_pos) {
                new_active_tile = ActiveTile(Some(*entity));
            }

            // let mut tiles = map.tiles.iter().collect::<Vec<(&Pos, &Entity)>>();
            // tiles.sort_by(|(point_a, _), (point_b, _)| point_b.2.cmp(&point_a.2));

            // for (p, e) in tiles.iter() {
            //     if picked.is_some() {
            //         break;
            //     }

            //     // offset to layer 0 to find grid coords
            //     let y_offset = (map.tile_size / 4.0) * f32::from(p.2);
            //     let layer_offset_screen_pos = Vec2::new(screen_pos.x, screen_pos.y - y_offset);
            //     let layer_offset_world_pos = map.screen_pos_to_world_pos(layer_offset_screen_pos);

            //     if **p == layer_offset_world_pos {
            //         picked = Some((**p, **e));
            //     }
            // }

            // if let Some((picked_pos, picked_entity)) = picked {
            //     // can't pick tiles that are underneath others
            //     if !map.tiles.iter().any(|(pos, _)| {
            //         pos.0 == picked_pos.0 && pos.1 == picked_pos.1 && pos.2 > picked_pos.2
            //     }) {
            //         new_active_tile = ActiveTile(Some(picked_entity))
            //     }
            // }
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
}
