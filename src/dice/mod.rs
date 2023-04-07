pub mod resources;

use bevy::prelude::Plugin;

use self::resources::{DiceRoller, DiceService};

pub struct DicePlugin;

impl Plugin for DicePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(DiceService::new());
    }
}
