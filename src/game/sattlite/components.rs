use bevy::prelude::Component;

#[derive(Component)]
pub struct SatteliteComponent {
    pub speed: u32,
    pub angle: f32,
    pub radius: f32,
}
