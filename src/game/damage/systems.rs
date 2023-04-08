use bevy::prelude::{Query, With, EventReader};

use crate::game::health::components::HealthComponent;

use super::{events::DamageEvent, components::DamageDealerComponent};

pub fn damage_income_system(
    mut health_queary: Query<&mut HealthComponent, With<HealthComponent>>,
    dealer_queary: Query<&DamageDealerComponent, With<DamageDealerComponent>>,
    mut damage_event_reader: EventReader<DamageEvent>,
) {
    if damage_event_reader.is_empty() {
        return;
    }

    for event in damage_event_reader.iter() {
        println!("IN DAMAGE INCOME");
        if  !health_queary.contains(event.target)
            || !dealer_queary.contains(event.dealer) {
            continue;
        }
        let dealer = dealer_queary.get(event.dealer).unwrap();
        let mut target = health_queary.get_mut(event.target).unwrap();
        target.apply_damage(dealer.damage);
    }
}
