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
    pub cutscene_image: Entity,
    pub cutscene_timer: Timer,
}

#[derive(Component)]
pub struct OpeningCutsceneImageNode;

pub fn opening_cutscene_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cutscene_assets: Res<CutsceneAssets>,
) {
    // root node
    let cutscene_image = commands
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
                    // bevy logo (image)
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
        cutscene_image,
        cutscene_timer: Timer::from_seconds(10.0, TimerMode::Once),
    });
}

pub fn opening_cutscene_system(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cutscene_image_node_query: Query<&mut Style, With<OpeningCutsceneImageNode>>,
    mut cutscene_data: ResMut<OpeningCutsceneData>,
    time: Res<Time>,
) {
    if keyboard_input.any_pressed([KeyCode::Space]) {
        next_state.set(GameState::SetupLevelManager);
    }

    for mut cutscene_image_node_style in cutscene_image_node_query.iter_mut() {
        if let Val::Px(value) = cutscene_image_node_style.position.left {
            if value < 0.0 {
                cutscene_image_node_style.position.left = Val::Px(value + 1.0);
            } else if value >= 0.0 {
                cutscene_data.cutscene_timer.tick(time.delta());
                if cutscene_data.cutscene_timer.just_finished() {
                    next_state.set(GameState::SetupLevelManager);
                }
            }
        } else {
            panic!("Expected right position to be in pixels");
        }
    }
}

pub fn opening_cutscene_cleanup(mut commands: Commands, cutscene_data: Res<OpeningCutsceneData>) {
    commands
        .entity(cutscene_data.cutscene_image)
        .despawn_recursive();
}
