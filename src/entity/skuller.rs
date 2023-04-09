use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::{
    animation::Animated,
    behaviour::separation::{separation_system, Separation},
};

use super::{
    creature::{Creature, CreatureBundle, Hitbox, Velocity},
    Enemy, ZSort,
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
                    friction: 1024.0,
                    max_speed: 128.0,
                    health: 100.0,
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
            skuller: Skuller,
            name: Name::new("Skuller"),
            separation: Separation::default(),
        }
    }
}
