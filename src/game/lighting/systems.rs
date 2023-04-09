use bevy::{
    prelude::{default, Commands, Entity, EventWriter, Query, Res, ResMut, Transform, Vec2, With},
    sprite::{Sprite, SpriteBundle},
    time::Time,
};
use rand::{seq::IteratorRandom, thread_rng};

use crate::{
    assets_cache::resources::AssetsCache,
    game::{
        damage::{components::DamageDealerComponent, events::DamageEvent},
        flight::resources::FireSpawnConfig,
    },
};

use crate::game::{enemy::components::Enemy, target::components::TargetHolderComponent};

use super::components::Lighting;

pub fn spawn_lightning_bolts(
    mut commands: Commands,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    asset_service: Res<AssetsCache>,
    mut config: ResMut<FireSpawnConfig>,
    time: Res<Time>,
    mut damage_event_writer: EventWriter<DamageEvent>,
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
    )
}

fn spawn_lightning_bolt(
    commands: &mut Commands,
    target_entity: Entity,
    target_transform: &Transform,
    asset_service: &Res<AssetsCache>,
    damage_event_writer: &mut EventWriter<DamageEvent>,
) {
    let lighing_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                transform: *target_transform,
                texture: asset_service.sprites.projectiles.bottle.clone(),
                ..default()
            },
            Lighting {},
            TargetHolderComponent {
                target_entity: target_entity,
            },
            DamageDealerComponent { damage: 1000 },
        ))
        .id();
    damage_event_writer.send(DamageEvent {
        dealer: lighing_entity,
        target: target_entity,
    });
}
