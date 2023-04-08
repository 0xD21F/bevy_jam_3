use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    animation::{Animated, SpriteSheetAnimationPlugin},
    app_state::{loading::SpriteAssets, AppState},
    behaviour::BehaviourPlugin,
    camera::CameraPlugin,
    entity::{
        creature::{Creature, CreatureBundle, CreaturePlugin},
        player::{Player, PlayerBundle, PlayerPlugin},
        spawner::{EnemyType, Spawner, SpawnerPlugin},
        Enemy, EnemyPlugin, ZSort,
    },
    PIXELS_PER_METER,
};

use self::{level_manager::LevelManagerPlugin, opening_cutscene::OpeningCutscenePlugin};

pub mod level_manager;
pub mod opening_cutscene;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameState>()
            .add_plugin(OpeningCutscenePlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(SpriteSheetAnimationPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CreaturePlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(BehaviourPlugin)
            .add_plugin(SpawnerPlugin)
            .add_plugin(LevelManagerPlugin);

        app.add_system(spawn_player.in_schedule(OnEnter(GameState::InLevel)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Exited,
    OpeningCutscene,
    SetupLevelManager,
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
