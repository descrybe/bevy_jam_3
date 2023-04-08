use bevy::{prelude::World, window::Window};

// pub mod enemy_factory;

pub trait Factory<TType> {
    fn create(&self, product_type: TType, quantity: u32, world: &mut World, window: Window);
}
