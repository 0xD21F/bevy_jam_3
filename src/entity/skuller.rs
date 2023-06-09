use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::{
    animation::Animated,
    behaviour::separation::{separation_system, Separation},
};

use super::{
    creature::{Creature, CreatureBundle, Hitbox, Velocity},
    Enemy, EnemyHurtboxDamage, ZSort,
};

pub struct SkullerPlugin;

impl Plugin for SkullerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Skuller>);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Skuller;

#[derive(Bundle)]
pub struct SkullerBundle {
    pub creature: CreatureBundle,
    pub enemy: Enemy,
    pub name: Name,
    pub skuller: Skuller,
    pub separation: Separation,
    pub hurtbox: EnemyHurtboxDamage,
}

impl SkullerBundle {
    pub fn new(
        texture_atlas_handle: Handle<TextureAtlas>,
        sprite_size: f32,
        animation: Animated,
        transform: Transform,
    ) -> Self {
        Self {
            creature: CreatureBundle {
                creature: Creature {
                    acceleration: 2048.0,
                    friction: 512.0,
                    max_speed: 400.0,
                    health: 20.0,
                    max_health: 20.0,
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
            hurtbox: EnemyHurtboxDamage(3),
            enemy: Enemy,
            skuller: Skuller,
            name: Name::new("Skuller"),
            separation: Separation::default(),
        }
    }
}
