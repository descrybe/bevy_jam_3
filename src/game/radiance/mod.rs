pub mod components;
mod events;
mod systems;

use systems::*;

use self::events::LaunchRadianceEvent;

use super::GameSimulationState;
use crate::AppState;

use bevy::prelude::*;

const RADIANCE_SIZE: f32 = 100.0;
const RADIANCE_DAMAGE: i32 = 80;
const COOLDOWN_DELAY: f32 = 0.2;

pub struct RadiancePlugin;

impl Plugin for RadiancePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate_ability_entity.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (spawn_radiance, trigger_bullet_ability)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameSimulationState::Running)),
            )
            .add_event::<LaunchRadianceEvent>();
    }
}
