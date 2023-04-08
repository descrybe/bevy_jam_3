use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthComponent {
    amount: i32,
}

impl HealthComponent {
    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn apply_damage(&mut self, amount: i32) {
        self.amount -= amount;
    }

    pub fn new(amount: i32) -> Self {
        HealthComponent { amount }
    }
}
