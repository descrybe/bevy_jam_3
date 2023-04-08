use bevy::prelude::*;

use crate::game::GameSimulationState;

pub fn set_game_paused(mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>) {
    game_simulation_next_state.set(GameSimulationState::Paused);
}

pub fn set_game_running(mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>) {
    game_simulation_next_state.set(GameSimulationState::Running);
}

pub fn toggle_game_running_state(
    mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>,
    simulation_state: Res<State<GameSimulationState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let concrete_simultation_state = simulation_state.0;

        match concrete_simultation_state {
            GameSimulationState::Running => game_simulation_next_state.set(GameSimulationState::Running),
            GameSimulationState::Paused => game_simulation_next_state.set(GameSimulationState::Paused)
        }
    }
}