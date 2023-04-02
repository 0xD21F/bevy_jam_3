use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use rand::Rng;

use crate::{player::Player, enemy::{Enemy, EnemySpawnTimer, EnemyBundle}, unit::{Velocity, Unit, UnitBundle}, PIXELS_PER_METER};

pub struct SkullerPlugin;

impl Plugin for SkullerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(skuller_movement_system)
            .add_system(spawn_skuller);
    }
}

#[derive(Component, Reflect)]
pub struct Skuller;

impl Default for Skuller {
    fn default() -> Self {
        Self {}
    }
}

pub fn spawn_skuller(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>, mut spawn_timer: ResMut<EnemySpawnTimer>) {
    // Tick the timer
    spawn_timer.timer.tick(time.delta());

    let sprite_size = PIXELS_PER_METER;

    if spawn_timer.timer.finished() {
        let enemy_entity = commands.spawn(EnemyBundle {
            unit_bundle: UnitBundle {
                sprite: SpriteBundle {
                    texture: asset_server.load("sprites/skull.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                        ..default()
                    },
                    ..default()
                },
                collider: Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0),
                unit: Unit {
                    deceleration: 10000.0,
                    ..default()
                },
                ..default()
            },
            ..default()
        });
    }
}

fn skuller_movement_system (
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Enemy, &mut Unit, &Skuller, &mut Velocity, &mut Transform), Without<Player>>,
) {
    let player_position = player_query.single().translation;
    let desired_distance = 64.0; // the desired distance to maintain from the player
    let min_distance = 32.0; // the minimum distance to maintain from the player

    for (mut enemy, mut unit, skuller, mut velocity, mut transform) in enemy_query.iter_mut() {
        let direction = (player_position - transform.translation).normalize();
        if direction == Vec3::ZERO {
            continue;
        }
        let distance = (player_position - transform.translation).length();
        let speed = 100.0;

        if distance >= desired_distance {
            velocity.value = Vec2::new(direction.x * speed, direction.y * speed);
        } else if distance <= min_distance {
            velocity.value = Vec2::new(-direction.x * speed, -direction.y * speed);
        } else {
            let move_direction = direction * (desired_distance - min_distance);
            velocity.value = Vec2::new(move_direction.x * speed, move_direction.y * speed);
        }

        let mut rng = rand::thread_rng();

        let rand_radians = rng.gen_range(0.0..=std::f32::consts::PI);
        let rand_angle_offset = rng.gen_range(0.0..=2.0 * std::f32::consts::PI);
        let wiggle_magnitude = rng.gen_range(0.1..=1.0);
        let wiggle_direction = Vec2::new(
            (rand_radians + rand_angle_offset).cos() * wiggle_magnitude,
            (rand_radians + rand_angle_offset).sin() * wiggle_magnitude,
        );

        transform.translation.x += wiggle_direction.x;
        transform.translation.y += wiggle_direction.y;
    }
}