use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};

use crate::{game::level_manager::LevelManager, player::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_movement_system)
            .add_system(spawn_camera.on_startup());
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.5, 0.5, 1.0),
            ..default()
        },
        ..default()
    });
}

pub fn camera_movement_system(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player_transform in player_query.iter() {
        for mut camera_transform in &mut camera_query.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

pub fn camera_clamp_to_current_level(
    mut camera_query: Query<(&OrthographicProjection, &mut Transform), Without<Player>>,
    level_query: Query<
        (&Transform, &Handle<LdtkLevel>),
        (Without<OrthographicProjection>, Without<Player>),
    >,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    let (orthographic_projection, mut camera_transform) = camera_query.single_mut();

    for (level_transform, level_handle) in level_query.iter() {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level = &ldtk_level.level;
            let half_width =
                (orthographic_projection.area.width() * camera_transform.scale.x) / 2.0;
            let half_height =
                (orthographic_projection.area.height() * camera_transform.scale.y) / 2.0;

            let level_min = Vec2::new(
                level_transform.translation.x + half_width,
                level_transform.translation.y + half_height,
            );
            let level_max = Vec2::new(
                level_transform.translation.x + level.px_wid as f32 - half_width,
                level_transform.translation.y + level.px_hei as f32 - half_height,
            );

            // Clamp the camera's position independently for each axis
            if level_min.x <= level_max.x {
                camera_transform.translation.x = camera_transform
                    .translation
                    .x
                    .clamp(level_min.x, level_max.x);
            } else {
                // Set the camera's x position to the center of the level
                camera_transform.translation.x =
                    level_transform.translation.x + level.px_wid as f32 / 2.0;
            }

            if level_min.y <= level_max.y {
                camera_transform.translation.y = camera_transform
                    .translation
                    .y
                    .clamp(level_min.y, level_max.y);
            } else {
                // Set the camera's y position to the center of the level
                camera_transform.translation.y =
                    level_transform.translation.y + level.px_hei as f32 / 2.0;
            }
        }
    }
}
