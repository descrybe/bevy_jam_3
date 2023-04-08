use bevy::{
    prelude::{Commands, Query, Res, Transform, Vec3},
    time::{Time, Timer, TimerMode},
};
use std::time::Duration;

use super::components::Flight;
use crate::game::{flight::resources::FireSpawnConfig, target::components::DirectionHolderComponent};

pub fn flight_engage(mut flight_query: Query<(&mut Transform, &Flight, &DirectionHolderComponent)>, time: Res<Time>) {
    for (mut transform, flight, direction_holder) in flight_query.iter_mut() {
        let direction = Vec3::new(direction_holder.direction.x, direction_holder.direction.y, 0.0);
        transform.translation += direction * flight.speed * time.delta_seconds();
    }
}

pub fn setup_fire_spawning(mut commands: Commands) {
    commands.insert_resource(FireSpawnConfig {
        timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
    })
}
