pub mod components;
mod events;
mod systems;

use systems::*;

use self::events::LaunchBulletEvent;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

const BULLET_SIZE: f32 = 40.0;
const ROTATION_SPEED: f32 = 720.0;
const BULLET_SPEED: f32 = 280.0;
const BULLET_DAMAGE: i32 = 40;
const COOLDOWN_DELAY: f32 = 0.5;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate_ability_entity.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (spawn_bullet, trigger_bullet_ability)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameSimulationState::Running)),
            )
            .add_event::<LaunchBulletEvent>();
    }
}
