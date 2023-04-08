use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{camera::CameraPlugin, animation::{SpriteSheetAnimationPlugin, Animated}, entity::{player::{PlayerPlugin, PlayerBundle, Player}, creature::{CreaturePlugin, CreatureBundle, Creature}, EnemyPlugin, spawner::{SpawnerPlugin, Spawner, EnemyType}, ZSort, Enemy}, behaviour::BehaviourPlugin, app_state::{AppState, loading::SpriteAssets}, PIXELS_PER_METER, level::LevelElement};

// use self::level_manager::LevelManagerPlugin;

pub mod level_manager;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_state::<GameState>()
            .add_plugin(CameraPlugin)
            .add_plugin(SpriteSheetAnimationPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CreaturePlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(BehaviourPlugin)
            .add_plugin(SpawnerPlugin);
            // .add_plugin(LevelManagerPlugin);

        app
            .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(spawn_spawner.in_schedule(OnEnter(AppState::InGame)))
            .add_system(game_cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    SetupLevel,
    InLevel,
    MutationSelection,
}

pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_size = PIXELS_PER_METER * 2.0;

    let texture_handle = sprites.player.clone();
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation = Animated {
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        first: 0,
        last: 0,
        ..default()
    };

    let _player_entity = commands.spawn(PlayerBundle {
        unit_bundle: CreatureBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation.first),
                ..default()
            },
            animation,
            collider: Collider::ball(sprite_size / 3.0),
            zsort: ZSort {
                offset_y: -(sprite_size / 2.0 - 20.0),
            },
            creature: Creature {
                max_speed: 256.0,
                acceleration: 2048.0,
                friction: 512.0,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

pub fn spawn_spawner(mut commands: Commands, sprites: Res<SpriteAssets>) {
    let _spawner_entity = commands
        .spawn((
            Spawner {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                spawn_rate: 3,
                spawn_count: 3,
                enemy_type: EnemyType::Slimer,
            },
            SpriteBundle {
                texture: sprites.sorcerian.clone(),
                ..default()
            },
        ))
        .insert(Name::new("Spawner"))
        .insert(ZSort::default());

    let _spawner_entity = commands
        .spawn((
            Spawner {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                spawn_rate: 3,
                spawn_count: 3,
                enemy_type: EnemyType::Mutant,
            },
            SpriteBundle {
                texture: sprites.sorcerian.clone(),
                ..default()
            },
        ))
        .insert(Name::new("Spawner"))
        .insert(ZSort::default());

    let _spawner_entity = commands
        .spawn((
            Spawner {
                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                spawn_rate: 3,
                spawn_count: 3,
                enemy_type: EnemyType::Skuller,
            },
            SpriteBundle {
                texture: sprites.sorcerian.clone(),
                ..default()
            },
        ))
        .insert(Name::new("Spawner"))
        .insert(ZSort::default());
}

pub fn game_cleanup(
    mut commands: Commands,
    spawner_query: Query<Entity, With<Spawner>>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for entity in spawner_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
