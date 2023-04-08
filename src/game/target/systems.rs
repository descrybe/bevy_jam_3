use std::ops::Sub;

use bevy::prelude::{Query, Transform, Vec2, With};

use super::components::{DirectionHolderComponent, TargetHolderComponent};

pub fn target_tracking_system(
    mut target_holder_query: Query<
        (
            &TargetHolderComponent,
            &mut DirectionHolderComponent,
            &Transform,
        ),
        With<TargetHolderComponent>,
    >,
    target_query: Query<&Transform>,
) {
    for (target_holder, mut direction_holder, transform) in target_holder_query.iter_mut() {
        let target_transform = target_query.get(target_holder.target_entity).unwrap();
        let new_direction = target_transform.translation.sub(transform.translation);
        let new_coordinates = [new_direction.x, new_direction.y];
        direction_holder.direction = Vec2::from_array(new_coordinates).normalize();
    }
}
