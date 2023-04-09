use bevy::{
    prelude::{Commands, Component, Entity, EventReader, EventWriter, Query, Res, With},
    time::Time,
};

use crate::game::{
    collision::events::CollisionEvent, health::components::HealthComponent,
    target::events::TargetLostEvent,
};

use super::{
    components::{DamageDealerComponent, SelfDestructable},
    events::DamageEvent,
};

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

pub fn damage_dealler_destruct_system(
    mut event_reader: EventReader<DamageEvent>,
    mut query: Query<&mut SelfDestructable>,
) {
    if event_reader.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        let entity = event.dealer;
        if !query.contains(entity) {
            continue;
        }

        let mut dealer = query.get_mut(entity).unwrap();
        dealer.start_countdown();
    }
}

pub fn update_timers(mut query: Query<&mut SelfDestructable>, time: Res<Time>) {
    for mut item in query.iter_mut() {
        item.update_timer(time.delta());
    }
}

pub fn self_destructing_despawn_system(
    mut commands: Commands,
    query: Query<(Entity, &SelfDestructable)>,
) {
    if query.is_empty() {
        return;
    }

    let entities_to_despawn = query
        .iter()
        .filter(|(_, options)| options.is_ready_to_die())
        .map(|(entity, _)| entity);

    for entity in entities_to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn target_lost_event_system(
    mut event_reader: EventReader<TargetLostEvent>,
    mut query: Query<&mut SelfDestructable>,
) {
    if event_reader.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        if !query.contains(event.sender) {
            continue;
        }

        let mut destructable_component = query.get_mut(event.sender).unwrap();
        destructable_component.start_countdown();
    }
}
