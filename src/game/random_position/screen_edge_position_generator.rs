use crate::{
    dice::resources::*,
    game::random_position::{Point, PositionGenerator, StraightLine},
};
use bevy::prelude::{FromWorld, Resource, World};

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

fn normalize_number(value: f32, min: f32, max: f32) -> f32 {
    return min + value * (max - min) / 100.0;
}

fn normalize_x(value: f32, constraints: &StraightLine<f32>) -> f32 {
    return normalize_number(value, constraints.first_point.x, constraints.second_point.x);
}

fn normalize_y(value: f32, constraints: &StraightLine<f32>) -> f32 {
    return normalize_number(value, constraints.first_point.y, constraints.second_point.y);
}

#[derive(Resource)]
pub struct ScreenEdgePositionGenerator {
    dice_service: DiceService,
}

impl FromWorld for ScreenEdgePositionGenerator {
    fn from_world(_world: &mut World) -> Self {
        ScreenEdgePositionGenerator {
            dice_service: DiceService::new(),
        }
    }
}

impl PositionGenerator<f32> for ScreenEdgePositionGenerator {
    fn generate(&mut self, constraints: &StraightLine<f32>) -> Option<Point<f32>> {
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
