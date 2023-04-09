use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::animation::Animated;

use super::{ZSort};

#[derive(Component, Reflect)]
pub struct Creature {
    pub acceleration: f32,
    pub friction: f32,
    pub max_speed: f32,
    pub health: f32,
}

impl Default for Creature {
    fn default() -> Self {
        Self {
            acceleration: 128.0,
            friction: 128.0,
            max_speed: 128.0,
            health: 128.0,
        }
    }
}

#[derive(Component, Reflect)]
pub struct Velocity {
    pub value: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub animation: Animated,
    pub sprite: SpriteSheetBundle,
    pub collider: Collider,
    pub velocity: Velocity,
    pub zsort: ZSort,
    pub sensor: Sensor,
    pub hitbox: Hitbox,
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature::default(),
            animation: Animated::default(),
            sprite: Default::default(),
            collider: Collider::ball(1.0),
            sensor: Sensor::default(),
            hitbox: Hitbox,
            velocity: Velocity::default(),
            zsort: ZSort::default(),
        }
    }
}

#[derive(Component, Reflect)]
pub struct Hitbox;

#[derive(Component, Reflect)]
pub struct DontSetFacing;

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_friction_system)
            .add_system(apply_velocity_system)
            .add_system(z_ordering_system)
            .add_system(set_sprite_facing_system);
    }
}

pub fn apply_velocity_system(time: Res<Time>, mut player_info: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in player_info.iter_mut() {
        let delta = velocity.value * time.delta_seconds();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;

        if delta == Vec2::ZERO {
            transform.translation.x = transform.translation.x.round();
            transform.translation.y = transform.translation.y.round();
        }
    }
}

pub fn apply_friction_system(time: Res<Time>, mut player_info: Query<(&mut Velocity, &Creature)>) {
    for (mut velocity, creature) in player_info.iter_mut() {
        if velocity.value != Vec2::ZERO {
            let friction = creature.friction * time.delta_seconds();
            let friction_vector = velocity.value.normalize() * friction;

            let new_velocity = velocity.value - friction_vector;
            if new_velocity.length() < friction {
                velocity.value = Vec2::ZERO;
            } else {
                velocity.value -= friction_vector;
            }
        }
    }
}

pub fn set_sprite_facing_system(
    mut query: Query<(&mut TextureAtlasSprite, &Velocity), Without<DontSetFacing>>,
) {
    for (mut sprite, velocity) in query.iter_mut() {
        if velocity.value.x > 0.0 {
            sprite.flip_x = false;
        } else if velocity.value.x < 0.0 {
            sprite.flip_x = true;
        }
    }
}

pub fn z_ordering_system(
    mut z_sort_query: Query<(&mut Transform, &ZSort)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
) {
    // TODO: Handle multiple cameras
    let (camera_transform, camera) = camera_query.single();

    for (mut transform, zsort) in z_sort_query.iter_mut() {
        // Based on the screen space Y position, set the Z position to be in front of or behind other sprites
        let viewport_pos = camera.world_to_viewport(camera_transform, transform.translation);
        if let Some(pos) = viewport_pos {
            transform.translation.z = (-((pos.y * 2.0) - zsort.offset_y * 2.0) / 1000.0).min(0.0);
        } else {
            transform.translation.z = 0.0;
        }
    }
}
