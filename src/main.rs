mod camera;
mod debug;
mod enemies;
mod enemy;
mod level;
mod player;
mod unit;

use camera::*;
use debug::*;
use enemy::*;
use level::*;
use player::*;
use unit::*;

use bevy::{prelude::*, window::*};

use bevy_rapier2d::prelude::*;
use unit::Velocity;

pub const PIXELS_PER_METER: f32 = 32.0;

pub const PLAYER_LAYER: Group = Group::GROUP_1;
pub const LEVEL_LAYER: Group = Group::GROUP_2;
pub const ENEMY_LAYER: Group = Group::GROUP_3;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1024., 768.),
                        title: "Ape Effect".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(DebugPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(UnitPlugin)
        .add_system(setup.on_startup())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .register_type::<Unit>()
        .register_type::<Velocity>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
