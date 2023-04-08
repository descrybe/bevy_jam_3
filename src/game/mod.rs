pub mod enemy;
pub mod player;
mod random_position;
pub mod score;
pub mod systems;
pub mod target;
pub mod bullet;
pub mod flight;
pub mod rotator;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use systems::*;
use bullet::BulletPlugin;
use flight::FirePlugin;
use rotator::RotatorPlugin;

use crate::events::GameOver;
use crate::AppState;

use self::random_position::RandomPositionPlugin;
use self::target::TargetPlagin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_state::<GameSimulationState>()
            .add_system(pause_game.in_schedule(OnEnter(AppState::MainMenu)))
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(RandomPositionPlugin)
            .add_plugin(TargetPlagin)
            .add_plugin(BulletPlugin)
            .add_plugin(FirePlugin)
            .add_plugin(RotatorPlugin)
            .add_system(toggle_game_active_state.run_if(in_state(AppState::Game)))
            .add_system(resume_game.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameSimulationState {
    Running,
    #[default]
    Paused,
}
