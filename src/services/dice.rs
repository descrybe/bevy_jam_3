use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EDice {
    D4 = 4,
    D6 = 6,
    D100 = 100,
}

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
    fn roll(&mut self, dice_type: EDice) -> Option<u16>;
    fn roll_few_times(&mut self, dice_type: EDice, number_of_rolls: u64) -> Option<Vec<u16>>;
}

pub struct DiceService {
    dice_map: HashMap<EDice, Dice>,
}

impl DiceRoller for DiceService {
    fn new() -> DiceService {
        let mut dice_map = HashMap::<EDice, Dice>::new();
        dice_map.insert(EDice::D4, Dice::new(4));
        dice_map.insert(EDice::D6, Dice::new(6));
        dice_map.insert(EDice::D100, Dice::new(100));
        return DiceService { dice_map: dice_map };
    }

    fn roll(&mut self, dice_type: EDice) -> Option<u16> {
        let dice = self.dice_map.get_mut(&dice_type)?;
        return Some(dice.roll());
    }

    fn roll_few_times(&mut self, dice_type: EDice, number_of_rolls: u64) -> Option<Vec<u16>> {
        let mut result = Vec::new();
        for _ in 1..number_of_rolls {
            let single_roll_result = self.roll(dice_type)?;
            result.push(single_roll_result);
        }

        return Some(result);
    }
}
