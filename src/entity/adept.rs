use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::{
    animation::Animated,
    behaviour::separation::{separation_system, Separation},
    PIXELS_PER_METER,
};

use super::{
    creature::{Creature, CreatureBundle, Hitbox, Velocity},
    Enemy, ZSort,
};

pub struct AdeptPlugin;

impl Plugin for AdeptPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Adept>);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Adept;

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
            adept: Adept,
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
