use std::ops::Sub;

use bevy::prelude::{Commands, Entity, EventWriter, Query, Transform, Vec2, With};

use super::{
    components::{DirectionHolderComponent, TargetHolderComponent},
    events::TargetLostEvent,
};

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

pub fn target_actualization_system(
    mut event_writer: EventWriter<TargetLostEvent>,
    target_holder_query: Query<(&TargetHolderComponent, Entity)>,
    entity_query: Query<Entity>,
) {
    if target_holder_query.is_empty() || entity_query.is_empty() {
        return;
    }

    for (target_holder, holder_entity) in target_holder_query.iter() {
        if entity_query.contains(target_holder.target_entity) {
            continue;
        }

        event_writer.send(TargetLostEvent {
            sender: holder_entity,
        });
    }
}
