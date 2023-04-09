pub mod components;
mod systems;

use self::systems::spawn_lightning_bolts;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            (spawn_lightning_bolts)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
