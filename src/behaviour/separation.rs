use bevy::prelude::*;

use rand::Rng;

use crate::{entity::creature::Velocity, PIXELS_PER_METER};

const EPSILON: f32 = 1e-6;
const SEPARATION_FORCE: f32 = 25.0;
const SEPARATION_STRENGTH: f32 = 2.0;

#[derive(Component, Reflect)]
pub struct Separation {
    pub radius: f32,
}

impl Default for Separation {
    fn default() -> Self {
        Self {
            radius: PIXELS_PER_METER * 2.0,
        }
    }
}

fn separate(position: Vec2, radius: f32, positions: &[Vec2]) -> Option<(Vec2, f32)> {
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
        // if all entities are at the same spot, return a random direction
        steer = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero();
    } else {
        steer /= count as f32;
    }
    steer = steer.normalize_or_zero() * SEPARATION_FORCE;
    Some((steer, steer.length()))
}

pub fn separation_system<T: Component>(
    mut enemy_query: Query<(&Separation, &Transform, &mut Velocity, &T)>,
    // mut debug_shapes: ResMut<DebugShapes>,
    // mut debug_lines: ResMut<DebugLines>,
) {
    // Create a vector of positions for the enemy entities.
    let mut positions: Vec<Vec2> = Vec::new();
    for (_, transform, _, _) in enemy_query.iter_mut() {
        positions.push(transform.translation.truncate());
    }

    // Loop through the query and calculate the separation for each enemy entity.
    // Adjust the Velocity for each entity to steer away from nearby enemies.
    for (separation, transform, mut velocity, _) in enemy_query.iter_mut() {
        // Get the position of the current enemy entity.
        let enemy_position = transform.translation.truncate();

        // Filter and map the positions vector to get the positions of nearby enemies.
        let nearby_positions: Vec<Vec2> = positions
            .iter()
            .filter(|&other_position| enemy_position.distance(*other_position) < separation.radius)
            .copied()
            .collect();

        // Calculate the separation for the current enemy entity and adjust its Velocity accordingly.
        if let Some((separation, steer_away)) =
            separate(enemy_position, separation.radius, &nearby_positions)
        {
            let steer_away = if steer_away == 0.0 {
                EPSILON
            } else {
                steer_away
            };
            let separation_mag = separation.length();
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

            let separation_scaled = separation * separation_scale;

            velocity.value += separation_scaled * SEPARATION_STRENGTH;
        }

        if cfg!(debug_assertions) {
            // debug_shapes
            //     .rect()
            //     .position(transform.translation)
            //     .size(Vec2::new(separation.radius, separation.radius));
        }
    }
}
