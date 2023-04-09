use bevy::prelude::Component;

#[derive(Component)]
pub struct HealthComponent {
    amount: i32,
    max_health: i32,
}

impl HealthComponent {
    pub fn amount(&self) -> i32 {
        self.amount
    }

    pub fn apply_damage(&mut self, amount: i32) {
        self.amount -= amount;
    }

    pub fn heal(&mut self, amount: i32) {
        let result = self.amount + amount;

        if result > self.max_health {
            self.amount = self.max_health;
        } else {
            self.amount = result;
        }
    }

    pub fn recover(&mut self) {
        self.amount = self.max_health;
    }

    pub fn increase_max_health(&mut self, amount: i32) {
        self.max_health += amount;
        self.heal(amount);
    }

    pub fn new(amount: i32) -> Self {
        HealthComponent {
            amount,
            max_health: amount,
        }
    }
}
