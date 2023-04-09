use std::time::Duration;

use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct DamageDealerComponent {
    pub damage: i32,
}

#[derive(Component)]
pub struct SelfDestructable {
    timer: Timer,
}

impl SelfDestructable {
    pub fn new(delay_in_sec: f32) -> Self {
        let mut timer = Timer::from_seconds(delay_in_sec, bevy::time::TimerMode::Once);
        timer.pause();
        SelfDestructable { timer }
    }

    pub fn start_countdown(&mut self) {
        self.timer.unpause();
    }

    pub fn update_timer(&mut self, delta: Duration) {
        self.timer.tick(delta);
    }

    pub fn is_ready_to_die(&self) -> bool {
        return self.timer.finished();
    }
}
