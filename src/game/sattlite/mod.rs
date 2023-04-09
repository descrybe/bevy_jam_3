pub mod components;
mod systems;

use systems::*;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct SattelitePlugin;

impl Plugin for SattelitePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            (update_sattelite)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
