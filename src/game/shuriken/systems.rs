use std::f32::consts::PI;

use super::{components::Shuriken, SHURIKEN_DAMAGE};
use crate::{
    assets_cache::resources::AssetsCache,
    game::{
        collision::components::{Collidable, CollisionData},
        damage::components::{DamageDealerComponent, SelfDestructable},
        enemy::components::Enemy,
        player::components::Player,
        rotator::components::Rotator,
        sattlite::components::SatteliteComponent,
    },
};

use bevy::{
    prelude::{Commands, Entity, Query, Res, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

const CIRCLE_SIZE: f32 = 30.0;
const ROTATION_SPEED: f32 = 360.0;
const ANGLE_DELTA: f32 = 60.0;
const SATTELITE_SPEED: u32 = 3;
const SATTELITE_RADIUS: f32 = 100.0;

static mut SHURIKEN_SPAWN_COUNT: u8 = 2;

pub fn spawn_circle(
    mut commands: Commands,
    asset_service: Res<AssetsCache>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let mut angle_delta = 0.0;
    if enemy_query.is_empty() {
        return;
    }
    //TODO: delete this block and change it to something appreciable
    if unsafe { SHURIKEN_SPAWN_COUNT } == 0 {
        return;
    } else {
        if unsafe { SHURIKEN_SPAWN_COUNT } == 1 {
            angle_delta = ANGLE_DELTA;
        }
        unsafe { SHURIKEN_SPAWN_COUNT -= 1 };
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Option::Some(Vec2::new(CIRCLE_SIZE, CIRCLE_SIZE)),
                ..default()
            },
            transform: player_query.get_single().unwrap().clone(),
            texture: asset_service.sprites.projectiles.shuriken.clone(),
            ..default()
        },
        Shuriken {},
        Rotator {
            angle: ROTATION_SPEED,
        },
        DamageDealerComponent {
            damage: SHURIKEN_DAMAGE,
        },
        Collidable {
            size: Vec2 {
                x: CIRCLE_SIZE,
                y: CIRCLE_SIZE,
            },
            collision: CollisionData {
                is_collided: false,
                collision_side: Vec::new(),
            },
        },
        SatteliteComponent {
            speed: SATTELITE_SPEED,
            angle: angle_delta,
            radius: SATTELITE_RADIUS,
        },
    ));
}
