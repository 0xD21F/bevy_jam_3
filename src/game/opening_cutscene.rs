use bevy::prelude::*;

use crate::app_state::loading::CutsceneAssets;

use super::GameState;

pub struct OpeningCutscenePlugin;

impl Plugin for OpeningCutscenePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(opening_cutscene_setup.in_schedule(OnEnter(GameState::OpeningCutscene)))
            .add_system(opening_cutscene_system.in_set(OnUpdate(GameState::OpeningCutscene)))
            .add_system(opening_cutscene_cleanup.in_schedule(OnExit(GameState::OpeningCutscene)));
    }
}

#[derive(Resource)]
pub struct OpeningCutsceneData {
    pub cutscene_image_node: Entity,
    pub cutscene_timer_1: Timer,
    pub cutscene_timer_2: Timer,
    pub cutscene_timer_3: Timer,
}

#[derive(Component)]
pub struct OpeningCutsceneImageNode;

pub fn opening_cutscene_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cutscene_assets: Res<CutsceneAssets>,
) {
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
            // text
            parent.spawn((
                TextBundle::from_section(
                    "Press space to skip",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                }),
                Label,
            ));
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
                .insert(OpeningCutsceneImageNode)
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            size: Size::height(Val::Px(1080.0)),
                            ..default()
                        },
                        image: UiImage {
                            texture: cutscene_assets.opening1.clone(),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();
    commands.insert_resource(OpeningCutsceneData {
        cutscene_image_node,
        cutscene_timer_1: Timer::from_seconds(2.0, TimerMode::Once),
        cutscene_timer_2: Timer::from_seconds(5.0, TimerMode::Once),
        cutscene_timer_3: Timer::from_seconds(9.0, TimerMode::Once),
    });
}

pub fn opening_cutscene_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cutscene_image_node_query: Query<&mut Style, With<OpeningCutsceneImageNode>>,
    mut cutscene_data: ResMut<OpeningCutsceneData>,
    mut cutscene_image: Query<&mut UiImage>,
    cutscene_assets: Res<CutsceneAssets>,
    time: Res<Time>,
) {
    if keyboard_input.any_pressed([KeyCode::Space]) {
        next_state.set(GameState::SetupLevelManager);
    }

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

    if (cutscene_finished_panning) {
        cutscene_data.cutscene_timer_1.tick(time.delta());
        if cutscene_data.cutscene_timer_1.finished() {
            if let Ok(mut ui_image) = cutscene_image.get_single_mut() {
                ui_image.texture = cutscene_assets.opening2.clone();
            }
        }

        cutscene_data.cutscene_timer_2.tick(time.delta());
        if cutscene_data.cutscene_timer_1.finished() && cutscene_data.cutscene_timer_2.finished() {
            if let Ok(mut ui_image) = cutscene_image.get_single_mut() {
                ui_image.texture = cutscene_assets.opening3.clone();
            }
        }

        cutscene_data.cutscene_timer_3.tick(time.delta());
        if cutscene_data.cutscene_timer_1.finished()
            && cutscene_data.cutscene_timer_2.finished()
            && cutscene_data.cutscene_timer_3.finished()
        {
            next_state.set(GameState::SetupLevelManager);
        }
    }
}

pub fn opening_cutscene_cleanup(mut commands: Commands, cutscene_data: Res<OpeningCutsceneData>) {
    commands
        .entity(cutscene_data.cutscene_image_node)
        .despawn_recursive();
}
