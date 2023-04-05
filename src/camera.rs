use bevy::prelude::*;

use crate::player::*;

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
