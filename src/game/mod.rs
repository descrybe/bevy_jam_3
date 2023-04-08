pub mod enemy;
pub mod player;
mod random_position;
pub mod score;
pub mod systems;
pub mod target;
pub mod bullet;
pub mod flight;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::*;
use bullet::BulletPlugin;
use flight::FirePlugin;

use crate::{events::GameOver, AppState};

use self::random_position::RandomPositionPlugin;
use self::target::TargetPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_state::<GameSimulationState>()
            .add_system(set_game_paused.in_schedule(OnExit(AppState::Game)))
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(RandomPositionPlugin)
            .add_plugin(TargetPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(FirePlugin)
            .add_system(set_game_running.in_schedule(OnEnter(AppState::Game)))
            .add_system(toggle_game_running_state.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameSimulationState {
    #[default]
    Running,
    Paused,
}
