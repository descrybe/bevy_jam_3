pub mod components;
pub mod events;
mod systems;

use bevy::prelude::Plugin;

use self::{
    events::DeathEvent,
    systems::{damage_income_system, health_check_system},
};

pub const DEATH_EDGE: u32 = 0;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DeathEvent>()
            .add_system(health_check_system)
            .add_system(damage_income_system);
    }
}
