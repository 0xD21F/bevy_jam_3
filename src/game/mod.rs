use bevy::prelude::*;

use crate::{
    animation::SpriteSheetAnimationPlugin,
    behaviour::BehaviourPlugin,
    camera::CameraPlugin,
    entity::{creature::CreaturePlugin, player::PlayerPlugin, spawner::SpawnerPlugin, EnemyPlugin},
};

use self::{
    level_manager::LevelManagerPlugin, mutation_manager::MutationManagerPlugin,
    mutation_selection::MutationSelectionPlugin, opening_cutscene::OpeningCutscenePlugin,
    ui::UiPlugin,
};

pub mod level_manager;
pub mod mutation_manager;
pub mod mutation_selection;
pub mod opening_cutscene;
pub mod ui;

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
            .add_plugin(LevelManagerPlugin)
            .add_plugin(MutationManagerPlugin)
            .add_plugin(MutationSelectionPlugin)
            .add_plugin(UiPlugin);
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
    LevelComplete,
    MutationSelection,
}
