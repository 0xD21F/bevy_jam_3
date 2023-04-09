use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LdtkEntityAppExt, LdtkIntCellAppExt},
    EntityInstance, IntGridCell, LdtkEntity, LdtkIntCell, LdtkSettings, LdtkWorldBundle,
    LevelSelection,
};
use bevy_rapier2d::prelude::{ActiveCollisionTypes, Collider, RapierContext};

use crate::{
    app_state::{loading::LevelAssets, AppState},
    camera::{camera_clamp_to_current_level, camera_movement_system},
    entity::spawner::{EnemyType, Spawner},
    game::{
        level_manager::{LevelManager, LevelObject},
        GameState,
    },
    PIXELS_PER_METER,
};

use super::{level_start::LevelStart, player::Player};

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct LevelExitEntityBundle {
    #[from_entity_instance]
    #[bundle]
    pub level_end: LevelExitBundle,
}

#[derive(Component, Clone, Default, Debug)]
pub struct LevelExit;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct LevelExitBundle {
    #[bundle]
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub level_start: LevelExit,
    pub level_object: LevelObject, // Marker component for entities related to the current level
    pub collider: Collider,
    pub collision_types: ActiveCollisionTypes,
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

impl From<&EntityInstance> for LevelExitBundle {
    fn from(entity_instance: &EntityInstance) -> LevelExitBundle {
        // Helper closure to get field value and simplify error handling
        let get_field_value = |field_identifier: &str| {
            find_field_value(entity_instance, field_identifier)
                .ok_or_else(|| format!("Missing field: {}", field_identifier))
        };

        let level_start_position = entity_instance.grid.as_vec2();

        LevelExitBundle {
            transform: Transform::from_xyz(level_start_position.x, level_start_position.y, 0.0),
            global_transform: GlobalTransform::default(),
            level_start: LevelExit,
            level_object: LevelObject,
            collider: Collider::cuboid(PIXELS_PER_METER * 2.1, PIXELS_PER_METER * 2.1),
            collision_types: ActiveCollisionTypes::STATIC_STATIC,
        }
    }
}

pub struct LevelExitPlugin;

impl Plugin for LevelExitPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_ldtk_entity::<LevelExitEntityBundle>("LevelExit");

        app.add_system(level_end_system.in_set(OnUpdate(GameState::LevelComplete)));
    }
}

// Move the player to the next level if this level is over
pub fn level_end_system(
    rapier_context: Res<RapierContext>,
    query: Query<(Entity, &LevelExit, &Collider), Without<Player>>,
    mut player_query: Query<(Entity, &Collider), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (player_entity, _) in player_query.iter_mut() {
        for (level_end_entity, _, _) in query.iter() {
            if rapier_context.intersection_pair(player_entity, level_end_entity) == Some(true) {
                next_state.set(GameState::MutationSelection);
            }
        }
    }
}
