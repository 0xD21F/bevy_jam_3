use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LdtkEntityAppExt},
    EntityInstance, LdtkEntity,
};

use crate::game::{level_manager::LevelObject, GameState};

use super::player::Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct LevelStartEntityBundle {
    #[from_entity_instance]
    #[bundle]
    pub level_start: LevelStartBundle,
}

#[derive(Component, Clone, Default, Debug)]
pub struct LevelStart;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct LevelStartBundle {
    #[bundle]
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub level_start: LevelStart,
    pub level_object: LevelObject, // Marker component for entities related to the current level
}

fn find_field_value<'a>(
    entity_instance: &'a EntityInstance,
    field_identifier: &str,
) -> Option<&'a FieldValue> {
    entity_instance
        .field_instances
        .iter()
        .find(|f| f.identifier.as_str() == field_identifier)
        .map(|field_instance| &field_instance.value)
}

impl From<&EntityInstance> for LevelStartBundle {
    fn from(entity_instance: &EntityInstance) -> LevelStartBundle {
        // Helper closure to get field value and simplify error handling
        let _get_field_value = |field_identifier: &str| {
            find_field_value(entity_instance, field_identifier)
                .ok_or_else(|| format!("Missing field: {}", field_identifier))
        };

        let level_start_position = entity_instance.grid.as_vec2();

        LevelStartBundle {
            transform: Transform::from_xyz(level_start_position.x, level_start_position.y, 0.0),
            global_transform: GlobalTransform::default(),
            level_start: LevelStart,
            level_object: LevelObject,
        }
    }
}

pub struct LevelStartPlugin;

impl Plugin for LevelStartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_ldtk_entity::<LevelStartEntityBundle>("LevelStart");

        app.add_system(level_start_system.in_schedule(OnEnter(GameState::InLevel)));
    }
}

// Move the player to the start of the level
pub fn level_start_system(
    query: Query<(&LevelStart, &Transform), Without<Player>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let player = player_query.get_single_mut();
    let level_start = query.get_single();

    // If player and level exist
    if let (Ok(mut player), Ok((_, transform))) = (player, level_start) {
        // Move player to level start
        player.translation = transform.translation;
    }
}
