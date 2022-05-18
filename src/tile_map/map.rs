use bevy::prelude::*;

use super::{graphics::MapSprites, layer::Layer};

#[derive(Component)]
pub struct Map {
    pub entity: Entity,
    pub layers: Vec<Layer>,
}

impl Map {
    pub(crate) fn insert_layer(&mut self, commands: &mut Commands, layer: Layer) {
        commands.entity(self.entity).add_child(layer.entity);
        self.layers.push(layer);
    }

    pub(crate) fn spawn(&self, commands: &mut Commands, graphics: &Res<MapSprites>) {
        commands
            .entity(self.entity)
            .insert_bundle(TransformBundle::default());

        for layer in &self.layers {
            // let layer_commands = commands.entity(layer.entity);

            commands.entity(self.entity).add_child(layer.entity);
        }
    }
}
