use bevy::prelude::{Component, Entity, EventReader, EventWriter, Query, With};

use crate::game::{collision::events::CollisionEvent, health::components::HealthComponent};

use super::{components::DamageDealerComponent, events::DamageEvent};

pub fn damage_income_system(
    mut health_queary: Query<&mut HealthComponent, With<HealthComponent>>,
    dealer_queary: Query<&DamageDealerComponent, With<DamageDealerComponent>>,
    mut damage_event_reader: EventReader<DamageEvent>,
) {
    if damage_event_reader.is_empty() {
        return;
    }

    for event in damage_event_reader.iter() {
        if !health_queary.contains(event.target) || !dealer_queary.contains(event.dealer) {
            continue;
        }
        let dealer = dealer_queary.get(event.dealer).unwrap();
        let mut target = health_queary.get_mut(event.target).unwrap();
        target.apply_damage(dealer.damage);
    }
}

pub fn collision_damage_system<TAttacker: Component, TVictim: Component>(
    mut damage_event_writer: EventWriter<DamageEvent>,
    mut collision_event_reader: EventReader<CollisionEvent>,
    damage_dealer_query: Query<&DamageDealerComponent, With<TAttacker>>,
    health_query: Query<Entity, (With<HealthComponent>, With<TVictim>)>,
) {
    if collision_event_reader.is_empty() {
        return;
    }

    if damage_dealer_query.is_empty() || health_query.is_empty() {
        return;
    }

    for event in collision_event_reader.iter() {
        let combinations = [
            (*event.first(), *event.second()),
            (*event.second(), *event.first()),
        ];

        for (atacker, victim) in combinations {
            if !(damage_dealer_query.contains(atacker) && health_query.contains(victim)) {
                continue;
            }

            damage_event_writer.send(DamageEvent {
                dealer: atacker,
                target: victim,
            });
            return;
        }
    }
}
