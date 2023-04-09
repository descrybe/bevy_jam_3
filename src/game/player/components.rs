use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub exp: usize,
    pub lvl: usize,
}

impl Player {
    pub fn give_exp(&mut self, exp: usize) -> bool {
        self.exp += exp;
        if self.exp >= 100 {
            self.lvl += 1;
            return true;
        }
        return true;
    }
}