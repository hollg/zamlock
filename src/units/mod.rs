mod movement;
mod unit;

use bevy::prelude::PluginGroup;
use movement::MovementPlugin;
use unit::Unit;
use unit::UnitPlugin;

pub struct UnitPluginGroup;

impl PluginGroup for UnitPluginGroup {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(UnitPlugin).add(MovementPlugin);
    }
}
