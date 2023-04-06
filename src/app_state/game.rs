use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    animation::{Animated, SpriteSheetAnimationPlugin},
    behaviour::BehaviourPlugin,
    camera::CameraPlugin,
    entity::{
        creature::{Creature, CreatureBundle, CreaturePlugin},
        player::{Player, PlayerBundle, PlayerPlugin},
        spawner::{Spawner, SpawnerPlugin},
        Enemy, EnemyPlugin, ZSort,
    },
    level::{build_level, LevelElement, LevelElementDefinition},
    PIXELS_PER_METER,
};

use super::{loading::SpriteAssets, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(CameraPlugin)
            .add_plugin(SpriteSheetAnimationPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CreaturePlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(BehaviourPlugin)
            .add_plugin(SpawnerPlugin);

        app.add_system(setup_level.in_schedule(OnEnter(AppState::InGame)))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(spawn_spawner.in_schedule(OnEnter(AppState::InGame)))
            .add_system(game_cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}

pub fn setup_level(mut commands: Commands, sprites: Res<SpriteAssets>) {
    let level_bounds_half_extents = 50.0 * PIXELS_PER_METER;

    let level_definition = [
        // Bottom of the level
        LevelElementDefinition {
            position: Vec2::new(0.0, -level_bounds_half_extents),
            size: Vec2::new(level_bounds_half_extents * 2.0, 1.0 * PIXELS_PER_METER),
            ..default()
        },
        // Top of level
        LevelElementDefinition {
            position: Vec2::new(0.0, level_bounds_half_extents),
            size: Vec2::new(level_bounds_half_extents * 2.0, 1.0 * PIXELS_PER_METER),
            ..default()
        },
        // Left of level
        LevelElementDefinition {
            position: Vec2::new(-level_bounds_half_extents, 0.0),
            size: Vec2::new(1.0 * PIXELS_PER_METER, level_bounds_half_extents * 2.0),
            ..default()
        },
        // Right of level
        LevelElementDefinition {
            position: Vec2::new(level_bounds_half_extents, 0.0),
            size: Vec2::new(1.0 * PIXELS_PER_METER, level_bounds_half_extents * 2.0),
            ..default()
        },
    ];

    build_level(&mut commands, &sprites, &level_definition);
}

pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_size = PIXELS_PER_METER * 2.0;

    let texture_handle = sprites.player.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation = Animated {
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        first: 0,
        last: 0,
        ..default()
    };

    let _player_entity = commands.spawn(PlayerBundle {
        unit_bundle: CreatureBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation.first),
                ..default()
            },
            animation,
            collider: Collider::ball(sprite_size / 3.0),
            zsort: ZSort {
                offset_y: -(sprite_size / 2.0 - 20.0),
            },
            creature: Creature {
                max_speed: 512.0,
                acceleration: 512.0,
                deceleration: 512.0,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

pub fn spawn_spawner(mut commands: Commands, sprites: Res<SpriteAssets>) {
    let _spawner_entity = commands
        .spawn((
            Spawner {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                spawn_rate: 5,
                spawn_count: 2500,
            },
            SpriteBundle {
                texture: sprites.sorcerian.clone(),
                ..default()
            },
        ))
        .insert(Name::new("Spawner"))
        .insert(ZSort::default());
}

pub fn game_cleanup(
    mut commands: Commands,
    spawner_query: Query<Entity, With<Spawner>>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    level_query: Query<Entity, With<LevelElement>>,
) {
    for entity in spawner_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in level_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
