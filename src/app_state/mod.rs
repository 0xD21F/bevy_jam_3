use bevy::prelude::*;

use self::{
    game::GameStatePlugin, loading::LoadingPlugin, main_menu::MainMenuPlugin,
    opening_cutscene::OpeningCutscenePlugin,
};

pub mod game;
pub mod loading;
pub mod main_menu;
pub mod opening_cutscene;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    OpeningCutscene,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<AppState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(OpeningCutscenePlugin)
            .add_plugin(GameStatePlugin);
    }
}
