mod animation;
mod app_state;
mod behaviour;
mod camera;
mod creature_state;
mod debug;
mod entity;
mod level;

use animation::*;
use app_state::{game::GamePlugin, main_menu::MainMenuPlugin, *};
use camera::*;
use debug::*;
use entity::{creature::CreaturePlugin, player::PlayerPlugin, spawner::SpawnerPlugin, *};

use bevy::{prelude::*, window::*};

use bevy_rapier2d::prelude::*;

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
        .add_plugin(AppStatePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .run();
}
