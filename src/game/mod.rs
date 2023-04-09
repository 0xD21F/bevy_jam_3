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
