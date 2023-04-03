use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

#[derive(Resource, Default)]
pub struct DebugState {}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .add_plugin(RapierDebugRenderPlugin::default())
                .add_plugin(DebugLinesPlugin::default())
                // .add_plugin(LogDiagnosticsPlugin::default())
                // .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .init_resource::<DebugState>();
        }
    }
}
