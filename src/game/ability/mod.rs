pub mod components;
pub mod events;
mod systems;

use bevy::prelude::{IntoSystemConfig, Plugin};
use systems::*;

use self::events::{CooldownTriggerEvent, TriggerAbilityEvent};

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<TriggerAbilityEvent>()
            .add_system(update_timers)
            .add_system(trigger_periodical_ability)
            .add_system(trigger_cooldown.after(update_timers))
            .add_event::<CooldownTriggerEvent>();
    }
}
