mod systems;
mod components;

use bevy::prelude::*;
use systems::*;

use crate::game::GameSimulationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pause_menu.in_schedule(OnEnter(GameSimulationState::Paused)));
    }
}
