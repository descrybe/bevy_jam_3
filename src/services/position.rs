pub mod enemies_spawn_position;

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
    fn new() -> Self;
    fn generate(&mut self, constraints: &StraightLine<T>) -> Option<Point<T>>;
}
