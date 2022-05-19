use bevy::prelude::*;

use crate::camera::{mouse_pos_to_screen_pos, MainCamera};

use super::{graphics::MapSprites, map::Map, tile::Tile};

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
        tile_query: Query<&Tile>,
    ) {
        let map = map_query.get_single().expect("No map!");
        let mut new_active_tile: ActiveTile = ActiveTile(None);

        if let Some(screen_pos) = mouse_pos_to_screen_pos(wnds, q_camera) {
            let mut picked: Option<(Tile, Entity, usize)> = None;

            for layer in map.layers.iter().rev() {
                if picked.is_some() {
                    break;
                }
                // offset to layer 0 to find grid coords
                let layer_offset = (map.tile_size / 4.0) * layer.index as f32;
                let layer_offset_screen_pos = Vec2::new(screen_pos.x, screen_pos.y - layer_offset);
                let layer_offset_world_pos = map.screen_pos_to_world_pos(layer_offset_screen_pos);

                if let Some(tile_entity) = layer.tiles.get(&layer_offset_world_pos) {
                    let tile = tile_query
                        .get(*tile_entity)
                        .expect("No tile for picked tile entity");
                    picked = Some((*tile, *tile_entity, layer.index));
                }
            }

            if let Some((picked_tile, picked_entity, picked_layer_index)) = picked {
                // can't pick tiles that are underneath others
                if !map.layers.iter().any(|layer| {
                    layer.index > picked_layer_index && layer.tiles.get(&picked_tile.pos).is_some()
                }) {
                    new_active_tile = ActiveTile(Some(picked_entity))
                }
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
}
