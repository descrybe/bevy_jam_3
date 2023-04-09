pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::{IntoSystemConfigs, Plugin};

use self::{
    events::DamageEvent,
    systems::{collision_damage_system, damage_income_system},
};

use super::{bullet::components::Bullet, enemy::components::Enemy, player::components::Player};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DamageEvent>()
            .add_system(damage_income_system)
            .add_systems(
                (
                    collision_damage_system::<Bullet, Enemy>,
                    collision_damage_system::<Enemy, Player>,
                )
                    .after(damage_income_system),
            );
    }
}
