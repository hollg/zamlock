use bevy::{prelude::*, sprite::Anchor};
use pathfinding::prelude::astar;

use crate::tile_map::{DeselectUnitEvent, Map, Pos, SelectUnitEvent};

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
struct VillagerSprites {
    texture_atlas: Handle<TextureAtlas>,
    sw_index: usize,
    se_index: usize,
    ne_index: usize,
    nw_index: usize,
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
    anchor: Anchor,
}

pub(crate) struct Sprites {
    texture_atlas: Handle<TextureAtlas>,
    north_east: usize,
    north_west: usize,
    south_east: usize,
    south_west: usize,
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

    fn spawn_villager(
        commands: &mut Commands,
        graphics: &Res<VillagerSprites>,
        screen_coords: Vec3,
        tile_entity: Entity,
        starting_pos: Pos,
    ) {
        let unit = Unit {
            tile: tile_entity,
            pos: starting_pos,
            facing: Direction::SouthWest,
            move_speed: 0.8,
            move_distance: 3,
            sprites: Sprites {
                texture_atlas: graphics.texture_atlas.clone(),
                north_east: graphics.ne_index,
                north_west: graphics.nw_index,
                south_east: graphics.se_index,
                south_west: graphics.sw_index,
            },
            anchor: Anchor::Custom(Vec2::new(0.0, -0.3)),
        };

        let mut sprite = TextureAtlasSprite::new(graphics.sw_index);
        sprite.anchor = unit.anchor.clone();

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite,
                texture_atlas: unit.sprites.texture_atlas.clone(),
                transform: Transform::from_translation(screen_coords),
                ..default()
            })
            .insert(unit);
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VillagerSprites::default())
            .insert_resource(SelectedUnit::default())
            .add_startup_system(Self::load_unit_graphics)
            .add_startup_system(Self::spawn_villagers.after(Self::load_unit_graphics))
            .add_system(Self::select_unit)
            .add_system(Self::deselect_unit)
            .add_system(Self::change_facing);
    }
}

impl UnitPlugin {
    fn load_unit_graphics(
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut graphics: ResMut<VillagerSprites>,
    ) {
        let image_handle = assets.load("units/basic-sheet.png");
        let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(128.0));

        let south_west_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::splat(0.0),
            max: Vec2::new(32.0, 48.0),
        });
        let south_east_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(32.0, 0.0),
            max: Vec2::new(64.0, 48.0),
        });
        let north_east_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(64.0, 0.0),
            max: Vec2::new(96.0, 48.0),
        });
        let north_west_index = atlas.add_texture(bevy::sprite::Rect {
            min: Vec2::new(96.0, 0.0),
            max: Vec2::new(128.0, 48.0),
        });

        let atlas_handle = texture_atlases.add(atlas);

        *graphics = VillagerSprites {
            texture_atlas: atlas_handle,
            sw_index: south_west_index,
            se_index: south_east_index,
            nw_index: north_west_index,
            ne_index: north_east_index,
        }
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
        mut unit_query: Query<(&Unit, &mut TextureAtlasSprite)>,
    ) {
        for ChangeFacingEvent(entity, direction) in events.iter() {
            let (unit, mut sprite) = unit_query.get_mut(*entity).expect("No unit for entity");

            *sprite = match *direction {
                Direction::NorthEast => TextureAtlasSprite::new(unit.sprites.north_east),
                Direction::NorthWest => TextureAtlasSprite::new(unit.sprites.north_west),
                Direction::SouthEast => TextureAtlasSprite::new(unit.sprites.south_east),
                Direction::SouthWest => TextureAtlasSprite::new(unit.sprites.south_west),
            };
            sprite.anchor = Anchor::Custom(Vec2::new(0.0, -0.3))
        }
    }

    fn spawn_villagers(
        mut commands: Commands,
        graphics: Res<VillagerSprites>,
        map_query: Query<&Map>,
    ) {
        let map = map_query.get_single().expect("Not exactly 1 map");
        let starting_pos1 = Pos::new(4.0, 0.0, 4.0);
        let starting_pos2 = Pos::new(6.0, 0.0, 4.0);

        let tile_entity1 = map.tiles.get(&starting_pos1).expect("No such tile");
        let tile_entity2 = map.tiles.get(&starting_pos2).expect("No such tile");
        let screen_coords1 = map.world_pos_to_unit_screen_pos_absolute(starting_pos1);
        let screen_coords2 = map.world_pos_to_unit_screen_pos_absolute(starting_pos2);

        Unit::spawn_villager(
            &mut commands,
            &graphics,
            screen_coords1,
            *tile_entity1,
            starting_pos1,
        );

        Unit::spawn_villager(
            &mut commands,
            &graphics,
            screen_coords2,
            *tile_entity2,
            starting_pos2,
        );
    }
}
