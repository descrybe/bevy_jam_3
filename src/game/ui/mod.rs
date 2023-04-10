mod constants;
mod dices_preview;
mod experience_ui;
mod lvl_up_dices;
mod pause_menu;
mod player_health_bar;
mod tilemap;

use super::GameSimulationState;
use crate::AppState;
use bevy::prelude::*;
use dices_preview::*;
use experience_ui::*;
use lvl_up_dices::*;
use pause_menu::*;
use player_health_bar::*;
use tilemap::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_simple_map.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(spawn_health_bar.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(spawn_exp_bar.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(spawn_preview_dices.in_schedule(OnExit(AppState::MainMenu)))
            .add_plugin(PauseMenuPlugin)
            .add_plugin(LvlUpPlugin)
            .add_systems(
                (
                    update_health_bar_params,
                    stick_exp_bar,
                    stick_first_dice,
                    stick_second_dice,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameSimulationState::Running)),
            );
    }
}
