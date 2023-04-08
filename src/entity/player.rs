use bevy::prelude::*;

use crate::app_state::AppState;

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
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (unit, mut velocity) in player_info.iter_mut() {
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
        let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
        let attack = keyboard_input.any_pressed([KeyCode::Z, KeyCode::O]);
        let dash = keyboard_input.any_pressed([KeyCode::X, KeyCode::P]);
        let quit = keyboard_input.any_pressed([KeyCode::Escape]);
        if quit {
            next_state.set(AppState::MainMenu);
        }

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        if x_axis != 0 || y_axis != 0 {
            let input_vector = Vec2::new(x_axis as f32, y_axis as f32);
            let input_magnitude = input_vector.length();
            let normalized_input_vector = input_vector / input_magnitude;

            let acceleration_vector = normalized_input_vector
                * unit.acceleration
                * input_magnitude
                * time.delta_seconds();
            velocity.value += acceleration_vector;

            // Limit maximum speed
            let max_speed = unit.max_speed;
            velocity.value = velocity.value.clamp_length_max(max_speed);
        }
    }
}
