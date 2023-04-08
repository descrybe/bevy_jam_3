use bevy::{prelude::Resource, time::Timer};

#[derive(Resource)]
pub struct EnemyWavesSpawnConfig {
    pub timer: Timer,
}
