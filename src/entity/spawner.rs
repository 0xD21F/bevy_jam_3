use std::{str::FromStr, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::prelude::ActiveCollisionTypes;

use rand::Rng;
use seldom_state::prelude::{NotTrigger, StateMachine};

use crate::{
    animation::Animated,
    app_state::{loading::SpriteAssets, AppState},
    behaviour::{
        states::{
            approach_and_keep_distance::ApproachAndKeepDistance,
            attack::{AttackAndKeepDistance, LabBossAttack},
            fire_projectile::{FireProjectile, FireProjectileAndKeepDistance},
            idle::Idle,
            wander::Wander,
        },
        triggers::Near,
    },
    PIXELS_PER_METER,
};

use super::adept::AdeptBundle;
use super::creature::FacePlayer;
use super::goblin::GoblinBundle;
use super::lab_boss::LabBossBundle;
use super::sorcerian::SorcerianBundle;
use super::{
    creature::DontSetFacing, mutant::MutantBundle, player::Player, skuller::SkullerBundle,
    slimer::SlimerBundle,
};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_system.in_set(OnUpdate(AppState::InGame)));
    }
}

// Whenever the timer finishes, spawn_rate entities will be spawned until spawn_count is reached
#[derive(Component, Reflect, Default, FromReflect, Clone, Debug)]
pub struct Spawner {
    pub timer: Timer,
    pub spawn_rate: usize,
    pub spawn_count: usize,
    pub enemy_type: EnemyType,
}

#[derive(Reflect, Default, FromReflect, Clone, Debug)]
pub enum EnemyType {
    #[default]
    Slimer,
    Mutant,
    Goblin,
    GoblinBrute,
    Adept,
    Skuller,
    LabBoss,
    Sorcerian,
}

impl FromStr for EnemyType {
    type Err = ();

    fn from_str(input: &str) -> Result<EnemyType, Self::Err> {
        match input {
            "Slimer" => Ok(EnemyType::Slimer),
            "Mutant" => Ok(EnemyType::Mutant),
            "Goblin" => Ok(EnemyType::Goblin),
            "GoblinBrute" => Ok(EnemyType::GoblinBrute),
            "Adept" => Ok(EnemyType::Adept),
            "Skuller" => Ok(EnemyType::Skuller),
            "LabBoss" => Ok(EnemyType::LabBoss),
            "Sorcerian" => Ok(EnemyType::Sorcerian),
            _ => Err(()),
        }
    }
}

pub fn spawn_system(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    time: Res<Time>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(Entity, &mut Spawner, &Transform)>,
    player_query: Query<Entity, With<Player>>,
) {
    let player = player_query.get_single();
    if let Ok(player) = player {
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
                    match spawner.enemy_type {
                        EnemyType::Slimer => spawn_slimer(
                            &mut commands,
                            &sprites,
                            &mut texture_atlases,
                            *transform,
                            player,
                        ),
                        EnemyType::Mutant => spawn_mutant(
                            &mut commands,
                            &sprites,
                            &mut texture_atlases,
                            *transform,
                            player,
                        ),
                        EnemyType::Goblin => spawn_goblin(
                            &mut commands,
                            &sprites,
                            &mut texture_atlases,
                            *transform,
                            player,
                        ),
                        EnemyType::GoblinBrute => spawn_goblin_brute(
                            &mut commands,
                            &sprites,
                            &mut texture_atlases,
                            *transform,
                            player,
                        ),
                        EnemyType::Adept => spawn_adept(
                            &mut commands,
                            &sprites,
                            &mut texture_atlases,
                            *transform,
                            player,
                        ),
                        EnemyType::Skuller => spawn_skuller(
                            &mut commands,
                            &sprites,
                            &mut texture_atlases,
                            *transform,
                            player,
                        ),
                        EnemyType::LabBoss => spawn_lab_boss(
                            &mut commands,
                            &sprites,
                            &mut texture_atlases,
                            *transform,
                            player,
                        ),
                        EnemyType::Sorcerian => spawn_sorcerian(
                            &mut commands,
                            &sprites,
                            &mut texture_atlases,
                            *transform,
                            player,
                        ),
                    }

                    // Decrement the number of entities left to spawn
                    spawner.spawn_count -= 1;
                }
            }
        }
    }
}

fn spawn_skuller(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
    target: Entity,
) {
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
            transform,
        ))
        .insert(DontSetFacing)
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(
            // This state machine handles the enemy's transitions
            // The initial state is `Idle`
            StateMachine::new(Idle)
                .trans::<Idle>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 1.0,
                        outer_distance: PIXELS_PER_METER * 8.0,
                    },
                )
                .trans::<Idle>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Wander::default(),
                )
                .trans::<Wander>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 1.0,
                        outer_distance: PIXELS_PER_METER * 8.0,
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Idle,
                ),
        );
}

