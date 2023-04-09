use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

use super::{
    loading::{Background, MusicAssets, UiAssets},
    AppState,
};
use bevy_kira_audio::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(main_menu_setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(main_menu_music.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(main_menu_system.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(main_menu_cleanup.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(main_menu_bob_start.in_set(OnUpdate(AppState::MainMenu)));
    }
}

#[derive(Resource)]
pub struct MenuUiData {
    pub ui_entity: Entity,
}

#[derive(Default, Component)]
pub struct PressSpaceMarker;

pub fn main_menu_setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    ui_assets: Res<UiAssets>,
) {
    // root node
    let ui_entity = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::height(Val::Px(1080.0)),
                            ..default()
                        },
                        image: UiImage {
                            texture: ui_assets.title.clone(),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .insert(PressSpaceMarker)
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::height(Val::Px(1080.0)),
                            ..default()
                        },
                        image: UiImage {
                            texture: ui_assets.space.clone(),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    commands.insert_resource(MenuUiData { ui_entity });
}

pub fn main_menu_system(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

pub fn main_menu_cleanup(mut commands: Commands, menu_data: Res<MenuUiData>) {
    commands.entity(menu_data.ui_entity).despawn_recursive();
}

// Bob the "Main Menu" text
pub fn main_menu_bob_start(time: Res<Time>, mut query: Query<(&PressSpaceMarker, &mut Style)>) {
    // I should have used sprites....
    let elapsed_time = time.elapsed_seconds();
    let amplitude_x = 50.0;
    let amplitude_y = 25.0;
    let frequency_x = 1.0;
    let frequency_y = 1.5;

    for (_, mut style) in query.iter_mut() {
        let x = amplitude_x * (elapsed_time * frequency_x).sin();
        let y = amplitude_y * (elapsed_time * frequency_y).cos();

        style.position = UiRect {
            top: Val::Px(y),
            left: Val::Px(x),
            ..default()
        }
    }
}

fn main_menu_music(background: Res<AudioChannel<Background>>, music_assets: Res<MusicAssets>) {
    background.stop();
    background.play(music_assets.title.clone()).looped();
}
