use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Sensor};
use rand::Rng;

use crate::{
    animation::Animated,
    app_state::AppState,
    behaviour::{
        separation::{separation_system, Separation},
        states::approach_and_keep_distance::ApproachAndKeepDistance,
    },
    PIXELS_PER_METER,
};

use super::{
    creature::{Creature, CreatureBundle, Hitbox, Velocity},
    Enemy, EnemyHurtboxDamage, ZSort,
};

pub struct GoblinPlugin;

impl Plugin for GoblinPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Goblin>);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Goblin;

#[derive(Bundle)]
pub struct GoblinBundle {
    pub creature: CreatureBundle,
    pub enemy: Enemy,
    pub name: Name,
    pub goblin: Goblin,
    pub separation: Separation,
    pub hurtbox: EnemyHurtboxDamage,
}

impl GoblinBundle {
    pub fn new(
        texture_atlas_handle: Handle<TextureAtlas>,
        sprite_size: f32,
        animation: Animated,
        transform: Transform,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let max_speed = rng.gen_range(150.0..250.0);
        Self {
            creature: CreatureBundle {
                creature: Creature {
                    acceleration: rng.gen_range(1000.0..2000.0),
                    friction: rng.gen_range(500.0..750.0),
                    max_speed: max_speed,
                    health: 80.0,
                    max_health: 80.0,
                    damage_invulnerability: Timer::from_seconds(1.0, TimerMode::Once),
                },
                animation: animation.clone(),
                sprite: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation.first),
                    transform,
                    ..default()
                },
                collider: Collider::ball(sprite_size / 3.0),
                velocity: Velocity::default(),
                zsort: ZSort {
                    offset_y: -(sprite_size / 2.0),
                },
                sensor: Sensor,
                hitbox: Hitbox,
            },
            hurtbox: EnemyHurtboxDamage(4),
            enemy: Enemy,
            goblin: Goblin,
            name: Name::new("Goblin"),
            separation: Separation {
                radius: PIXELS_PER_METER * 3.0,
                separation_force: 1000.0,
                max_speed_during_separation: Some(1280.0),
                max_speed_reset: Some(max_speed),
                ..default()
            },
        }
    }
}
