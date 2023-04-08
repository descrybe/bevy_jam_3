use bevy::prelude::Plugin;

use self::systems::target_tracking_system;

pub mod components;
mod systems;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(target_tracking_system);
    }
}
