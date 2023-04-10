pub mod components;
pub mod events;
pub mod resources;
mod systems;

use events::*;
use systems::*;

use super::GameSimulationState;
use crate::{systems::camera_follow, AppState};

use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_HEALTH: i32 = 20000;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChooseModificationEvent>()
            .add_system(spawn_player.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems(
                (
                    camera_follow,
                    player_movement,
                    change_player_direction,
                    player_health_check_system,
                    player_chose_modification,
                    animate_player,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameSimulationState::Running)),
            );
    }
}
