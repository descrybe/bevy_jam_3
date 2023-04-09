pub mod ability;
pub mod bullet;
pub mod collision;
pub mod damage;
pub mod enemy;
pub mod flight;
pub mod health;
pub mod lighting;
pub mod movement;
pub mod player;
pub mod player_binder;
pub mod radiance;
mod random_position;
pub mod rotator;
pub mod sattlite;
pub mod score;
pub mod shuriken;
pub mod systems;
pub mod target;
pub mod ui;

use bevy::prelude::*;

use bullet::BulletPlugin;
use enemy::EnemyPlugin;
use flight::FirePlugin;
use lighting::LightingPlugin;
use player::PlayerPlugin;
use player_binder::PlayerBinderPlugin;
use radiance::RadiancePlugin;
use rotator::RotatorPlugin;
use sattlite::SattelitePlugin;
use score::ScorePlugin;
use shuriken::ShurikenPlugin;
use systems::*;
use ui::UIPlugin;

use crate::{events::GameOver, AppState};

use self::ability::AbilityPlugin;
use self::collision::CollisionPlugin;
use self::damage::DamagePlugin;
use self::health::HealthPlugin;
use self::movement::MovementPlugin;
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
            .add_plugin(CollisionPlugin)
            .add_plugin(AbilityPlugin)
            .add_plugin(HealthPlugin)
            .add_plugin(DamagePlugin)
            .add_plugin(LightingPlugin)
            .add_plugin(ShurikenPlugin)
            .add_plugin(SattelitePlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(RadiancePlugin)
            .add_plugin(PlayerBinderPlugin)
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
