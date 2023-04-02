use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{unit::{UnitBundle, Unit, Velocity}, PIXELS_PER_METER};

#[derive(Component, Reflect)]
pub struct Player;

impl Default for Player {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub unit_bundle: UnitBundle,
    pub player: Player,
    pub name: Name,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            unit_bundle: UnitBundle::default(),
            player: Player::default(),
            name: Name::new("Player"),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_player.on_startup())
            .add_system(player_movement_system);
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite_size = PIXELS_PER_METER * 2.0;

    let player_entity = commands.spawn(PlayerBundle {
        unit_bundle: UnitBundle {
            sprite: SpriteBundle {
                texture: asset_server.load("sprites/ape.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                    ..default()
                },
                ..default()
            },
            collider: Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0),
            ..default()
        },
        ..default()
    });
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Unit, &mut Velocity), With<Player>>,
) {
    for (unit, mut velocity) in player_info.iter_mut() {
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        // X-axis movement
        if x_axis != 0 {
            let acceleration = unit.acceleration * x_axis as f32 * time.delta_seconds();
            velocity.value.x += acceleration;

            // Limit maximum speed
            let max_speed = unit.max_speed;
            velocity.value.x = velocity.value.x.clamp(-max_speed, max_speed);
        } else {
            // Apply deceleration when no input is detected
            let deceleration = unit.deceleration * time.delta_seconds();
            if velocity.value.x.abs() < deceleration {
                velocity.value.x = 0.0;
            } else {
                velocity.value.x -= deceleration * velocity.value.x.signum();
            }
        }

        // Y-axis movement
        if y_axis != 0 {
            let acceleration = unit.acceleration * y_axis as f32 * time.delta_seconds();
            velocity.value.y += acceleration;

            // Limit maximum speed
            let max_speed = unit.max_speed;
            velocity.value.y = velocity.value.y.clamp(-max_speed, max_speed);
        } else {
            // Apply deceleration when no input is detected
            let deceleration = unit.deceleration * time.delta_seconds();
            if velocity.value.y.abs() < deceleration {
                velocity.value.y = 0.0;
            } else {
                velocity.value.y -= deceleration * velocity.value.y.signum();
            }
        }
    }
}
