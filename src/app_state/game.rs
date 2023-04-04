use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    animation::Animated,
    entity::{creature::CreatureBundle, player::PlayerBundle, spawner::Spawner},
    level::{build_level, LevelElementDefinition},
    PIXELS_PER_METER,
};

use super::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(setup_level.in_schedule(OnEnter(AppState::InGame)))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(spawn_spawner.in_schedule(OnEnter(AppState::InGame)));
    }
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    build_level(&mut commands, &asset_server, &level_definition);
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_size = PIXELS_PER_METER * 2.0;

    let texture_handle = asset_server.load("sprites/ape.png");
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
            collider: Collider::cuboid(sprite_size / 2.0, sprite_size / 2.0),
            ..default()
        },
        ..default()
    });
}

pub fn spawn_spawner(mut commands: Commands) {
    let _spawner_entity = commands
        .spawn((
            Spawner {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                spawn_rate: 4,
                spawn_count: 100,
                ..default()
            },
            Transform::default(),
            GlobalTransform::default(),
        ))
        .insert(Name::new("Spawner"));
}
