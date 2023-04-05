use crate::services::dice::{DiceRoller, DiceService};

use super::PositionGenerator;

pub struct EnemiesSpawnPositionService {
    dice_service: DiceService,
}

impl PositionGenerator<u32> for EnemiesSpawnPositionService {
    fn new() -> Self {
        return EnemiesSpawnPositionService {
            dice_service: DiceService::new(),
        };
    }

    fn generate(&self) -> super::Point<u32> {
        todo!()
    }
}
