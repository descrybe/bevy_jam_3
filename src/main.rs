pub mod components;
pub mod events;
pub mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use systems::*;
use events::*;

use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .add_event::<GameOver>()
        .add_system(camera_follow)
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
