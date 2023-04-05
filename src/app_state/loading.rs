use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};

use super::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, SpriteAssets>(AppState::Loading)
        .add_system(loading_setup.in_schedule(OnEnter(AppState::Loading)))
        .add_system(loading_cleanup.in_schedule(OnExit(AppState::Loading)));
    }
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "sprites/ape.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/sorcerian.png")]
    pub sorcerian: Handle<Image>,
    #[asset(path = "sprites/skuller.png")]
    pub skuller: Handle<Image>,
}

#[derive(Resource)]
pub struct LoadingUiData {
    pub node: Entity,
}

pub fn loading_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let node = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();

    commands.insert_resource(LoadingUiData { node });
}

pub fn loading_cleanup(mut commands: Commands, loading_ui_data: Res<LoadingUiData>) {
    commands.entity(loading_ui_data.node).despawn_recursive();
}
