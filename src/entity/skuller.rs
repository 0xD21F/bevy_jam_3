use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    behaviour::{
        approach_and_keep_distance::{approach_and_keep_distance, ApproachAndKeepDistance},
        separation::{separation_system, Separation},
    },
    player::Player,
    PIXELS_PER_METER, animation::Animated,
};

use super::{
    creature::{Creature, CreatureBundle, Velocity},
    Enemy,
};

pub struct SkullerPlugin;

impl Plugin for SkullerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Skuller>)
            .add_system(approach_and_keep_distance::<Player, Skuller>);
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
    pub approach_and_keep_distance: ApproachAndKeepDistance,
    pub separation: Separation,
}

impl SkullerBundle {
    pub fn new(texture_atlas_handle: Handle<TextureAtlas>, sprite_size: f32, animation: Animated, transform: Transform) -> Self {
        Self {
            creature: CreatureBundle {
                creature: Creature {
                    acceleration: 500.0,
                    deceleration: 500.0,
                    max_speed: 150.0,
                    health: 100.0,
                },
                animation: animation.clone(),
                sprite: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation.first),
                    transform: transform.clone(),
                    ..default()
                },
                collider: Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0),
                velocity: Velocity::default(),
            },
            enemy: Enemy,
            skuller: Skuller,
            name: Name::new("Skuller"),
            approach_and_keep_distance: ApproachAndKeepDistance {
                inner_distance: PIXELS_PER_METER * 2.0,
                outer_distance: PIXELS_PER_METER * 4.0,
            },
            separation: Separation::default(),
        }
    }
}