use bevy::prelude::*;

use super::game::systems::*;
use crate::AppState;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(main_menu)
            .add_system(toggle_game_active_state.run_if(in_state(AppState::Game)));
    }
}

pub fn main_menu() {
    println!("You are on the main menu.");
}
