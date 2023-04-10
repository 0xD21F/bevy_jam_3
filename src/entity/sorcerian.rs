use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::{
    animation::Animated,
    behaviour::separation::{separation_system, Separation},
    PIXELS_PER_METER,
};

use super::{
    creature::{Creature, CreatureBundle, Hitbox, Velocity},
    Enemy, ZSort, EnemyHurtboxDamage,
};

pub struct SorcerianPlugin;

impl Plugin for SorcerianPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Sorcerian>);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Sorcerian;

#[derive(Bundle)]
pub struct SorcerianBundle {
    pub creature: CreatureBundle,
    pub enemy: Enemy,
    pub name: Name,
    pub sorcerian: Sorcerian,
    pub hurtbox: EnemyHurtboxDamage,
}

impl SorcerianBundle {
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
                    health: 500.0,
                    max_health: 500.0,
                    damage_invulnerability: Timer::from_seconds(1.5, TimerMode::Once),
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
            hurtbox: EnemyHurtboxDamage(12),
            enemy: Enemy,
            sorcerian: Sorcerian,
            name: Name::new("Sorcerian"),
        }
    }
}
