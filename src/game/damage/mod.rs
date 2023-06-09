pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::{IntoSystemConfig, IntoSystemConfigs, Plugin};

use self::{events::DamageEvent, systems::*};

use super::{
    bullet::components::Bullet, enemy::components::Enemy, player::components::Player,
    radiance::components::Radiance, shuriken::components::Shuriken, lighting::components::Lighting,
};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<DamageEvent>()
            .add_system(damage_income_system)
            .add_system(target_lost_event_system)
            .add_system(update_timers.before(damage_dealler_destruct_system))
            .add_system(damage_dealler_destruct_system.after(damage_income_system))
            .add_system(self_destructing_despawn_system)
            .add_systems(
                (
                    collision_damage_system::<Bullet, Enemy>,
                    collision_damage_system::<Radiance, Enemy>,
                    collision_damage_system::<Shuriken, Enemy>,
                    collision_damage_system::<Lighting, Enemy>,
                    collision_damage_system::<Enemy, Player>,
                )
                    .after(damage_dealler_destruct_system),
            );
    }
}
