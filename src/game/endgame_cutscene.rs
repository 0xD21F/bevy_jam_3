use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

use crate::app_state::{
    loading::{Background, CutsceneAssets, MusicAssets},
    AppState,
};

use bevy_kira_audio::AudioControl;

use super::GameState;

pub struct EndgameCutscenePlugin;

impl Plugin for EndgameCutscenePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(endgame_cutscene_setup.in_schedule(OnEnter(GameState::EndgameCutscene)))
            .add_system(endgame_cutscene_system.in_set(OnUpdate(GameState::EndgameCutscene)))
            .add_system(endgame_cutscene_cleanup.in_schedule(OnExit(GameState::EndgameCutscene)));
    }
}

#[derive(Resource)]
pub struct EndgameCutsceneData {
    pub cutscene_image_node: Entity,
    pub cutscene_timer_1: Timer,
    pub cutscene_timer_2: Timer,
    pub cutscene_timer_3: Timer,
}

#[derive(Component)]
pub struct EndgameCutsceneImageNode;

pub fn endgame_cutscene_setup(
    mut commands: Commands,
    cutscene_assets: Res<CutsceneAssets>,
    background: Res<AudioChannel<Background>>,
    music_assets: Res<MusicAssets>,
) {
    background.stop();
    background.play(music_assets.ending.clone());

    // root node
    let cutscene_image_node = commands
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
                        position: UiRect {
                            left: Val::Px(-1000.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(EndgameCutsceneImageNode)
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::height(Val::Px(1080.0)),
                            ..default()
                        },
                        image: UiImage {
                            texture: cutscene_assets.ending1.clone(),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    commands.insert_resource(EndgameCutsceneData {
        cutscene_image_node,
        cutscene_timer_1: Timer::from_seconds(5.0, TimerMode::Once),
        cutscene_timer_2: Timer::from_seconds(10.0, TimerMode::Once),
        cutscene_timer_3: Timer::from_seconds(19.0, TimerMode::Once),
    });
}

pub fn endgame_cutscene_system(
    _commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut cutscene_image_node_query: Query<&mut Style, With<EndgameCutsceneImageNode>>,
    mut cutscene_data: ResMut<EndgameCutsceneData>,
    mut cutscene_image: Query<&mut UiImage>,
    cutscene_assets: Res<CutsceneAssets>,
    time: Res<Time>,
) {
    let mut cutscene_finished_panning = false;
    for mut cutscene_image_node_style in cutscene_image_node_query.iter_mut() {
        if let Val::Px(value) = cutscene_image_node_style.position.left {
            if value < 0.0 {
                cutscene_image_node_style.position.left = Val::Px(value + 1.0);
            } else {
                cutscene_finished_panning = true;
            }
        } else {
            panic!("Expected right position to be in pixels");
        }
    }

    if cutscene_finished_panning {
        cutscene_data.cutscene_timer_1.tick(time.delta());
        if cutscene_data.cutscene_timer_1.finished() {
            if let Ok(mut ui_image) = cutscene_image.get_single_mut() {
                ui_image.texture = cutscene_assets.ending2.clone();
            }
        }

        cutscene_data.cutscene_timer_2.tick(time.delta());
        if cutscene_data.cutscene_timer_1.finished() && cutscene_data.cutscene_timer_2.finished() {
            if let Ok(mut ui_image) = cutscene_image.get_single_mut() {
                ui_image.texture = cutscene_assets.ending3.clone();
            }
        }

        cutscene_data.cutscene_timer_3.tick(time.delta());
        if cutscene_data.cutscene_timer_1.finished()
            && cutscene_data.cutscene_timer_2.finished()
            && cutscene_data.cutscene_timer_3.finished()
        {
            next_state.set(AppState::MainMenu);
        }
    }
}

pub fn endgame_cutscene_cleanup(mut commands: Commands, cutscene_data: Res<EndgameCutsceneData>) {
    commands
        .entity(cutscene_data.cutscene_image_node)
        .despawn_recursive();
}
