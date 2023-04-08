use std::ops::Sub;

use bevy::prelude::{Commands, Entity, Query, Transform, Vec2, With};

use super::components::{DirectionHolderComponent, TargetHolderComponent};

pub fn target_tracking_system(
    mut commands: Commands,
    mut target_holder_query: Query<
        (
            &TargetHolderComponent,
            &mut DirectionHolderComponent,
            &Transform,
            Entity,
        ),
        With<TargetHolderComponent>,
    >,
    target_query: Query<&Transform>,
) {
    for (target_holder, mut direction_holder, transform, entity) in target_holder_query.iter_mut() {
        if !target_query.contains(target_holder.target_entity) {
            commands.entity(entity).remove::<TargetHolderComponent>();
            continue;
        }
        let target_transform = target_query.get(target_holder.target_entity).unwrap();
        let new_direction = target_transform.translation.sub(transform.translation);
        let new_coordinates = [new_direction.x, new_direction.y];
        direction_holder.direction = Vec2::from_array(new_coordinates).normalize();
    }
}