fn spawn_slimer(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
    target: Entity,
) {
    // Spawn the entity
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        sprites.slimer.clone(),
        Vec2::new(32.0, 32.0),
        2,
        1,
        None,
        None,
    ));
    let sprite_size = PIXELS_PER_METER;
    let mut rng = rand::thread_rng();

    let mut timer = Timer::from_seconds(0.15, TimerMode::Repeating);
    timer.tick(Duration::from_millis(rng.gen_range(0..=150)));

    let _enemy_entity = commands
        .spawn(SlimerBundle::new(
            texture_atlas_handle,
            sprite_size,
            Animated {
                timer,
                first: 0,
                last: 1,
                ..default()
            },
            transform,
        ))
        .insert(DontSetFacing)
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(
            // This state machine handles the enemy's transitions
            // The initial state is `Idle`
            StateMachine::new(Idle)
                .trans::<Idle>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 4.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                )
                .trans::<Idle>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Wander::default(),
                )
                .trans::<Wander>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 4.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Idle,
                )
                .trans::<ApproachAndKeepDistance>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 4.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 2.0,
                        outer_distance: PIXELS_PER_METER * 3.0,
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 1.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 4.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                ),
        );
}

fn spawn_mutant(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
    target: Entity,
) {
    // Spawn the entity
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        sprites.mutant.clone(),
        Vec2::new(64.0, 64.0),
        4,
        1,
        None,
        None,
    ));
    let sprite_size = PIXELS_PER_METER * 2.0;
    let mut rng = rand::thread_rng();

    let mut timer = Timer::from_seconds(0.40, TimerMode::Repeating);
    timer.tick(Duration::from_millis(rng.gen_range(0..=150)));

    let _enemy_entity = commands
        .spawn(MutantBundle::new(
            texture_atlas_handle,
            sprite_size,
            Animated {
                timer,
                first: 0,
                last: 2,
                ..default()
            },
            transform,
        ))
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(FacePlayer)
        .insert(
            // This state machine handles the enemy's transitions
            // The initial state is `Idle`
            StateMachine::new(Idle)
                .trans::<Idle>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 4.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 5.0,
                    }),
                    FireProjectileAndKeepDistance {
                        fire_projectile: FireProjectile {
                            target,
                            projectile: Projectile::MutantProjectile,
                        },
                        keep_distance: ApproachAndKeepDistance {
                            target,
                            inner_distance: PIXELS_PER_METER * 4.0,
                            outer_distance: PIXELS_PER_METER * 12.0,
                        },
                    },
                )
                .trans::<FireProjectileAndKeepDistance>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 4.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 2.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                ),
        );
}

fn spawn_goblin(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
    target: Entity,
) {
    // Spawn the entity
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        sprites.goblin.clone(),
        Vec2::new(64.0, 64.0),
        3,
        1,
        None,
        None,
    ));
    let sprite_size = PIXELS_PER_METER * 2.0;
    let mut rng = rand::thread_rng();

    let mut timer = Timer::from_seconds(0.40, TimerMode::Repeating);
    timer.tick(Duration::from_millis(rng.gen_range(0..=150)));

    let _enemy_entity = commands
        .spawn(GoblinBundle::new(
            texture_atlas_handle,
            sprite_size,
            Animated {
                timer,
                first: 0,
                last: 1,
                ..default()
            },
            transform,
        ))
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(FacePlayer)
        .insert(
            // This state machine handles the enemy's transitions
            // The initial state is `Idle`
            StateMachine::new(Idle)
                .trans::<Idle>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * rng.gen_range(1.0..4.0),
                        outer_distance: PIXELS_PER_METER * rng.gen_range(4.0..8.0),
                    },
                )
                .trans::<Idle>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Wander::default(),
                )
                .trans::<Wander>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * rng.gen_range(1.0..4.0),
                        outer_distance: PIXELS_PER_METER * rng.gen_range(4.0..8.0),
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Idle,
                ),
        );
}

fn spawn_goblin_brute(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
    target: Entity,
) {
    // Spawn the entity
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        sprites.goblin.clone(),
        Vec2::new(64.0, 64.0),
        3,
        1,
        None,
        None,
    ));
    let sprite_size = PIXELS_PER_METER * 3.0;
    let mut rng = rand::thread_rng();

    let mut timer = Timer::from_seconds(0.40, TimerMode::Repeating);
    timer.tick(Duration::from_millis(rng.gen_range(0..=150)));

    let _enemy_entity = commands
        .spawn(GoblinBundle::new(
            texture_atlas_handle,
            sprite_size,
            Animated {
                timer,
                first: 0,
                last: 0,
                ..default()
            },
            transform,
        ))
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(FacePlayer)
        .insert(
            // This state machine handles the enemy's transitions
            // The initial state is `Idle`
            StateMachine::new(Idle)
                .trans::<Idle>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 4.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                )
                .trans::<Idle>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Wander::default(),
                )
                .trans::<Wander>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 4.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Idle,
                ),
        );
}

