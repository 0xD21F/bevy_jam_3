use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::{
    animation::Animated,
    behaviour::separation::{separation_system, Separation},
    game::{level_manager::SpawnerBundle, GameState},
    PIXELS_PER_METER,
};

use super::{
    creature::{Creature, CreatureBundle, Hitbox, Velocity},
    spawner::{EnemyType, Spawner},
    Enemy, ZSort,
};

pub struct AdeptPlugin;

impl Plugin for AdeptPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Adept>)
            .add_system(summon_skullers.in_set(OnUpdate(GameState::InLevel)));
    }
}

#[derive(Component, Reflect, Default)]
pub struct Adept {
    pub skuller_timer: Timer,
}

#[derive(Bundle)]
pub struct AdeptBundle {
    pub creature: CreatureBundle,
    pub enemy: Enemy,
    pub name: Name,
    pub adept: Adept,
    pub separation: Separation,
}

impl AdeptBundle {
    pub fn new(
        texture_atlas_handle: Handle<TextureAtlas>,
        sprite_size: f32,
        animation: Animated,
        transform: Transform,
    ) -> Self {
        Self {
            creature: CreatureBundle {
                creature: Creature {
                    acceleration: 1000.0,
                    friction: 500.0,
                    max_speed: 150.0,
                    health: 100.0,
                    max_health: 100.0,
                    damage_invulnerability: Timer::from_seconds(0.5, TimerMode::Once),
                },
                animation: animation.clone(),
                sprite: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation.first),
                    transform,
                    ..default()
                },
                collider: Collider::ball(sprite_size / 2.0),
                velocity: Velocity::default(),
                zsort: ZSort {
                    offset_y: -(sprite_size / 2.0),
                },
                sensor: Sensor,
                hitbox: Hitbox,
            },
            enemy: Enemy,
            adept: Adept {
                skuller_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            },
            name: Name::new("Adept"),
            separation: Separation {
                radius: PIXELS_PER_METER * 1.0,
                separation_force: 1000.0,
                max_speed_during_separation: Some(1280.0),
                max_speed_reset: Some(150.0),
                ..default()
            },
        }
    }
}

pub fn summon_skullers(
    mut commands: Commands,
    mut adept_query: Query<(Entity, &Transform, &mut Adept)>,
    time: Res<Time>,
) {
    for (_adept_entity, adept_transform, mut adept) in adept_query.iter_mut() {
        adept.skuller_timer.tick(time.delta());
        if adept.skuller_timer.finished() {
            commands.spawn((
                SpawnerBundle {
                    spawner: Spawner {
                        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                        spawn_rate: 5,
                        spawn_count: 5,
                        enemy_type: EnemyType::Skuller,
                    },
                },
                *adept_transform,
                GlobalTransform::default(),
            ));
            adept.skuller_timer.reset()
        }
    }
}
