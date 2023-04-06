use bevy::prelude::*;

use crate::game::GameSimulationState;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<GameSimulationState>>) {
    simulation_state_next_state.set(GameSimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<GameSimulationState>>) {
    simulation_state_next_state.set(GameSimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<GameSimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<GameSimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if simulation_state.0 == GameSimulationState::Running {
            simulation_state_next_state.set(GameSimulationState::Paused);
            println!("Game is Paused.");
        }
        if simulation_state.0 == GameSimulationState::Paused {
            simulation_state_next_state.set(GameSimulationState::Running);
            println!("Game is Running.");
        }
    }
}