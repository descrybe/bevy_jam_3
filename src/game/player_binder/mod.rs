pub mod components;
mod systems;

use systems::*;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct PlayerBinderPlugin;

impl Plugin for PlayerBinderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            (update_bind)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
