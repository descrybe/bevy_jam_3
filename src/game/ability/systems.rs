use bevy::{
    prelude::{Component, Entity, EventReader, EventWriter, Query, Res, With},
    time::Time,
};

use super::{
    components::{AbilityComponent, Cooldown, ECooldownState, PeriodicAbility},
    events::{CooldownTriggerEvent, TriggerAbilityEvent},
};

pub fn update_timers(mut query: Query<&mut Cooldown>, time: Res<Time>) {
    if query.is_empty() {
        return;
    }

    for mut cooldown in query.iter_mut() {
        cooldown.update_cooldown(time.delta());
    }
}

pub fn trigger_cooldown(
    mut event_reader: EventReader<CooldownTriggerEvent>,
    mut query: Query<&mut Cooldown>,
) {
    if event_reader.is_empty() {
        return;
    }

    for event in event_reader.iter() {
        if !query.contains(event.target) {
            continue;
        }

        let mut cooldown = query.get_mut(event.target).unwrap();
        cooldown.trigger_cooldown();
    }
}

pub fn trigger_periodical_ability(
    ability_query: Query<(Entity, &Cooldown), (With<PeriodicAbility>, With<AbilityComponent>)>,
    mut ability_event_writer: EventWriter<TriggerAbilityEvent>,
    mut colldown_event_writer: EventWriter<CooldownTriggerEvent>,
) {
    for (entity, cooldown) in ability_query.iter() {
        if cooldown.state() == ECooldownState::Disabled {
            continue;
        }

        ability_event_writer.send(TriggerAbilityEvent { owner: entity });
        colldown_event_writer.send(CooldownTriggerEvent { target: entity })
    }
}
