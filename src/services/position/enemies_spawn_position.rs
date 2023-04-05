use std::ops::{Add, Div, Mul, Sub};

use crate::services::dice::{DiceRoller, DiceService, EDice};

use super::{Point, PositionGenerator, StraightLine};

enum ScreenSides {
    Left = 1,
    Up = 2,
    Right = 3,
    Down = 4,
}

impl From<u16> for ScreenSides {
    fn from(value: u16) -> Self {
        let result = match value {
            x if x == ScreenSides::Left as u16 => ScreenSides::Left,
            x if x == ScreenSides::Right as u16 => ScreenSides::Right,
            x if x == ScreenSides::Up as u16 => ScreenSides::Up,
            x if x == ScreenSides::Down as u16 => ScreenSides::Down,
            _ => ScreenSides::Left,
        };

        return result;
    }
}

fn normalize_number<T>(value: T, min: T, max: T) -> T
where
    <T as Mul<<T as Sub>::Output>>::Output: Div<u32>,
    T: Sub
        + Mul<<T as Sub>::Output>
        + Add<<<T as Mul<<T as Sub>::Output>>::Output as Div<u32>>::Output, Output = T>
        + std::marker::Copy,
{
    return min + value * (max - min) / 100;
}

fn normalize_x<T>(value: T, constraints: &StraightLine<T>) -> T
where
    <T as Mul<<T as Sub>::Output>>::Output: Div<u32>,
    T: Sub
        + Mul<<T as Sub>::Output>
        + Add<<<T as Mul<<T as Sub>::Output>>::Output as Div<u32>>::Output, Output = T>
        + std::marker::Copy,
{
    return normalize_number::<T>(value, constraints.first_point.x, constraints.second_point.x);
}

fn normalize_y<T>(value: T, constraints: &StraightLine<T>) -> T
where
    <T as Mul<<T as Sub>::Output>>::Output: Div<u32>,
    T: Sub
        + Mul<<T as Sub>::Output>
        + Add<<<T as Mul<<T as Sub>::Output>>::Output as Div<u32>>::Output, Output = T>
        + std::marker::Copy,
{
    return normalize_number::<T>(value, constraints.first_point.y, constraints.second_point.y);
}

pub struct EnemiesSpawnPositionService {
    dice_service: DiceService,
}

impl PositionGenerator<u32> for EnemiesSpawnPositionService {
    fn new() -> Self {
        return EnemiesSpawnPositionService {
            dice_service: DiceService::new(),
        };
    }

    fn generate(&mut self, constraints: &StraightLine<u32>) -> Option<Point<u32>> {
        let rolled_side = self.dice_service.roll(EDice::D4)?;
        let rolled_position = self.dice_service.roll(EDice::D100)?;
        let chosen_side = ScreenSides::from(rolled_side);

        let result = match chosen_side {
            ScreenSides::Left => Point {
                x: constraints.first_point.x,
                y: normalize_y(rolled_position.into(), constraints),
            },
            ScreenSides::Up => Point {
                x: normalize_x(rolled_position.into(), constraints),
                y: constraints.second_point.y,
            },
            ScreenSides::Right => Point {
                x: constraints.second_point.x,
                y: normalize_y(rolled_position.into(), constraints),
            },
            ScreenSides::Down => Point {
                x: normalize_x(rolled_position.into(), constraints),
                y: constraints.first_point.y,
            },
        };

        return Some(result);
    }
}
