use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;
use seldom_state::prelude::{NotTrigger, StateMachine};

use crate::{
    animation::Animated,
    app_state::{loading::SpriteAssets, AppState},
    behaviour::{
        states::{approach_and_keep_distance::ApproachAndKeepDistance, wander::Wander, idle::Idle},
        triggers::Near,
    },
    PIXELS_PER_METER,
};

use super::{creature::DontSetFacing, skuller::SkullerBundle, player::Player};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_system.in_set(OnUpdate(AppState::InGame)));
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
    sprites: Res<SpriteAssets>,
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut Spawner, &Transform)>,
    player_query: Query<Entity, With<Player>>,
) {

    let player = player_query.single();

    for (entity, mut spawner, transform) in &mut query.iter_mut() {
        // If there's nothing left to spawn, destroy the spawner
        if spawner.spawn_count == 0 {
            commands.entity(entity).despawn();
            return;
        }

        spawner.timer.tick(time.delta());
        if spawner.timer.just_finished() {
            // If the spawn rate is greater than the number of entities left to spawn, set the spawn rate to the number of entities left to spawn
            if spawner.spawn_rate > spawner.spawn_count {
                spawner.spawn_rate = spawner.spawn_count;
            }

            for _ in 0..spawner.spawn_rate {
                // Spawn the entity
                let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
                    sprites.skuller.clone(),
                    Vec2::new(32.0, 32.0),
                    8,
                    1,
                    None,
                    None,
                ));
                let sprite_size = PIXELS_PER_METER;
                let mut rng = rand::thread_rng();

                let mut timer = Timer::from_seconds(0.15, TimerMode::Repeating);
                timer.tick(Duration::from_millis(rng.gen_range(0..=150)));

                let _enemy_entity = commands
                    .spawn(SkullerBundle::new(
                        texture_atlas_handle,
                        sprite_size,
                        Animated {
                            timer,
                            first: 0,
                            last: 7,
                            ..default()
                        },
                        *transform,
                    ))
                    .insert(DontSetFacing)
                    .insert(
                        // This state machine handles the enemy's transitions
                        // The initial state is `Idle`
                        StateMachine::new(Idle)
                            .trans::<Idle>(
                                Near {
                                    target: player,
                                    range: PIXELS_PER_METER * 20.0,
                                },
                                ApproachAndKeepDistance {
                                    target: player,
                                    inner_distance: PIXELS_PER_METER * 4.0,
                                    outer_distance: PIXELS_PER_METER * 6.0,
                                },
                            )
                            .trans::<Idle>(
                                NotTrigger(Near {
                                    target: player,
                                    range: PIXELS_PER_METER * 20.0,
                                }),
                                Wander::default(),
                            )
                            .trans::<Wander>(
                                Near {
                                    target: player,
                                    range: PIXELS_PER_METER * 20.0,
                                },
                                ApproachAndKeepDistance {
                                    target: player,
                                    inner_distance: PIXELS_PER_METER * 4.0,
                                    outer_distance: PIXELS_PER_METER * 6.0,
                                },
                            )
                            .trans::<ApproachAndKeepDistance>(
                                NotTrigger(Near {
                                    target: player,
                                    range: 500.,
                                }),
                                Idle,
                            ),
                    );

                // Decrement the number of entities left to spawn
                spawner.spawn_count -= 1;
            }
        }
    }
}
