use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use rand::Rng;

use crate::{
    animation::Animated,
    behaviour::{approach_and_keep_distance::ApproachAndKeepDistance, separation::Separation},
    PIXELS_PER_METER,
};

use super::{
    creature::{Creature, CreatureBundle},
    skuller::{Skuller, SkullerBundle},
    Enemy,
};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_system);
    }
}

// Whenever the timer finishes, spawn_rate entities will be spawned until spawn_count is reached
#[derive(Component, Reflect, Default)]
pub struct Spawner {
    pub timer: Timer,
    pub spawn_rate: usize,
    pub spawn_count: usize,
}

fn spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut Spawner, &Transform)>,
) {
    for (entity, mut spawner, transform) in &mut query.iter_mut() {
        // If there's nothing left to spawn, destroy the spawner
        if spawner.spawn_count == 0 {
            commands.entity(entity).despawn();
            return;
        }

        spawner.timer.tick(time.delta());
        if spawner.timer.just_finished() {
            // If the spawn rate is greater than the number of entities left to spawn, set the spawn rate to the number of entities left to spawn
            if (spawner.spawn_rate > spawner.spawn_count) {
                spawner.spawn_rate = spawner.spawn_count;
            }

            for _ in 0..spawner.spawn_rate as usize {
                // Spawn the entity
                let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
                    asset_server.load("sprites/skuller.png"),
                    Vec2::new(32.0, 32.0),
                    5,
                    1,
                    None,
                    None,
                ));
                let animation = Animated {
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    first: 0,
                    last: 4,
                    ..default()
                };
                let sprite_size = PIXELS_PER_METER;

                let _enemy_entity = commands
                    .spawn(
                        SkullerBundle::new(
                            texture_atlas_handle,
                            sprite_size,
                            animation,
                            transform.clone()
                        ));

                // Decrement the number of entities left to spawn
                spawner.spawn_count -= 1;
            }
        }
    }
}
