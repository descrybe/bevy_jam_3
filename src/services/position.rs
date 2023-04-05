pub mod enemies_spawn_position;

pub struct Point<T> {
    x: T,
    y: T,
}

pub trait PositionGenerator<T> {
    fn new() -> Self;
    fn generate(&self) -> Point<T>;
}
