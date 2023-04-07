use bevy::prelude::*;

use crate::game::GameSimulationState;

pub fn pause_game(mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>) {
    game_simulation_next_state.set(GameSimulationState::Paused);
}

pub fn resume_game(mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>) {
    game_simulation_next_state.set(GameSimulationState::Running);
}

pub fn toggle_game_active_state(
    mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>,
    simulation_state: Res<State<GameSimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if simulation_state.0 == GameSimulationState::Running {
            game_simulation_next_state.set(GameSimulationState::Paused);
            println!("Game is Paused.");
        }
        if simulation_state.0 == GameSimulationState::Paused {
            game_simulation_next_state.set(GameSimulationState::Running);
            println!("Game is Running.");
        }
    }
}
