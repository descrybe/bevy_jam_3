pub mod components;
pub mod events;
pub mod systems;

use bevy::prelude::{IntoSystemConfigs, Plugin};

use self::{events::CollisionEvent, systems::*};

use super::{bullet::components::Bullet, enemy::components::Enemy, player::components::Player, shuriken::components::Shuriken};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CollisionEvent>()
            .add_system(clear_collision)
            .add_systems(
                (
                    collision_detection_system::<Bullet, Enemy>,
                    collision_detection_system::<Shuriken, Enemy>,
                    collision_detection_system::<Player, Enemy>,
                    internal_collision_detection_system::<Enemy>,
                )
                    .after(clear_collision),
            );
    }
}
