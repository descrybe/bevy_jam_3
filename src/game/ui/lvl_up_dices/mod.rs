mod systems;
mod components;

use bevy::prelude::*;
use systems::*;
use components::*;

// TODO: move module to another folder

use crate::{main_menu::systems::toggle_game_status, AppState};

pub struct LvlUpPlugin;

impl Plugin for LvlUpPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_lvlup_dices.in_schedule(OnEnter(AppState::LvlUp)))
            .add_system((lvlup_dice_interaction).in_set(OnUpdate(AppState::LvlUp)))
            .add_system(toggle_game_status.run_if(in_state(AppState::LvlUp)))
            .add_system(despawn_lvlup_dices.in_schedule(OnExit(AppState::LvlUp)));
    }
}
