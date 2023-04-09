mod animation;
mod app_state;
mod behaviour;
mod camera;
mod debug;
mod entity;
mod game;

use app_state::*;

use bevy_ecs_ldtk::LdtkPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_mod_ui_texture_atlas_image::UiAtlasImagePlugin;
use debug::*;
use entity::*;

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
                        resolution: WindowResolution::new(1920., 1080.),
                        title: "Mutape".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(DebugPlugin)
        .add_plugin(LdtkPlugin)
        .add_plugin(UiAtlasImagePlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(AppStatePlugin)
        .run();
}
