use bevy::prelude::*;

use super::layer::Layer;

#[derive(Component)]
pub struct Map {
    pub entity: Entity,
    pub layers: Vec<Layer>,
}

impl Map {
    pub(crate) fn new(entity: Entity) -> Map {
        Map {
            entity,
            layers: vec![],
        }
    }

    pub(crate) fn insert_layer(&mut self, commands: &mut Commands, layer: Layer) {
        commands.entity(self.entity).add_child(layer.entity);
        self.layers.push(layer);
    }

    pub(crate) fn spawn(&self, commands: &mut Commands) {
        commands
            .entity(self.entity)
            .insert_bundle(TransformBundle::default());

        for layer in &self.layers {
            commands.entity(self.entity).add_child(layer.entity);
        }
    }
}
