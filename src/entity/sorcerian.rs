use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_rapier2d::prelude::{Collider, Sensor};
use rand::Rng;

use crate::{
    animation::Animated,
    app_state::loading::{SfxAssets, SoundEffects},
    behaviour::separation::{separation_system, Separation},
    game::{level_manager::SpawnerBundle, GameState},
    PIXELS_PER_METER,
};

use super::{
    adept,
    creature::{Creature, CreatureBundle, Hitbox, Velocity},
    spawner::{EnemyType, Spawner},
    Enemy, EnemyHurtboxDamage, ZSort,
};

pub struct SorcerianPlugin;

impl Plugin for SorcerianPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(separation_system::<Sorcerian>)
            .add_system(do_shit.in_set(OnUpdate(GameState::InLevel)));
    }
}

#[derive(Component, Reflect, Default)]
pub struct Sorcerian {
    spawn_timer: Timer,
}

#[derive(Bundle)]
pub struct SorcerianBundle {
    pub creature: CreatureBundle,
    pub enemy: Enemy,
    pub name: Name,
    pub sorcerian: Sorcerian,
    pub hurtbox: EnemyHurtboxDamage,
}

impl SorcerianBundle {
    pub fn new(
        texture_atlas_handle: Handle<TextureAtlas>,
        sprite_size: f32,
        animation: Animated,
        transform: Transform,
    ) -> Self {
        Self {
            creature: CreatureBundle {
                creature: Creature {
                    acceleration: 1000.0,
                    friction: 250.0,
                    max_speed: 300.0,
                    health: 500.0,
                    max_health: 500.0,
                    damage_invulnerability: Timer::from_seconds(0.5, TimerMode::Once),
                },
                animation: animation.clone(),
                sprite: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation.first),
                    transform,
                    ..default()
                },
                collider: Collider::ball(sprite_size / 2.0),
                velocity: Velocity::default(),
                zsort: ZSort {
                    offset_y: -(sprite_size / 2.0),
                },
                sensor: Sensor,
                hitbox: Hitbox,
            },
            hurtbox: EnemyHurtboxDamage(12),
            enemy: Enemy,
            sorcerian: Sorcerian {
                spawn_timer: Timer::from_seconds(5.0, TimerMode::Once),
            },
            name: Name::new("Sorcerian"),
        }
    }
}

pub fn do_shit(
    mut commands: Commands,
    mut sorcerian_query: Query<(Entity, &Transform, &mut Velocity, &mut Sorcerian)>,
    time: Res<Time>,
    sfx: Res<AudioChannel<SoundEffects>>,
    music_assets: Res<SfxAssets>,
) {
    for (sorcerian_query, adept_transform, mut velocity, mut sorcerian) in
        sorcerian_query.iter_mut()
    {
        sorcerian.spawn_timer.tick(time.delta());
        if sorcerian.spawn_timer.finished() {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(1..8);

            // Play random laff
            let fiftyfifty = rng.gen_range(0..1);
            if (fiftyfifty == 0) {
                sfx.play(music_assets.laff1.clone());
            } else {
                sfx.play(music_assets.laff2.clone());
            }

            // Sometimes dash away
            if (fiftyfifty == 0) {
                let random_vec2 = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
                velocity.value += random_vec2 * 1500.0;
            }

            let enemy_type = match random_number {
                1 => EnemyType::Adept,
                2 => EnemyType::Goblin,
                3 => EnemyType::Goblin,
                4 => EnemyType::Mutant,
                5 => EnemyType::Skuller,
                6 => EnemyType::Slimer,
                7 => EnemyType::Slimer,
                _ => EnemyType::Skuller,
            };

            commands.spawn((
                SpawnerBundle {
                    spawner: Spawner {
                        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                        spawn_rate: random_number,
                        spawn_count: random_number,
                        enemy_type: enemy_type,
                    },
                },
                adept_transform.clone(),
                GlobalTransform::default(),
            ));
            sorcerian.spawn_timer.reset()
        }
    }
}
