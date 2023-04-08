use bevy::prelude::*;

use crate::game::damage::events::DamageEvent;

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

pub fn damage_income_system(
    mut health_queary: Query<&mut HealthComponent, With<HealthComponent>>,
    mut damage_event_reader: EventReader<DamageEvent>,
) {
    if damage_event_reader.is_empty() {
        return;
    }

    for event in damage_event_reader.iter() {
        let mut target = health_queary.get_mut(event.target).unwrap();
        target.apply_damage(event.damage_amount);
    }
}
