use std::str::FromStr;

use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::{FieldValue, LdtkEntityAppExt, LdtkIntCellAppExt},
    EntityInstance, IntGridCell, LdtkEntity, LdtkIntCell, LdtkSettings, LdtkWorldBundle,
    LevelSelection,
};
use bevy_rapier2d::prelude::Collider;
use rand::Rng;

use crate::{
    app_state::{loading::{LevelAssets, SpriteAssets}, AppState},
    camera::{camera_clamp_to_current_level, camera_movement_system},
    entity::{
        spawner::{spawn_system, EnemyType, Spawner},
        Enemy,
    },
    PIXELS_PER_METER,
};

use super::GameState;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct NonPassable {
    #[from_int_grid_cell]
    #[bundle]
    pub wall_collider: WallColliderBundle,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallColliderBundle {
    pub collider: Collider,
}

impl From<IntGridCell> for WallColliderBundle {
    fn from(_int_grid_cell: IntGridCell) -> WallColliderBundle {
        WallColliderBundle {
            collider: Collider::cuboid(PIXELS_PER_METER * 1.0, PIXELS_PER_METER * 1.0),
        }
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct SpawnerEntityBundle {
    #[from_entity_instance]
    #[bundle]
    pub spawner: SpawnerBundle,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct SpawnerBundle {
    #[bundle]
    pub spawner: Spawner,
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

impl From<&EntityInstance> for SpawnerBundle {
    fn from(entity_instance: &EntityInstance) -> SpawnerBundle {
        // Helper closure to get field value and simplify error handling
        let get_field_value = |field_identifier: &str| {
            find_field_value(entity_instance, field_identifier)
                .ok_or_else(|| format!("Missing field: {}", field_identifier))
        };

        // Get field values or return default SpawnerBundle on error
        let (timer, spawn_rate, spawn_count, enemy_type_string) = match (
            get_field_value("timer"),
            get_field_value("spawn_rate"),
            get_field_value("spawn_count"),
            get_field_value("enemy_type"),
        ) {
            (
                Ok(FieldValue::Float(timer)),
                Ok(FieldValue::Int(spawn_rate)),
                Ok(FieldValue::Int(spawn_count)),
                Ok(FieldValue::Enum(enemy_type)),
            ) => (
                Timer::from_seconds(timer.unwrap(), TimerMode::Repeating),
                spawn_rate.unwrap() as usize,
                spawn_count.unwrap() as usize,
                enemy_type.as_ref().unwrap().as_str(),
            ),
            _ => {
                // Default spawner if something messed up. Spawn 10 slimes.
                return SpawnerBundle {
                    spawner: Spawner {
                        timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                        spawn_rate: 1,
                        spawn_count: 10,
                        enemy_type: EnemyType::Slimer,
                    },
                };
            }
        };

        SpawnerBundle {
            spawner: Spawner {
                timer,
                spawn_rate,
                spawn_count,
                enemy_type: EnemyType::from_str(enemy_type_string).unwrap(),
            },
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct LevelObject; // Marker component for entities related to the current level

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(LdtkSettings { ..default() })
            .register_ldtk_int_cell::<NonPassable>(1)
            .register_ldtk_entity::<SpawnerEntityBundle>("Spawner");

        app.add_system(level_manager_setup.in_schedule(OnEnter(GameState::SetupLevelManager)))
            .add_system(level_manager_cleanup.in_schedule(OnExit(AppState::InGame)))
            .add_system(level_setup.in_schedule(OnEnter(GameState::SetupLevel)))
            .add_system(level_cleanup.in_schedule(OnExit(GameState::LevelComplete)))
            .add_system(
                level_enemies_remaining_check
                    .in_set(OnUpdate(GameState::InLevel))
                    .before(spawn_system),
            )
            .add_system(
                camera_clamp_to_current_level
                    .in_set(OnUpdate(GameState::InLevel))
                    .after(camera_movement_system),
            )
            .add_system(
                camera_clamp_to_current_level
                    .in_set(OnUpdate(GameState::LevelComplete))
                    .after(camera_movement_system),
            )
            .add_system(portal_sprite.in_set(OnUpdate(GameState::InLevel)));
    }
}

pub fn level_enemies_remaining_check(
    mut next_state: ResMut<NextState<GameState>>,
    spawner_query: Query<&Spawner>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    // Calculate the total number of enemies remaining
    let remaining_spawns: u32 = spawner_query
        .iter()
        .map(|spawner| spawner.spawn_count as u32)
        .sum();
    let spawned_enemies: u32 = enemy_query.iter().count() as u32;
    let total_enemies = remaining_spawns + spawned_enemies;

    println!("Total enemies: {}", total_enemies);

    // Check if all enemies are killed and the level is complete
    if total_enemies == 0 {
        next_state.set(GameState::LevelComplete);
    }
}

#[derive(Resource, Reflect, Default)]
pub struct LevelManager {
    pub current_level: usize,
    pub remaining_enemies: usize,
}

pub fn level_manager_setup(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.insert_resource(LevelManager {
        current_level: 0,
        remaining_enemies: 0,
    });
    next_state.set(GameState::SetupLevel);
}

pub fn level_manager_cleanup(mut commands: Commands) {
    commands.remove_resource::<LevelManager>();
}

pub fn level_setup(
    mut commands: Commands,
    _state: ResMut<State<GameState>>,
    mut level_manager: ResMut<LevelManager>,
    level_assets: Res<LevelAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Increment the current_level
    commands.insert_resource(LevelSelection::Index(level_manager.current_level));
    level_manager.current_level += 1;

    // Spawn the level
    commands
        .spawn(LdtkWorldBundle {
            ldtk_handle: level_assets.ldtk.clone(),
            // Seems like the foreground layer spawns at a slightly positive Z-level, making it invisible to the default 2d camera.
            // Forcing it to be a negative Z-level fixes this.
            transform: Transform::from_xyz(0.0, 0.0, -900.0),
            ..Default::default()
        })
        .insert(Name::new("Level"))
        .insert(LevelObject);

    next_state.set(GameState::InLevel);
}

pub fn level_cleanup(mut commands: Commands, query: Query<Entity, &LevelObject>) {
    // Remove all entities related to the current level
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Add a portal spritebundle to entities with the Spawner component

pub fn portal_sprite(
    mut commands: Commands,
    query: Query<(Entity, &Spawner), Without<Sprite>>,
    sprite_assets: Res<SpriteAssets>,
) {
    for (entity, spawner) in query.iter() {

    // Randomly choose portal sprite between 1 and 2
    let portal_sprite = match rand::thread_rng().gen_range(0..2) {
        0 => sprite_assets.portal1.clone(),
        1 => sprite_assets.portal2.clone(),
        _ => sprite_assets.portal1.clone(),
    };
    commands
        .entity(entity)
        .insert(
            (
                Sprite {
                    ..default()
                },
                portal_sprite,
                Visibility::default(),
                ComputedVisibility::default(),
            )
        );

    }
}