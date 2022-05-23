mod movement;
mod unit;

use bevy::prelude::PluginGroup;
use movement::MovementPlugin;
use unit::UnitPlugin;
pub use unit::{SelectedUnit, Unit, SelectMode};
pub use movement::ValidMove;

pub struct UnitPluginGroup;

impl PluginGroup for UnitPluginGroup {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(UnitPlugin).add(MovementPlugin);
    }
}
