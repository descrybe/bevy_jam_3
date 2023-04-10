mod systems;
mod components;

use bevy::prelude::*;
use systems::*;

use crate::AppState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pause_menu.in_schedule(OnEnter(AppState::PauseMenu)))
            .add_systems(
                (
                    resume_button_interaction,
                    exit_button_interaction,
                )
                    .in_set(OnUpdate(AppState::PauseMenu)),
            )
            .add_system(set_pause_menu_state)
            .add_system(toggle_game_status.run_if(in_state(AppState::PauseMenu)))
            .add_system(despawn_pause_menu.in_schedule(OnExit(AppState::PauseMenu)));
    }
}
