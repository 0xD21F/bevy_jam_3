use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::animation::Animated;

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
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature::default(),
            animation: Animated::default(),
            sprite: Default::default(),
            collider: Collider::cuboid(1.0, 1.0),
            velocity: Velocity::default(),
        }
    }
}

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_velocity_system);
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
