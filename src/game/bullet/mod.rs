pub mod components;
mod systems;

use systems::*;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

pub const BULLET_DAMAGE: i32 = 40;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            (spawn_bullet)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
