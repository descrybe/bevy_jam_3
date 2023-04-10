use bevy::prelude::{Component, NextState, ResMut};

use crate::{game::GameSimulationState, AppState};

pub const EXPERIENCE_THRESHOLD: usize = 10000;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub experience: usize,
    pub level: usize,
}

impl Player {
    pub fn get_level(&self) -> usize {
        self.level
    }

    pub fn get_experience_amount(&self) -> usize {
        self.experience
    }

    pub fn give_exp(&mut self, exp: usize) -> bool {
        self.experience += exp;

        return true;
    }

    pub fn lvl_up(
        &mut self,
        mut app_state_next_state: ResMut<NextState<AppState>>,
        mut game_simulation_next_state: ResMut<NextState<GameSimulationState>>,
    ) -> bool {
        self.level += 1;
        self.experience = 0;
        game_simulation_next_state.set(GameSimulationState::Paused);
        app_state_next_state.set(AppState::LvlUp);

        return true;
    }
}
