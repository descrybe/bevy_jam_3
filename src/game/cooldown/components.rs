use bevy::{prelude::Component, time::Timer};

use super::CooldownState;

#[derive(Component)]
pub struct CooldownComponent {
    pub seconds: u64,
    pub state: CooldownState,
    pub timer: Timer
}
