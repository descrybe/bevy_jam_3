mod systems;

use bevy::prelude::Plugin;

use self::systems::adjust_movement;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(adjust_movement);
    }
}
