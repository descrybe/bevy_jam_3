use bevy::prelude::*;

#[derive(Resource)]
pub struct Health {
    pub amount: u8,
}

impl Default for Health {
    fn default() -> Health {
        Health { amount: 100 }
    }
}

#[derive(Resource)]
pub struct Experience {
    pub amount: u8,
}

impl Default for Experience {
    fn default() -> Experience {
        Experience { amount: 0 }
    }
}

#[derive(Resource)]
pub struct Level {
    pub value: u8,
}

impl Default for Level {
    fn default() -> Level {
        Level { value: 1 }
    }
}
