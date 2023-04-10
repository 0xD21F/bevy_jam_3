use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::{
    animation::Animated,
    behaviour::separation::{separation_system, Separation},
    PIXELS_PER_METER,
};

use super::{
    creature::{Creature, CreatureBundle, Hitbox, Velocity},
    Enemy, EnemyHurtboxDamage, ZSort,
};

pub struct LabBossPlugin;

impl Plugin for LabBossPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<LabBoss>);
    }
}

#[derive(Component, Reflect, Default)]
pub struct LabBoss {
    pub attack_timer: Timer,
}

#[derive(Bundle)]
pub struct LabBossBundle {
    pub creature: CreatureBundle,
    pub enemy: Enemy,
    pub name: Name,
    pub lab_boss: LabBoss,
    pub separation: Separation,
}

impl LabBossBundle {
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
                    max_speed: 175.0,
                    health: 250.0,
                    max_health: 250.0,
                    damage_invulnerability: Timer::from_seconds(1.25, TimerMode::Once),
                },
                animation: animation.clone(),
                sprite: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation.first),
                    transform,
                    ..default()
                },
                collider: Collider::cuboid(sprite_size / 2.5, sprite_size / 2.0),
                velocity: Velocity::default(),
                zsort: ZSort {
                    offset_y: (sprite_size),
                },
                sensor: Sensor,
                hitbox: Hitbox,
            },
            enemy: Enemy,
            lab_boss: LabBoss {
                attack_timer: Timer::from_seconds(0.05, TimerMode::Once),
            },
            name: Name::new("LabBoss"),
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
