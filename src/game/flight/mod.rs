pub mod components;
pub mod resources;
mod systems;

use systems::*;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_fire_spawning).add_system(
            (flight_engage)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
