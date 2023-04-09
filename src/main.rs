pub mod assets_cache;
pub mod components;
pub mod dice;
pub mod events;
pub mod game;
mod main_menu;
mod systems;

use assets_cache::AssetsCachePlugin;
use dice::DicePlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

use events::*;
use systems::*;

use bevy::{prelude::*, window::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1366., 768.),
                title: "Side Effect".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(DicePlugin)
        .add_plugin(AssetsCachePlugin)
        .add_startup_system(spawn_camera)
        .add_event::<GameOver>()
        .add_system(set_game_active)
        .add_system(set_main_menu_active)
        .add_system(game_over_hander)
        .add_system(exit_game)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
