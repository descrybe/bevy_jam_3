pub mod bullet;
pub mod damage;
pub mod enemy;
pub mod flight;
pub mod health;
pub mod player;
mod random_position;
pub mod rotator;
pub mod score;
pub mod systems;
pub mod target;
pub mod ui;
pub mod lighting;
pub mod cooldown;

use bevy::prelude::*;

use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use flight::FirePlugin;
use player::PlayerPlugin;
use rotator::RotatorPlugin;
use score::ScorePlugin;
use ui::UIPlugin;
use lighting::LightingPlugin;
use cooldown::CooldownPlugin;
use systems::*;

use crate::{events::GameOver, AppState};

use self::damage::DamagePlugin;
use self::health::HealthPlugin;
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
            .add_plugin(RotatorPlugin)
            .add_plugin(UIPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(DamagePlugin)
            .add_plugin(LightingPlugin)
            .add_plugin(CooldownPlugin)
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
