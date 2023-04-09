use bevy::prelude::Entity;

pub struct CooldownTriggerEvent {
    pub target: Entity,
}

pub struct TriggerAbilityEvent {
    pub owner: Entity,
}

impl TriggerAbilityEvent {
    pub fn new(owner: Entity) -> Self {
        TriggerAbilityEvent { owner }
    }

    pub fn owner(&self) -> Entity {
        self.owner
    }
}