fn spawn_adept(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
    target: Entity,
) {
    // Spawn the entity
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        sprites.adept.clone(),
        Vec2::new(64.0, 64.0),
        2,
        1,
        None,
        None,
    ));
    let sprite_size = PIXELS_PER_METER * 2.0;
    let mut rng = rand::thread_rng();

    let mut timer = Timer::from_seconds(0.40, TimerMode::Repeating);
    timer.tick(Duration::from_millis(rng.gen_range(0..=150)));

    let _rng = rand::thread_rng();

    let _enemy_entity = commands
        .spawn(AdeptBundle::new(
            texture_atlas_handle,
            sprite_size,
            Animated {
                timer,
                first: 0,
                last: 1,
                ..default()
            },
            transform,
        ))
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(FacePlayer)
        .insert(
            // This state machine handles the enemy's transitions
            // The initial state is `Idle`
            StateMachine::new(Idle)
                .trans::<Idle>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 4.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                )
                .trans::<Idle>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Wander::default(),
                )
                .trans::<Wander>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 4.0,
                        outer_distance: PIXELS_PER_METER * 6.0,
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Idle,
                ),
        );
}

fn spawn_lab_boss(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
    target: Entity,
) {
    // Spawn the entity
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        sprites.lab_boss.clone(),
        Vec2::new(128.0, 128.0),
        3,
        1,
        None,
        None,
    ));
    let sprite_size = PIXELS_PER_METER * 2.0;
    let mut rng = rand::thread_rng();

    let mut timer = Timer::from_seconds(0.9, TimerMode::Repeating);
    timer.tick(Duration::from_millis(rng.gen_range(0..=150)));

    let _enemy_entity = commands
        .spawn(LabBossBundle::new(
            texture_atlas_handle,
            sprite_size,
            Animated {
                timer,
                first: 0,
                last: 1,
                ..default()
            },
            transform,
        ))
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(FacePlayer)
        .insert(
            // This state machine handles the enemy's transitions
            // The initial state is `Idle`
            StateMachine::new(Idle)
                .trans::<Idle>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 150.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 3.5,
                        outer_distance: PIXELS_PER_METER * 5.0,
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 4.0,
                    },
                    AttackAndKeepDistance {
                        approach_and_keep_distance: ApproachAndKeepDistance {
                            target,
                            inner_distance: PIXELS_PER_METER * 1.0,
                            outer_distance: PIXELS_PER_METER * 1.5,
                        },
                        attack: LabBossAttack,
                    },
                )
                .trans::<AttackAndKeepDistance>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 3.0,
                    }),
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 3.5,
                        outer_distance: PIXELS_PER_METER * 5.0,
                    },
                ),
        );
}

fn spawn_sorcerian(
    commands: &mut Commands,
    sprites: &SpriteAssets,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
    target: Entity,
) {
    // Spawn the entity
    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        sprites.sorcerian.clone(),
        Vec2::new(64.0, 64.0),
        2,
        1,
        None,
        None,
    ));
    let sprite_size = PIXELS_PER_METER * 2.0;
    let mut rng = rand::thread_rng();

    let mut timer = Timer::from_seconds(0.40, TimerMode::Repeating);
    timer.tick(Duration::from_millis(rng.gen_range(0..=150)));

    let _enemy_entity = commands
        .spawn(SorcerianBundle::new(
            texture_atlas_handle,
            sprite_size,
            Animated {
                timer,
                first: 0,
                last: 1,
                ..default()
            },
            transform,
        ))
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(FacePlayer)
        .insert(
            // This state machine handles the enemy's transitions
            // The initial state is `Idle`
            StateMachine::new(Idle)
                .trans::<Idle>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 5.0,
                        outer_distance: PIXELS_PER_METER * 8.0,
                    },
                )
                .trans::<Idle>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Wander::default(),
                )
                .trans::<Wander>(
                    Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    },
                    ApproachAndKeepDistance {
                        target,
                        inner_distance: PIXELS_PER_METER * 5.0,
                        outer_distance: PIXELS_PER_METER * 8.0,
                    },
                )
                .trans::<ApproachAndKeepDistance>(
                    NotTrigger(Near {
                        target,
                        range: PIXELS_PER_METER * 20.0,
                    }),
                    Idle,
                ),
        );
}

#[derive(Reflect, Clone, Copy)]
pub enum Projectile {
    MutantProjectile,
}
