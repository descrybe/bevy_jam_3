pub mod components;
pub mod dice;
pub mod events;
pub mod game;
mod main_menu;
mod systems;

use dice::DicePlugin;
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
        .add_plugin(DicePlugin)
        .add_event::<GameOver>()
        .add_system(set_game_active)
        .add_system(set_main_menu_active)
        .add_system(game_over_hander)
        .add_system(exit_game)
        .add_startup_system(spawn_camera)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
