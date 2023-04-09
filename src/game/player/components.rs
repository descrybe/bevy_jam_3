use bevy::prelude::Component;

pub const EXPERIENCE_THRESHOLD: usize = 10000;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub experience: usize,
    pub level: usize,
}

impl Player {
    pub fn get_level(&self) -> usize {
        self.level
    }

    pub fn get_experience_amount(&self) -> usize {
        self.experience
    }

    pub fn give_exp(&mut self, exp: usize) -> bool {
        self.experience += exp;

        if self.experience >= EXPERIENCE_THRESHOLD {
            self.level += 1;
            return true;
        }
        return true;
    }
}