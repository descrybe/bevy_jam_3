use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthComponent {
    amount: u32,
}

impl HealthComponent {
    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn apply_damage(&mut self, amount: u32) {
        self.amount -= amount;
    }

    pub fn new(amount: u32) -> Self {
        HealthComponent { amount }
    }
}
