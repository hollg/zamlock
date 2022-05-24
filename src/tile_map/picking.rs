use bevy::prelude::*;

use crate::{
    camera::{mouse_pos_to_screen_pos, MainCamera},
    units::{SelectedUnit, Unit, ValidMove},
};

use super::{graphics::MapSprites, map::Map, pos::Pos, tile::Tile};

pub(crate) struct TilePickingPlugin;

pub struct ActiveTile(pub Option<Entity>);
pub struct SelectUnitEvent(pub Entity);
pub struct DeselectUnitEvent(pub Entity);
pub struct SetPathEvent(pub Entity, pub Pos);

#[derive(Component)]
struct Highlight;

impl Plugin for TilePickingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveTile(None))
            .add_event::<SelectUnitEvent>()
            .add_event::<DeselectUnitEvent>()
            .add_event::<SetPathEvent>()
            .add_system_to_stage(CoreStage::PreUpdate, Self::set_active_tile)
            .add_system_to_stage(
                CoreStage::PreUpdate,
                Self::hover_tile.after(Self::set_active_tile),
            )
            .add_system(Self::click_tile.label("click_tile"));
    }
}

impl TilePickingPlugin {
    /// Populates `ActiveTile` resource with the entity for the tile that the mouse is hovering ove (if any).
    fn set_active_tile(
        wnds: Res<Windows>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        mut active_tile: ResMut<ActiveTile>,
        map_query: Query<&Map>,
    ) {
        let map = map_query.get_single().expect("No map!");
        let mut new_active_tile: ActiveTile = ActiveTile(None);

        if let Some(screen_pos) = mouse_pos_to_screen_pos(wnds, q_camera) {
            let mut picked: Option<(Pos, Entity)> = None;

            // sort tiles highest elevation first
            let mut tiles = map.tiles.iter().collect::<Vec<(&Pos, &Entity)>>();
            tiles.sort_by(|(point_a, _), (point_b, _)| point_b.y.cmp(&point_a.y));

            for (pos, entity) in tiles.iter() {
                if picked.is_some() {
                    break;
                }

                // translate mouse pos to ground level grid coord and pick current tile
                // if it matches on x and z coords
                let y_offset = map.tile_height() * f32::from(pos.y);
                let offset_screen_pos = Vec2::new(screen_pos.x, screen_pos.y - y_offset);
                let offset_world_pos = map.screen_pos_to_world_pos(offset_screen_pos);

                if pos.x == offset_world_pos.x && pos.z == offset_world_pos.z {
                    picked = Some((**pos, **entity));
                }
            }

            if let Some((picked_pos, picked_entity)) = picked {
                // can't pick tiles that are underneath others
                if !tiles.iter().any(|(pos, _)| {
                    pos.x == picked_pos.x && pos.z == picked_pos.z && pos.y > picked_pos.y
                }) {
                    new_active_tile = ActiveTile(Some(picked_entity));
                }
            }
        }

        *active_tile = new_active_tile;
    }

    /// Adds highlight sprite to `ActiveTile`
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
                transform: Transform::from_xyz(0.0, tile.size / 4.0, 0.001),
                ..default()
            })
            .insert(Highlight)
            .id();

        commands.entity(tile_entity).add_child(highlight);
    }

    /// Send TileClickEvent when click occurs
    /// while there is an `ActiveTile`
    // TODO: handle all clicks from here by sending different events depending on whether
    // there is a SelectedUnit, ActiveTile etc
    fn click_tile(
        active_tile: Res<ActiveTile>,
        selected_unit: Res<SelectedUnit>,
        mouse: Res<Input<MouseButton>>,
        mut select_events: EventWriter<SelectUnitEvent>,
        mut deselect_events: EventWriter<DeselectUnitEvent>,
        mut set_path_events: EventWriter<SetPathEvent>,
        unit_query: Query<(Entity, &Unit)>,
        valid_move_query: Query<(&Tile, Option<&ValidMove>)>,
    ) {
        if !mouse.just_pressed(MouseButton::Left) {
            return;
        }

        // clicked on a tile
        if let ActiveTile(Some(tile_entity)) = *active_tile {
            match *selected_unit {
                SelectedUnit::None => {
                    if let Some((unit_entity, _unit)) =
                        unit_query.iter().find(|(_e, u)| u.tile == tile_entity)
                    {
                        select_events.send(SelectUnitEvent(unit_entity))
                    }
                }
                SelectedUnit::Some {
                    entity: unit_entity,
                    mode: _,
                } => {
                    let (tile, valid_move) = valid_move_query
                        .get(tile_entity)
                        .expect("No tile for selected entity");

                    // Will need to check here for other interaction types in the future
                    if valid_move.is_some() {
                        set_path_events.send(SetPathEvent(unit_entity, tile.pos))
                    }

                    deselect_events.send(DeselectUnitEvent(unit_entity));
                }
            }
        }
    }
}
