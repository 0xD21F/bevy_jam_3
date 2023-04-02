use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugShapes};
use bevy_rapier2d::prelude::*;

#[derive(Component, Reflect)]
pub struct Unit {
    pub acceleration: f32,
    pub deceleration: f32,
    pub max_speed: f32,
    pub health: f32,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            acceleration: 500.0,
            deceleration: 500.0,
            max_speed: 500.0,
            health: 100.0,
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

#[derive(Component, Reflect)]
pub struct Flock {
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub radius: f32,
}

impl Default for Flock {
    fn default() -> Self {
        Self {
            separation_weight: 1.0,
            alignment_weight: 1.0,
            radius: 1.0,
        }
    }
}

#[derive(Bundle)]
pub struct UnitBundle {
    pub unit: Unit,
    pub sprite: SpriteBundle,
    pub collider: Collider,
    pub velocity: Velocity,
}

impl Default for UnitBundle {
    fn default() -> Self {
        Self {
            unit: Unit::default(),
            sprite: Default::default(),
            collider: Collider::cuboid(1.0, 1.0),
            velocity: Velocity::default(),
        }
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(apply_velocity_system);
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