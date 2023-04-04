use bevy::prelude::*;

use super::creature::{Creature, CreatureBundle, Velocity};

#[derive(Component, Reflect, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub unit_bundle: CreatureBundle,
    pub player: Player,
    pub name: Name,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            unit_bundle: CreatureBundle::default(),
            player: Player::default(),
            name: Name::new("Player"),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_movement_system);
    }
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Creature, &mut Velocity), With<Player>>,
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
