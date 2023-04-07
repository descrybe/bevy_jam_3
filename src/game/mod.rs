pub mod enemy;
pub mod player;
pub mod score;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::*;

use bevy::prelude::{Plugin, App, States};

use crate::events::GameOver;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOver>()
            .add_state::<GameSimulationState>()
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            // .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameSimulationState {
    Running,
    #[default]
    Paused,
}
