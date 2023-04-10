mod systems;
mod components;

use bevy::prelude::*;
use systems::*;

// TODO: move module to another folder

use crate::AppState;

pub struct LvlUpPlugin;

impl Plugin for LvlUpPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_lvlup_dices.in_schedule(OnEnter(AppState::LvlUp)))
            .add_system((lvlup_dice_interaction).in_set(OnUpdate(AppState::LvlUp)))
            .add_system(despawn_lvlup_dices.in_schedule(OnExit(AppState::LvlUp)));
    }
}
