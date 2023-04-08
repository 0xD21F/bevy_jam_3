use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    animation::Animated,
    behaviour::separation::{separation_system, Separation},
    PIXELS_PER_METER,
};

use super::{
    creature::{Creature, CreatureBundle, Velocity},
    Enemy, ZSort,
};

pub struct MutantPlugin;

impl Plugin for MutantPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Mutant>);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Mutant;

#[derive(Bundle)]
pub struct MutantBundle {
    pub creature: CreatureBundle,
    pub enemy: Enemy,
    pub name: Name,
    pub mutant: Mutant,
    pub separation: Separation,
}

impl MutantBundle {
    pub fn new(
        texture_atlas_handle: Handle<TextureAtlas>,
        sprite_size: f32,
        animation: Animated,
        transform: Transform,
    ) -> Self {
        Self {
            creature: CreatureBundle {
                creature: Creature {
                    acceleration: 512.0,
                    friction: 256.0,
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
                    offset_y: -(sprite_size),
                },
            },
            enemy: Enemy,
            mutant: Mutant,
            name: Name::new("Mutant"),
            separation: Separation {
                radius: PIXELS_PER_METER * 2.0,
                separation_force: 512.0,
                ..default()
            },
        }
    }
}
