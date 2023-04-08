pub mod components;
mod systems;

use systems::*;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

pub const BULLET_DAMAGE: i32 = 20;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (spawn_bullet, bullet_hit_enemy)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
