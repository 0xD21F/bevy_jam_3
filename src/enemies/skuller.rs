use std::rc::Rc;

use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugShapes};
use bevy_rapier2d::prelude::Collider;
use rand::Rng;

use crate::{
    player::Player,
    unit::{Unit, UnitBundle, Velocity},
    PIXELS_PER_METER, sprite_sheet_animation::{AnimationIndices, self, AnimationTimer},
};

use super::{Enemy, EnemyBundle, EnemySpawnTimer};

const EPSILON: f32 = 1e-6;
const SEPARATION_FORCE: f32 = 25.0;
const SEPARATION_STRENGTH: f32 = 1.0;

pub struct SkullerPlugin;

impl Plugin for SkullerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(skuller_separation_system)
            .add_system(skuller_movement_system)
            .add_system(spawn_skuller);
    }
}

#[derive(Component)]
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

#[derive(Component, Reflect, Default)]
pub struct Skuller {
    pub inner_distance: f32,
    pub outer_distance: f32,
}

pub fn spawn_skuller(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Tick the timer
    spawn_timer.timer.tick(time.delta());

    let texture_handle = asset_server.load("sprites/skuller.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 4 };

    let sprite_size = PIXELS_PER_METER;

    if spawn_timer.timer.finished() {
        let _enemy_entity = commands
            .spawn(EnemyBundle {
                unit_bundle: UnitBundle {
                    sprite: SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        sprite: TextureAtlasSprite::new(animation_indices.first),
                        ..default()
                    },
                    collider: Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0),
                    unit: Unit {
                        acceleration: 150.0,
                        deceleration: 1000.0,
                        max_speed: 150.0,
                        ..default()
                    },
                    animation_indices: animation_indices,
                    animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    ..default()
                },
                ..default()
            })
            .insert(Skuller {
                inner_distance: PIXELS_PER_METER * 6.0,
                outer_distance: PIXELS_PER_METER * 8.0,
            })
            .insert(Separation::default())
            .insert(Enemy);
    }
}

fn skuller_movement_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (&Enemy, &Unit, &Skuller, &mut Velocity, &mut Transform),
        Without<Player>,
    >,
) {
    let player_position = Vec2::new(
        player_query.single().translation.x,
        player_query.single().translation.y,
    );

    let mut rng = rand::thread_rng();

    for (_enemy, unit, skuller, mut velocity, _transform) in enemy_query.iter_mut() {
        let enemy_position = Vec2::new(_transform.translation.x, _transform.translation.y);

        let distance_from_player = player_position.distance(enemy_position);
        let change_velocity;

        // move to outer_distance
        if distance_from_player > skuller.outer_distance {
            let direction = player_position - enemy_position;
            let acceleration = unit.acceleration * time.delta_seconds();
            change_velocity = direction.normalize_or_zero() * acceleration;
        }
        // move to inner_distance
        else if distance_from_player < skuller.inner_distance && distance_from_player > 0.0 {
            let direction = enemy_position - player_position;
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
        steer = Vec2::new(rng.gen_range(0.1..1.0), rng.gen_range(0.1..1.0)).normalize_or_zero();
    } else {
        steer /= count as f32;
    }
    steer = steer.normalize_or_zero() * SEPARATION_FORCE;
    Some((steer, steer.length()))
}

// Defines a function named skuller_separation_system that accepts a query of enemy entities with the required components.
fn skuller_separation_system(
    mut enemy_query: Query<
        (&Separation, &Transform, &mut Velocity, &Skuller, &Unit),
        Without<Player>,
    >,
    mut debug_shapes: ResMut<DebugShapes>,
    mut debug_lines: ResMut<DebugLines>

) {
    // Create a vector of positions for the enemy entities.
    let mut positions: Vec<Vec2> = Vec::new();

    // Loop through the query and calculate the separation for each enemy entity.
    // Adjust the Velocity for each entity to steer away from nearby enemies.
    for (separation, transform, mut velocity, _, unit) in enemy_query.iter_mut() {
        // Get the position of the current enemy entity.
        let enemy_position = transform.translation.truncate();

        // Filter and map the positions vector to get the positions of nearby enemies.
        let nearby_positions: Vec<Vec2> = positions
            .iter()
            .filter(|&other_position| enemy_position.distance(*other_position) < separation.radius)
            .map(|&other_position| other_position)
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
            let separation_scaled = separation * separation_scale;

            velocity.value += separation_scaled * SEPARATION_STRENGTH;
            debug_lines.line(
                transform.translation,
                transform.translation + separation_scaled.extend(0.0) * SEPARATION_STRENGTH * 10.0,
                0.0
            )
        }

        debug_shapes
            .rect()
            .position(transform.translation)
            .size(Vec2::new(separation.radius, separation.radius));
        // Add the current enemy entity's position to the positions vector.
        positions.push(enemy_position);
    }
}
