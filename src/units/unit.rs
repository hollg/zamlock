use bevy::{prelude::*, sprite::Anchor};

use crate::tile_map::{Map, Pos};

#[derive(Default)]
pub(crate) struct UnitSprites {
    pub(crate) man_north_east: Handle<Image>,
    pub(crate) man_north_west: Handle<Image>,
    pub(crate) man_south_east: Handle<Image>,
    pub(crate) man_south_west: Handle<Image>,
}

#[derive(Component)]
struct Unit {
    pub(crate) pos: Pos,
    pub(crate) tile: Entity,
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitSprites::default())
            .add_startup_system(Self::load_unit_graphics)
            .add_startup_system(Self::spawn_unit.after(Self::load_unit_graphics));
    }
}

impl UnitPlugin {
    fn load_unit_graphics(asset_server: Res<AssetServer>, mut graphics: ResMut<UnitSprites>) {
        let man_north_east_handle =
            asset_server.load::<Image, &str>("units/man/man_north_east.png");
        let man_north_west_handle =
            asset_server.load::<Image, &str>("units/man/man_north_west.png");
        let man_south_east_handle =
            asset_server.load::<Image, &str>("units/man/man_south_east.png");
        let man_south_west_handle =
            asset_server.load::<Image, &str>("units/man/man_south_west.png");

        graphics.man_north_east = man_north_east_handle;
        graphics.man_north_west = man_north_west_handle;
        graphics.man_south_east = man_south_east_handle;
        graphics.man_south_west = man_south_west_handle;
    }

    fn spawn_unit(mut commands: Commands, graphics: ResMut<UnitSprites>, map_query: Query<&Map>) {
        let map = map_query.get_single().expect("Not exactly 1 map");
        let player_starting_pos = Pos::new(6.0, 0.0, 5.0);
        // let player_starting_pos = Pos::new(1.0, 0.5, 0.0);

        let tile_entity = map.tiles.get(&player_starting_pos).expect("No such tile");
        let screen_coords = map.world_pos_to_unit_screen_pos_absolute(player_starting_pos);

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(screen_coords),
                texture: graphics.man_south_east.clone(),
                sprite: Sprite {
                    // roughly the sprite's feet
                    anchor: Anchor::Custom(Vec2::new(0.0, -0.4)),
                    ..default()
                },
                ..default()
            })
            .insert(Unit {
                tile: *tile_entity,
                pos: player_starting_pos,
            });
    }
}
