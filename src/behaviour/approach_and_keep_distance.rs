use bevy::prelude::*;
use rand::Rng;

use crate::entity::creature::{Creature, Velocity};

#[derive(Component, Reflect, Default)]
pub struct ApproachAndKeepDistance {
    pub inner_distance: f32,
    pub outer_distance: f32,
}

pub fn approach_and_keep_distance<T: Component, U: Component>(
    time: Res<Time>,
    target_query: Query<&Transform, With<T>>,
    mut unit_query: Query<
        (
            &Creature,
            &ApproachAndKeepDistance,
            &mut Velocity,
            &mut Transform,
        ),
        Without<T>,
    >,
) {
    let mut rng = rand::thread_rng();

    for (unit, approach_and_circle, mut velocity, transform) in unit_query.iter_mut() {
        // Find the closest target
        let target_position = target_query
            .iter()
            .map(|target_transform| target_transform.translation.truncate())
            .min_by(|a, b| {
                let distance_a = a.distance(transform.translation.truncate());
                let distance_b = b.distance(transform.translation.truncate());
                distance_a
                    .partial_cmp(&distance_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(Vec2::new(0.0, 0.0));

        let enemy_position = transform.translation.truncate();

        let distance_from_player = target_position.distance(enemy_position);
        let change_velocity;

        // move to outer_distance
        if distance_from_player > approach_and_circle.outer_distance {
            let direction = target_position - enemy_position;
            let acceleration = unit.acceleration * time.delta_seconds();
            change_velocity = direction.normalize_or_zero() * acceleration;
        }
        // move to inner_distance
        else if distance_from_player < approach_and_circle.inner_distance
            && distance_from_player > 0.0
        {
            let direction = enemy_position - target_position;
            let acceleration = unit.acceleration * time.delta_seconds();
            change_velocity = direction.normalize_or_zero() * acceleration;
        }
        // wander randomly
        else {
            let mut wander_direction =
                Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero();
            wander_direction += velocity.value.normalize_or_zero();
            let acceleration = unit.acceleration * time.delta_seconds();
            change_velocity = wander_direction.normalize_or_zero() * acceleration;
        }
        let mut new_velocity = velocity.value + change_velocity;

        // apply deceleration
        if change_velocity == Vec2::ZERO {
            let deceleration = unit.deceleration * time.delta_seconds();

            let new_normalized_velocity = new_velocity.normalize_or_zero();

            let mut deceleration_velocity = -new_normalized_velocity * deceleration;

            if deceleration_velocity.length() > new_velocity.length() {
                deceleration_velocity = -new_velocity;
            }

            new_velocity += deceleration_velocity;
        }

        // apply max_speed
        let speed = new_velocity.length();
        if speed > unit.max_speed {
            new_velocity *= unit.max_speed / speed;
        }

        velocity.value = new_velocity;
    }
}
