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

pub struct SlimerPlugin;

impl Plugin for SlimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Slimer>);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Slimer;

#[derive(Bundle)]
pub struct SlimerBundle {
    pub creature: CreatureBundle,
    pub enemy: Enemy,
    pub name: Name,
    pub slimer: Slimer,
    pub separation: Separation,
    pub hurtbox: EnemyHurtboxDamage,
}

impl SlimerBundle {
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
                    health: 25.0,
                    max_health: 25.0,
                    damage_invulnerability: Timer::from_seconds(0.2, TimerMode::Once),
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
            hurtbox: EnemyHurtboxDamage(2),
            enemy: Enemy,
            slimer: Slimer,
            name: Name::new("Slimer"),
            separation: Separation {
                radius: PIXELS_PER_METER * 3.0,
                separation_force: 1024.0,
                max_speed_during_separation: Some(1024.0),
                max_speed_reset: Some(150.0),
                ..default()
            },
        }
    }
}
