use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::animation::Animated;

use super::ZSort;

#[derive(Component, Reflect)]
pub struct Creature {
    pub acceleration: f32,
    pub deceleration: f32,
    pub max_speed: f32,
    pub health: f32,
}

impl Default for Creature {
    fn default() -> Self {
        Self {
            acceleration: 128.0,
            deceleration: 128.0,
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
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature::default(),
            animation: Animated::default(),
            sprite: Default::default(),
            collider: Collider::cuboid(1.0, 1.0),
            velocity: Velocity::default(),
            zsort: ZSort::default(),
        }
    }
}

#[derive(Component, Reflect)]
pub struct DontSetFacing;

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_velocity_system)
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
    let (camera_transform, camera) = camera_query.single();

    for (mut creature_transform, zsort) in z_sort_query.iter_mut() {
        // Based on the screen space Y position, set the Z position to be in front of or behind other sprites based on the bottom of the sprite
        let viewport_pos =
            camera.world_to_viewport(camera_transform, creature_transform.translation);
        if let Some(pos) = viewport_pos {
            creature_transform.translation.z =
                (-((pos.y * 2.0) - zsort.offset_y * 2.0) / 1000.0).min(0.0);
        } else {
            // creature_transform.translation.z = 0.0;
        }
    }
}
