pub mod components;
pub mod events;

use bevy::prelude::Plugin;

use self::events::DamageEvent;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DamageEvent>();
    }
}
