pub mod systems;
pub mod components;
pub mod constants;

use bevy::prelude::*;

use systems::*;
use crate::AppState;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuState>()
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(play_button_interaction)
            .add_system(exit_button_interaction)
            .add_system(toggle_game_status.run_if(in_state(AppState::MainMenu)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
}
