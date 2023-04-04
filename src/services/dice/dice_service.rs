use super::{
    dice_type::EDice,
    roll_result::{EDiceSide, RollResult},
};

pub trait DiceServiceTrait {
    fn roll(&self, dice_type: EDice) -> RollResult;
}

pub struct DiceService();

impl DiceServiceTrait for DiceService {
    fn roll(&self, dice_type: EDice) -> RollResult {
        return RollResult {
            side: EDiceSide::Side1,
            side_as_number: 1,
        };
    }
}
