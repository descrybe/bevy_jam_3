pub mod experience_ui;
pub mod player_health_bar;
pub mod constants;

use crate::AppState;
use bevy::prelude::*;
use experience_ui::*;
use player_health_bar::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_health_bar.in_schedule(OnEnter(AppState::Game)))
            .add_system(spawn_exp_bar.in_schedule(OnEnter(AppState::Game)));
    }
}
