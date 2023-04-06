use bevy::prelude::*;
use rand::Rng;

use crate::entity::creature::{Creature, Velocity};

// Entities in the `Wander` state should wander around
#[derive(Clone, Component, Reflect, Default)]
#[component(storage = "SparseSet")]
pub struct Wander {
    wander_node: Option<Vec2>,
}

const NEXT_WANDER_NODE_DISTANCE: f32 = 16.0;
const REACHED_WANDER_NODE_DISTANCE: f32 = 2.0;

pub fn wander(
    mut wander_query: Query<(&mut Velocity, &Creature, &mut Wander, &Transform)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (mut velocity, creature, mut wander, transform) in wander_query.iter_mut() {
        let unit_position = transform.translation.truncate();
        
        // If the wander.wander_node is None, or if the REACHED_WANDER_NODE_DISTANCE is reached, then set a new wander node
        if wander.wander_node.is_none() || unit_position.distance(wander.wander_node.unwrap()) < REACHED_WANDER_NODE_DISTANCE {
            wander.wander_node = Some(unit_position + Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero() * NEXT_WANDER_NODE_DISTANCE);
        }

        // Get the direction from the unit to the wander node
        let direction = wander.wander_node.unwrap() - unit_position;

        // Set the unit's velocity to the direction, multiplied by the acceleration
        velocity.value += direction.normalize_or_zero() * creature.acceleration * time.delta_seconds();
    }
}