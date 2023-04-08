pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::Plugin;

use self::{events::DamageEvent, systems::damage_income_system};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DamageEvent>()
        .add_system(damage_income_system);
    }
}
