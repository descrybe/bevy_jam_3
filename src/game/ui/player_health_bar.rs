use crate::game::{
    health::components::HealthComponent,
    player::{components::Player, PLAYER_HEALTH},
};

// use super::constants::*;
use bevy::{prelude::*, window::PrimaryWindow};

const HEALTH_BAR_START_WIDTH: f32 = 110.0;

const PLAYER_HEALTH_BAR_HEIGHT:Val = Val::Px(6.0);

#[derive(Component)]
pub struct HealthBar {}

pub fn spawn_health_bar(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Percent(55.0),
                        left: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Auto,
                    },
                    size: Size::new(Val::Px(HEALTH_BAR_START_WIDTH), PLAYER_HEALTH_BAR_HEIGHT),
                    ..default()
                },
                background_color: Color::rgb(1.0, 0.0, 0.0).into(),
                transform: Transform::from_xyz(0.0, 0.0, 3.0),
                ..default()
            },
            HealthBar {},
        ));
}

pub fn update_health_bar_params(
    mut health_bar_query: Query<&mut Style, (Without<Player>, With<HealthBar>)>,
    health_query: Query<&HealthComponent, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let health = health_query.get_single().unwrap();
    let window = window_query.get_single().unwrap();
    if let Ok(mut health_bar_style) = health_bar_query.get_single_mut() {
        health_bar_style.position.left = Val::Px(window.width() / 2.0 - 57.0);
        let health_percent: f32 = (health.amount() * 100 / PLAYER_HEALTH) as f32;
        
        health_bar_style.size = Size::new(
            Val::Px(health_percent * HEALTH_BAR_START_WIDTH / 100.0),
            Val::Px(8.0),
        );
    }
}
