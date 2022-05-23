mod movement;
mod unit;

use bevy::prelude::PluginGroup;
use movement::MovementPlugin;
pub use movement::ValidMove;
use unit::{Direction, UnitPlugin};
pub use unit::{SelectMode, SelectedUnit, Unit};

pub struct UnitPluginGroup;

impl PluginGroup for UnitPluginGroup {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(UnitPlugin).add(MovementPlugin);
    }
}
