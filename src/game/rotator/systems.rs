use bevy::{
    prelude::{Query, Transform, Quat, With, Res}, time::Time,
};

use super::components::Rotator;

pub fn rotation_engage(
    mut rotator_query: Query<(&mut Transform, &Rotator), With<Rotator>>,
    time: Res<Time>,
) {
    for (mut rotator_transform, rotator) in rotator_query.iter_mut() {
        rotator_transform.rotate(Quat::from_rotation_z((rotator.angle).to_radians() * time.delta_seconds()));
    }
}
