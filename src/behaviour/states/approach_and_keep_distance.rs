use bevy::prelude::*;
use rand::Rng;

use crate::entity::creature::{Creature, Knockback, Velocity};

// Entities in the `ApproachAndKeepDistance` state should move towards the given entity if they are
// too far away, and move away if they are too close
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct ApproachAndKeepDistance {
    pub target: Entity,
    pub inner_distance: f32,
    pub outer_distance: f32,
}

pub fn approach_and_keep_distance(
    transforms: Query<&Transform>,
    mut approach_query: Query<(
        Entity,
        &mut Velocity,
        &Creature,
        &ApproachAndKeepDistance,
        Option<&Knockback>,
    )>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut velocity, creature, approach, knockback) in approach_query.iter_mut() {
        {
            let target_position = transforms
                .get(approach.target)
                .unwrap_or(&Transform::from_translation(Vec3::ZERO))
                .translation
                .truncate();
            let follower_position = transforms.get(entity).unwrap().translation.truncate();

            let distance_from_player = target_position.distance(follower_position);

            let change_velocity;
            // move to outer_distance
            if distance_from_player > approach.outer_distance {
                let direction = target_position - follower_position;
                let acceleration = creature.acceleration * time.delta_seconds();
                change_velocity = direction.normalize_or_zero() * acceleration;
            }
            // move to inner_distance
            else if distance_from_player < approach.inner_distance && distance_from_player > 0.0 {
                let direction = follower_position - target_position;
                let acceleration = creature.acceleration * time.delta_seconds();
                change_velocity = direction.normalize_or_zero() * acceleration;
            }
            // wander randomly
            else {
                let mut wander_direction =
                    Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                        .normalize_or_zero();
                wander_direction += velocity.value.normalize_or_zero();
                let acceleration = creature.acceleration * time.delta_seconds();
                change_velocity = wander_direction.normalize_or_zero() * acceleration;
            }
            let mut new_velocity = velocity.value + change_velocity;

            // apply max_speed
            let speed = new_velocity.length();
            if knockback.is_none() && speed > creature.max_speed {
                new_velocity *= creature.max_speed / speed;
            }

            velocity.value = new_velocity;
        }
    }
}
