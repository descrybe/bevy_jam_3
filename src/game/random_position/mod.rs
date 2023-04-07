pub mod screen_edge_position_generator;

use bevy::prelude::Plugin;

use self::screen_edge_position_generator::ScreenEdgePositionGenerator;

#[derive(Clone, Copy)]
pub struct StraightLine<T> {
    pub first_point: Point<T>,
    pub second_point: Point<T>,
}

#[derive(Clone, Copy, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub trait PositionGenerator<T> {
    fn generate(&mut self, constraints: &StraightLine<T>) -> Option<Point<T>>;
}

pub struct RandomPositionPlugin;

impl Plugin for RandomPositionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ScreenEdgePositionGenerator>();
    }
}
