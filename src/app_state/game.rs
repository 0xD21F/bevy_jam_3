use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

use crate::{
    animation::{Animated, SpriteSheetAnimationPlugin},
    behaviour::BehaviourPlugin,
    camera::CameraPlugin,
    entity::{
        creature::{Creature, CreatureBundle, CreaturePlugin},
        player::{Player, PlayerBundle, PlayerPlugin},
        spawner::{EnemyType, Spawner, SpawnerPlugin},
        Enemy, EnemyPlugin, ZSort,
    },
    game::{ GamePlugin, GameState},
    level::{build_level, LevelElement, LevelElementDefinition},
    PIXELS_PER_METER,
};

use super::{loading::SpriteAssets, AppState};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugin(GamePlugin);
        //     .add_plugin(CameraPlugin)
        //     .add_plugin(SpriteSheetAnimationPlugin)
        //     .add_plugin(PlayerPlugin)
        //     .add_plugin(CreaturePlugin)
        //     .add_plugin(EnemyPlugin)
        //     .add_plugin(BehaviourPlugin)
        //     .add_plugin(SpawnerPlugin)
        //     .add_plugin(LevelManagerPlugin);

        // app
        //     .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
        //     .add_system(spawn_spawner.in_schedule(OnEnter(AppState::InGame)))
        //     .add_system(game_cleanup.in_schedule(OnExit(AppState::InGame)));
    }
}
