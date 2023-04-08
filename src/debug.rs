use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::{
    animation::Animated,
    entity::{
        creature::{Creature, Velocity},
        spawner::Spawner,
    },
    game::level_manager::*,
};

#[derive(Resource, Default)]
pub struct DebugState {}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .add_plugin(RapierDebugRenderPlugin::default())
                .add_plugin(DebugLinesPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .register_type::<Creature>()
                .register_type::<Velocity>()
                .register_type::<Spawner>()
                .register_type::<Animated>()
                .register_type::<LevelManager>()
                .init_resource::<DebugState>();
        }
    }
}
