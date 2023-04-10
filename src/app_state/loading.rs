use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_ecs_ldtk::LdtkAsset;
use bevy_kira_audio::{AudioApp, AudioSource};

use super::AppState;

pub struct LoadingPlugin;

// Our type for the custom audio channel
#[derive(Resource)]
pub struct Background;

// Our type for the custom audio channel
#[derive(Resource)]
pub struct SoundEffects;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, SpriteAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, CutsceneAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, LevelAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, MusicAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, SfxAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, UiAssets>(AppState::Loading)
        .add_audio_channel::<Background>()
        .add_audio_channel::<SoundEffects>()
        .add_system(loading_setup.in_schedule(OnEnter(AppState::Loading)))
        .add_system(loading_cleanup.in_schedule(OnExit(AppState::Loading)));
    }
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "sprites/ape.png")]
    pub player: Handle<Image>,
    #[asset(path = "sprites/ape_rage.png")]
    pub player_rage: Handle<Image>,
    #[asset(path = "sprites/sorcerian.png")]
    pub sorcerian: Handle<Image>,
    #[asset(path = "sprites/skuller.png")]
    pub skuller: Handle<Image>,
    #[asset(path = "sprites/slimer.png")]
    pub slimer: Handle<Image>,
    #[asset(path = "sprites/mutant.png")]
    pub mutant: Handle<Image>,
    #[asset(path = "sprites/goblin.png")]
    pub goblin: Handle<Image>,
    #[asset(path = "sprites/adept.png")]
    pub adept: Handle<Image>,
    #[asset(path = "sprites/lab_boss.png")]
    pub lab_boss: Handle<Image>,
    #[asset(path = "sprites/portal1.png")]
    pub portal1: Handle<Image>,
    #[asset(path = "sprites/portal2.png")]
    pub portal2: Handle<Image>,
    #[asset(path = "sprites/dryskin.png")]
    pub dryskin: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct CutsceneAssets {
    #[asset(path = "cutscenes/opening1.png")]
    pub opening1: Handle<Image>,
    #[asset(path = "cutscenes/opening2.png")]
    pub opening2: Handle<Image>,
    #[asset(path = "cutscenes/opening3.png")]
    pub opening3: Handle<Image>,
    #[asset(path = "cutscenes/ending1.png")]
    pub ending1: Handle<Image>,
    #[asset(path = "cutscenes/ending2.png")]
    pub ending2: Handle<Image>,
    #[asset(path = "cutscenes/ending3.png")]
    pub ending3: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/mutating.png")]
    pub mutating: Handle<Image>,
    #[asset(path = "ui/title.png")]
    pub title: Handle<Image>,
    #[asset(path = "ui/space.png")]
    pub space: Handle<Image>,
    #[asset(path = "ui/mutation_icons.png")]
    pub mutation_icons: Handle<Image>,
    #[asset(path = "ui/portrait.png")]
    pub portrait: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "levels/tiles/tiles.png")]
    pub tiles: Handle<Image>,
    #[asset(path = "levels/ldtk/levels.ldtk")]
    pub ldtk: Handle<LdtkAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct MusicAssets {
    #[asset(path = "music/title.ogg")]
    pub title: Handle<AudioSource>,
    #[asset(path = "music/crystal.ogg")]
    pub crystal: Handle<AudioSource>,
    #[asset(path = "music/ending.ogg")]
    pub ending: Handle<AudioSource>,
    #[asset(path = "music/labs.ogg")]
    pub labs: Handle<AudioSource>,
    #[asset(path = "music/labsboss.ogg")]
    pub labsboss: Handle<AudioSource>,
    #[asset(path = "music/mutate.ogg")]
    pub mutate: Handle<AudioSource>,
    #[asset(path = "music/sorcerianboss.ogg")]
    pub sorcerianboss: Handle<AudioSource>,
    #[asset(path = "music/tower.ogg")]
    pub tower: Handle<AudioSource>,
    #[asset(path = "music/towerboss.ogg")]
    pub towerboss: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SfxAssets {
    #[asset(path = "sfx/hit.ogg")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "sfx/laff1.ogg")]
    pub laff1: Handle<AudioSource>,
    #[asset(path = "sfx/laff2.ogg")]
    pub laff2: Handle<AudioSource>,
    #[asset(path = "sfx/lariat.ogg")]
    pub lariat: Handle<AudioSource>,
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
