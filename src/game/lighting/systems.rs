use bevy::{
    prelude::{Commands, Entity, Query, With, Vec2, default, Transform, AssetServer, Res, ResMut, EventWriter},
    sprite::{Sprite, SpriteBundle}, time::Time,
};

use crate::game::{flight::resources::FireSpawnConfig, damage::{components::DamageDealerComponent, events::DamageEvent}};

use crate::game::{target::components::TargetHolderComponent, enemy::components::Enemy};

use super::components::Lighting;

pub fn spawn_lightning_bolts(
    mut commands: Commands,
    lightning_query: Query<&TargetHolderComponent, With<Lighting>>,
    enemy_query: Query<(&Transform, Entity), With<Enemy>>,
    asset_service: Res<AssetServer>,
    mut config: ResMut<FireSpawnConfig>,
    time: Res<Time>,
    mut damage_event_writer: EventWriter<DamageEvent>,
) {
    config.timer.tick(time.delta());

    if !config.timer.finished() {
        return;
    }

    if enemy_query.is_empty(){
        return;
    }

    let mut has_lightning_on_enemy = false;
    for (enemy_transform, enemy_entity) in enemy_query.iter() {
        has_lightning_on_enemy = false;
        for lightning_target in lightning_query.iter() {
            if lightning_target.target_entity == enemy_entity {
                has_lightning_on_enemy = true;
                break;
            }
        }
        if !has_lightning_on_enemy {
            spawn_lightning_bolt(&mut commands,
                enemy_entity, enemy_transform,
                &asset_service, damage_event_writer);
            return;
        }
    }
}

fn spawn_lightning_bolt(
    commands: &mut Commands,
    target_entity: Entity,
    target_transform: &Transform,
    asset_service: &Res<AssetServer>,
    mut damage_event_writer: EventWriter<DamageEvent>,
) {
    let lighing_entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Option::Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            transform: *target_transform,
            texture: asset_service.load("sprites/projectile.png"),
            ..default()
        },
        Lighting {},
        TargetHolderComponent {
            target_entity: target_entity,
        },
        DamageDealerComponent {
            damage: 1000,
        }
    )).id();
    damage_event_writer.send(DamageEvent {
        dealer: lighing_entity,
        target: target_entity,
    });
}