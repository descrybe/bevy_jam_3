use bevy::prelude::*;

use super::{components::HealthComponent, events::DeathEvent, DEATH_EDGE};

pub fn health_check_system(
    health_query: Query<(Entity, &HealthComponent), With<HealthComponent>>,
    mut event: EventWriter<DeathEvent>,
) {
    for (entity, health) in health_query.iter() {
        if health.amount() > DEATH_EDGE {
            continue;
        }

        event.send(DeathEvent { entity });
    }
}
