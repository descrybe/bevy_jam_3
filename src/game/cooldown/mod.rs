use bevy::prelude::{App, IntoSystemConfigs, OnUpdate, Plugin};

pub mod components;
pub mod events;
mod systems;

use crate::AppState;

use self::systems::{cooldown_event_holder, cooldown_update};
use super::{cooldown::events::CooldownEvent, GameSimulationState};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CooldownState {
    #[default]
    READY,
    PAUSED,
}

pub struct CooldownPlugin;

impl Plugin for CooldownPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CooldownEvent>().add_systems(
            (cooldown_update, cooldown_event_holder)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameSimulationState::Running)),
        );
    }
}
