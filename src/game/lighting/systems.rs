use bevy::{
    prelude::{
        default, Assets, Commands, Component, Deref, DerefMut, Entity, EventWriter, Query, Res,
        ResMut, Transform, Vec2, With,
    },
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer, TimerMode},
    window::{PrimaryWindow, Window},
};
use rand::{seq::IteratorRandom, thread_rng};

use crate::{
    assets_cache::resources::AssetsCache,
    game::{
        collision::components::{Collidable, CollisionData},
        damage::{
            components::{DamageDealerComponent, SelfDestructable},
            events::DamageEvent,
        },
        flight::resources::FireSpawnConfig,
    },
};

use crate::game::{enemy::components::Enemy, target::components::TargetHolderComponent};

use super::components::Lighting;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

pub fn animate_lighting(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn spawn_lightning_bolts(
    mut commands: Commands,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    asset_service: Res<AssetsCache>,
    mut config: ResMut<FireSpawnConfig>,
    time: Res<Time>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    config.timer.tick(time.delta());

    if !config.timer.finished() {
        return;
    }

    if enemy_query.is_empty() {
        return;
    }

    let mut generator = thread_rng();
    let (enemy_transform, enemy_entity) = enemy_query.iter().choose(&mut generator).unwrap();

    spawn_lightning_bolt(
        &mut commands,
        enemy_entity,
        enemy_transform,
        &asset_service,
        &mut damage_event_writer,
        texture_atlases,
    )
}

fn spawn_lightning_bolt(
    commands: &mut Commands,
    target_entity: Entity,
    target_transform: &Transform,
    asset_service: &Res<AssetsCache>,
    damage_event_writer: &mut EventWriter<DamageEvent>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Animation
    let texture_handle = asset_service.sprites.projectiles.lightning.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(40.0, 200.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 2 };
    let sprite_index = animation_indices.first;

    let lighing_entity = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: sprite_index,
                    custom_size: Option::Some(Vec2::new(40.0, 200.0)),
                    ..default()
                },
                transform: *target_transform,
                ..default()
            },
            animation_indices,
             AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Lighting {},
            TargetHolderComponent {
                target_entity: target_entity,
            },
            DamageDealerComponent { damage: 150 },
            SelfDestructable::new(1.0),
            Collidable {
                size: Vec2 { x: 40.0, y: 200.0 },
                collision: CollisionData {
                    is_collided: false,
                    collision_side: Vec::new(),
                },
            },
        ))
        .id();

    damage_event_writer.send(DamageEvent {
        dealer: lighing_entity,
        target: target_entity,
    });
}
