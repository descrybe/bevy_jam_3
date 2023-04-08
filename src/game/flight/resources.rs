use bevy::{prelude::Resource, time::Timer};

#[derive(Resource)]

pub struct FireSpawnConfig {
    pub timer: Timer,
}
