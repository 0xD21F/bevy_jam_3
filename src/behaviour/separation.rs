use bevy::prelude::*;
use rand::Rng;

use crate::{
    entity::creature::{Creature, Velocity},
    PIXELS_PER_METER,
};

const EPSILON: f32 = 1e-6;

#[derive(Component, Reflect)]
pub struct Separation {
    pub radius: f32,
    pub separation_strength: f32,
    pub separation_force: f32,
    pub max_speed_during_separation: Option<f32>,
    pub max_speed_reset: Option<f32>,
}

impl Default for Separation {
    fn default() -> Self {
        Self {
            radius: PIXELS_PER_METER * 2.0,
            separation_strength: 2.0,
            separation_force: 25.0,
            max_speed_during_separation: None,
            max_speed_reset: None,
        }
    }
}

fn separate(
    position: Vec2,
    radius: f32,
    separation_force: f32,
    positions: &[Vec2],
) -> Option<(Vec2, f32)> {
    let mut steer = Vec2::ZERO;
    let mut count = 0;

    for other_pos in positions
        .iter()
        .filter(|&&p| !matches!(p, pos if pos == position))
    {
        let distance = position.distance(*other_pos);
        if distance < radius {
            let diff = position - *other_pos;
            let diff = diff.normalize_or_zero();
            steer += diff;
            count += 1;
        }
    }
    let mut rng = rand::thread_rng();

    if count == 0 {
        steer = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero();
    } else {
        steer /= count as f32;
    }
    steer = steer.normalize_or_zero() * separation_force;
    Some((steer, steer.length()))
}

// TODO: This is O(n^2), look into using a quadtree or something similar. Performance isn't an issue until there are a lot of enemies on screen.
pub fn separation_system<T: Component>(
    mut enemy_query: Query<(&Separation, &Transform, &mut Velocity, &T, &mut Creature)>,
    // mut debug_shapes: ResMut<DebugShapes>,
    // mut debug_lines: ResMut<DebugLines>,
) {
    // Create a vector of positions for the enemy entities.
    let mut positions: Vec<Vec2> = Vec::new();
    for (_, transform, _, _, _) in enemy_query.iter_mut() {
        positions.push(transform.translation.truncate());
    }

    // Loop through the query and calculate the separation for each enemy entity.
    // Adjust the Velocity for each entity to steer away from nearby enemies.
    for (separation, transform, mut velocity, _, mut creature) in enemy_query.iter_mut() {
        // Get the position of the current enemy entity.
        let enemy_position = transform.translation.truncate();

        // Filter and map the positions vector to get the positions of nearby enemies.
        let nearby_positions: Vec<Vec2> = positions
            .iter()
            .filter(|&other_position| {
                *other_position != enemy_position
                    && enemy_position.distance(*other_position) < separation.radius
            })
            .copied()
            .collect();

        // Calculate the separation for the current enemy entity and adjust its Velocity accordingly.
        if let Some((separation_vector, steer_away)) = separate(
            enemy_position,
            separation.radius,
            separation.separation_force,
            &nearby_positions,
        ) {
            if let Some(max_speed_during_separation) = separation.max_speed_during_separation {
                if !nearby_positions.is_empty() {
                    creature.max_speed = max_speed_during_separation;
                } else if let Some(max_speed_reset) = separation.max_speed_reset {
                    creature.max_speed = max_speed_reset;
                }
            }
            let steer_away = if steer_away == 0.0 {
                EPSILON
            } else {
                steer_away
            };
            let separation_mag = separation_vector.length();
            let separation_scale = if separation_mag == 0.0 {
                0.0
            } else {
                1.0 / steer_away
            };

            if cfg!(debug_assertions) {
                // debug_lines.line(
                //     transform.translation,
                //     transform.translation + separation.extend(0.0),
                //     0.0,
                // );
            }

            let separation_scaled = separation_vector * separation_scale;

            velocity.value += separation_scaled * separation.separation_strength;
        }

        if cfg!(debug_assertions) {
            // debug_shapes
            //     .rect()
            //     .position(transform.translation)
            //     .size(Vec2::new(separation.radius, separation.radius));
        }
    }
}
