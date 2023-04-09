use bevy::prelude::Entity;

pub struct LaunchRadianceEvent {
    pub owner: Entity,
    pub target: Entity,
}
