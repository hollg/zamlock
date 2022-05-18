use bevy::prelude::*;

use super::layer::Layer;

#[derive(Component)]
pub struct Map {
    pub(crate) entity: Entity,
    pub(crate) layers: Vec<Layer>,
    pub(crate) tile_size: f32,
}

impl Map {
    pub(crate) fn new(entity: Entity, tile_size: f32) -> Map {
        Map {
            entity,
            layers: vec![],
            tile_size,
        }
    }

    pub(crate) fn insert_layer(&mut self, commands: &mut Commands, layer: Layer) {
        commands.entity(self.entity).add_child(layer.entity);
        self.layers.push(layer);
    }

    pub(crate) fn insert_layers(&mut self, commands: &mut Commands, layers: &[Layer]) {
        for layer in layers {
            self.insert_layer(commands, layer.clone());
        }
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
