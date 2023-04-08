use std::time::Duration;

use bevy::{prelude::{Query, Res, EventReader}, time::{Time, TimerMode}};

use super::{components::CooldownComponent, events::CooldownEvent, CooldownState};

pub fn cooldown_update(
    mut cooldown_query: Query<&mut CooldownComponent>,
    time: Res<Time>,
) {
    for mut cooldown_component in cooldown_query.iter_mut() {
        cooldown_component.timer.tick(time.delta());
        if cooldown_component.timer.finished()
        {
            println!("COOLDOWN IS FINISHED");
            cooldown_component.state = CooldownState::READY;
        }
    }
}

pub fn cooldown_event_holder(
    mut cooldown_query: Query<&mut CooldownComponent>,
    mut cooldown_event_reader: EventReader<CooldownEvent>,
) {
    if cooldown_event_reader.is_empty() {
        return;
    }

    for event in cooldown_event_reader.iter() {
        if let Ok(mut cooldown_component) = cooldown_query.get_mut(event.entity) {
            match cooldown_component.state {
                CooldownState::READY => {
                    let cooldown_seconds = cooldown_component.seconds;
                    println!("RESET COOLDOWN");
                    cooldown_component.state = CooldownState::PAUSED;
                    cooldown_component.timer.set_duration(Duration::from_secs(cooldown_seconds));
                    cooldown_component.timer.set_mode(TimerMode::Repeating);
                    cooldown_component.timer.reset();
                },
                CooldownState::PAUSED => {
                    // Nothing
                },
            }
        }
    }
}



