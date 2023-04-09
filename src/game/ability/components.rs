use std::time::Duration;

use bevy::{
    prelude::{Component, Entity},
    time::Timer,
};

#[derive(Component)]
pub struct AbilityComponent {
    pub owner: Entity,
}

#[derive(Component)]
pub struct PeriodicAbility {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ECooldownState {
    Active,
    Disabled,
}

#[derive(Component)]
pub struct Cooldown {
    duration: f32,
    timer: Timer,
    state: ECooldownState,
}

impl Cooldown {
    pub fn new(cooldown_seconds: f32) -> Self {
        let mut timer = Timer::from_seconds(cooldown_seconds, bevy::time::TimerMode::Once);
        timer.pause();
        Cooldown {
            duration: cooldown_seconds,
            timer,
            state: ECooldownState::Active,
        }
    }

    pub fn trigger_cooldown(&mut self) {
        self.state = ECooldownState::Disabled;
        self.timer.unpause();
    }

    pub fn update_cooldown(&mut self, delta: Duration) {
        self.timer.tick(delta);
        if self.timer.finished() {
            self.state = ECooldownState::Active;
            let duration = Duration::from_secs_f32(self.duration);
            self.timer.set_duration(duration);
            self.timer.reset();
            self.timer.pause();
        }
    }

    pub fn change_duration(&mut self, duration_seconds: f32) {
        self.duration = duration_seconds;
    }

    pub fn state(&self) -> ECooldownState {
        self.state
    }
}
