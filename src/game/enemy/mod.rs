pub mod components;
mod events;
mod resources;
mod systems;

use systems::*;

use crate::AppState;

use bevy::prelude::*;

use self::{events::WaveSpawnEvent, resources::EnemyWavesSpawnConfig};

use super::GameSimulationState;

pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_COUNT: usize = 50;

pub const ENEMY_HEALTH: i32 = 150;
pub const ENEMY_DAMAGE: i32 = 20;

const WAVE_SPAWN_DELAY: f32 = 2.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                enemy_movement,
                wave_timer_tracking_system,
                spawn_enemie_wave,
                kill_enemy,
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        )
        .add_event::<WaveSpawnEvent>()
        .insert_resource(EnemyWavesSpawnConfig {
            timer: Timer::from_seconds(WAVE_SPAWN_DELAY, TimerMode::Repeating),
        });
    }
}
