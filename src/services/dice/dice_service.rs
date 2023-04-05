use std::collections::HashMap;

use rand::{rngs::ThreadRng, Rng};

use super::dice_type::EDice;

#[derive(Debug)]
pub struct DiceServiceError();

struct Dice {
    number_of_sides: u16,
    random_generator: ThreadRng,
}

impl Dice {
    fn new(number_of_sides: u16) -> Dice {
        return Dice {
            number_of_sides,
            random_generator: rand::thread_rng(),
        };
    }

    fn roll(&mut self) -> u16 {
        let higher_edge = self.number_of_sides + 1;
        return self.random_generator.gen_range(1..higher_edge);
    }
}

pub trait DiceRoller {
    fn new() -> Self;
    fn roll(&mut self, dice_type: EDice) -> Result<u16, DiceServiceError>;
    fn roll_few_times(
        &mut self,
        dice_type: EDice,
        number_of_rolls: u64,
    ) -> Result<Vec<u16>, DiceServiceError>;
}

pub struct DiceService {
    dice_map: HashMap<EDice, Dice>,
}

impl DiceRoller for DiceService {
    fn new() -> DiceService {
        let mut dice_map = HashMap::<EDice, Dice>::new();
        dice_map.insert(EDice::D6, Dice::new(6));
        return DiceService { dice_map: dice_map };
    }

    fn roll(&mut self, dice_type: EDice) -> Result<u16, DiceServiceError> {
        let dice = self.dice_map.get_mut(&dice_type);
        return dice.map(|dice| dice.roll()).ok_or(DiceServiceError());
    }

    fn roll_few_times(
        &mut self,
        dice_type: EDice,
        number_of_rolls: u64,
    ) -> Result<Vec<u16>, DiceServiceError> {
        let mut result = Vec::new();
        (1..number_of_rolls).for_each(|_| {
            let single_roll_result = self.roll(dice_type).unwrap();
            result.push(single_roll_result);
        });

        return Ok(result);
    }
}
