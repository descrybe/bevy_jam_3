use bevy::prelude::Component;

use crate::game::player::events::Modification;

#[derive(Component)]
pub struct DiceButton {
    pub value: Modification,
}

#[derive(Component)]
pub struct LvlUpText {}
