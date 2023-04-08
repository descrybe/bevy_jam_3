pub mod components;
mod systems;

use systems::*;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct RotatorPlugin;

impl Plugin for RotatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system((rotation_engage)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
