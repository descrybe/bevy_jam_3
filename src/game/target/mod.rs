use bevy::prelude::Plugin;

use self::{
    events::TargetLostEvent,
    systems::{target_actualization_system, target_tracking_system},
};

pub mod components;
pub mod events;
mod systems;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<TargetLostEvent>()
            .add_system(target_actualization_system)
            .add_system(target_tracking_system);
    }
}
