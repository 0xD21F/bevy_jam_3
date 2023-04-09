use bevy::prelude::*;

use crate::{
    animation::SpriteSheetAnimationPlugin,
    behaviour::BehaviourPlugin,
    camera::CameraPlugin,
    entity::{creature::CreaturePlugin, player::PlayerPlugin, spawner::SpawnerPlugin, EnemyPlugin},
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
