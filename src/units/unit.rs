use bevy::{prelude::*, sprite::Anchor};
use pathfinding::{num_traits::ToPrimitive, prelude::astar};

use crate::tile_map::{DeselectUnitEvent, Map, Pos, SelectUnitEvent, Tile};

use super::movement::ChangeFacingEvent;

#[derive(Copy, Clone, Debug)]
pub enum SelectMode {
    Move,
}
#[derive(Copy, Clone, Default, Debug)]
pub enum SelectedUnit {
    #[default]
    None,
    Some {
        entity: Entity,
        mode: SelectMode,
    },
}

impl SelectedUnit {
    pub fn is_some(&self) -> bool {
        match self {
            SelectedUnit::None => false,
            SelectedUnit::Some { entity: _, mode: _ } => true,
        }
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

#[derive(Default)]
pub(crate) struct KnightSprites {
    pub(crate) knight_north_east: Handle<Image>,
    pub(crate) knight_north_west: Handle<Image>,
    pub(crate) knight_south_east: Handle<Image>,
    pub(crate) knight_south_west: Handle<Image>,
}

#[derive(Clone, Copy)]
pub enum Direction {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}
#[derive(Component)]
pub struct Unit {
    pub(crate) pos: Pos,
    pub tile: Entity,
    pub(crate) move_speed: f32,
    pub(crate) move_distance: usize,
    pub(crate) facing: Direction,
    pub(crate) sprites: Sprites,
}

pub(crate) struct Sprites {
    north_east: Handle<Image>,
    north_west: Handle<Image>,
    south_east: Handle<Image>,
    south_west: Handle<Image>,
}

impl Unit {
    pub(crate) fn get_valid_moves(&self, map: &Map) -> Vec<Pos> {
        let mut valid_moves: Vec<Pos> = vec![];

        // for each allowed move, get the frontier from each tile reachable by previous moves
        // return all reachable tiles
        let mut prev_frontier: Vec<Pos> = vec![self.pos];

        for _depth in 0..self.move_distance {
            let mut iteration_frontier = vec![];

            for pos in prev_frontier {
                iteration_frontier.append(&mut map.get_frontier(pos));
            }

            // clone because append leaves empty vec behind!
            valid_moves.append(&mut iteration_frontier.clone());
            prev_frontier = iteration_frontier;
        }

        let mut filtered_moves = valid_moves
            .into_iter()
            // remove current position from valid moves
            .filter(|pos| *pos != self.pos)
            .collect::<Vec<Pos>>();

        filtered_moves.dedup();

        filtered_moves
    }

    pub(crate) fn get_path(&self, target_pos: Pos, map: &Map) -> Vec<Pos> {
        let (mut path, _) = astar(
            &self.pos,
            |p| p.successors(map),
            |p| (p.distance(&target_pos) / 3.0) as u32,
            |p| p == &target_pos,
        )
        .expect("No path found");

        path.retain(|c| *c != self.pos);

        path
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KnightSprites::default())
            .insert_resource(SelectedUnit::default())
            .add_startup_system(Self::load_unit_graphics)
            .add_startup_system(Self::spawn_knight.after(Self::load_unit_graphics))
            .add_system(Self::select_unit)
            .add_system(Self::deselect_unit)
            .add_system(Self::change_facing);
    }
}

impl UnitPlugin {
    fn load_unit_graphics(asset_server: Res<AssetServer>, mut graphics: ResMut<KnightSprites>) {
        let knight_north_east_handle =
            asset_server.load::<Image, &str>("units/knight/knight_north_east.png");
        let knight_north_west_handle =
            asset_server.load::<Image, &str>("units/knight/knight_north_west.png");
        let knight_south_east_handle =
            asset_server.load::<Image, &str>("units/knight/knight_south_east.png");
        let knight_south_west_handle =
            asset_server.load::<Image, &str>("units/knight/knight_south_west.png");

        graphics.knight_north_east = knight_north_east_handle;
        graphics.knight_north_west = knight_north_west_handle;
        graphics.knight_south_east = knight_south_east_handle;
        graphics.knight_south_west = knight_south_west_handle;
    }

    fn select_unit(
        mut selected_unit: ResMut<SelectedUnit>,
        mut events: EventReader<SelectUnitEvent>,
    ) {
        for SelectUnitEvent(unit_entity) in events.iter() {
            *selected_unit = SelectedUnit::Some {
                entity: *unit_entity,
                mode: SelectMode::Move,
            }
        }
    }

    fn deselect_unit(
        mut selected_unit: ResMut<SelectedUnit>,
        mut events: EventReader<DeselectUnitEvent>,
    ) {
        for DeselectUnitEvent(_unit_entity) in events.iter() {
            *selected_unit = SelectedUnit::None;
        }
    }

    fn change_facing(
        mut events: EventReader<ChangeFacingEvent>,
        mut unit_query: Query<(&Unit, &mut Handle<Image>)>,
    ) {
        for ChangeFacingEvent(entity, direction) in events.iter() {
            let (unit, mut sprite) = unit_query.get_mut(*entity).expect("No unit for entity");

            *sprite = match *direction {
                Direction::NorthEast => unit.sprites.north_east.clone(),
                Direction::NorthWest => unit.sprites.north_west.clone(),
                Direction::SouthEast => unit.sprites.south_east.clone(),
                Direction::SouthWest => unit.sprites.south_west.clone(),
            }
        }
    }

    fn spawn_knight(mut commands: Commands, graphics: Res<KnightSprites>, map_query: Query<&Map>) {
        let map = map_query.get_single().expect("Not exactly 1 map");
        let starting_pos = Pos::new(5.0, 0.0, 3.0);

        let tile_entity = map.tiles.get(&starting_pos).expect("No such tile");
        let screen_coords = map.world_pos_to_unit_screen_pos_absolute(starting_pos);

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(screen_coords),
                texture: graphics.knight_north_west.clone(),
                sprite: Sprite {
                    // roughly the sprite's feet
                    anchor: Anchor::Custom(Vec2::new(0.0, -0.4)),
                    ..default()
                },
                ..default()
            })
            .insert(Unit {
                tile: *tile_entity,
                pos: starting_pos,
                facing: Direction::NorthWest,
                move_speed: 0.8,
                move_distance: 3,
                sprites: Sprites {
                    north_east: graphics.knight_north_east.clone(),
                    north_west: graphics.knight_north_west.clone(),
                    south_east: graphics.knight_south_east.clone(),
                    south_west: graphics.knight_south_west.clone(),
                },
            });
    }
}
