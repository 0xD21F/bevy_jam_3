use bevy::prelude::*;

use crate::app_state::loading::{CutsceneAssets, UiAssets};

use super::GameState;

pub struct MutationSelectionPlugin;

impl Plugin for MutationSelectionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(mutation_selection_setup.in_schedule(OnEnter(GameState::MutationSelection)))
            .add_system(mutation_selection_system.in_set(OnUpdate(GameState::MutationSelection)))
            .add_system(
                mutation_selection_cleanup.in_schedule(OnExit(GameState::MutationSelection)),
            );
    }
}

#[derive(Resource)]
pub struct MutationSelectionData {
    pub ui_entity: Entity,
    pub buttons_entity: Entity,
}

#[derive(Component)]
pub struct MutationSelectionUiNode;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn mutation_selection_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
                            texture: ui_assets.mutating.clone(),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    let buttons_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(256.0), Val::Px(256.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(225.0),
                            top: Val::Px(310.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Mutation 1",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(256.0), Val::Px(256.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(1270.0),
                            top: Val::Px(70.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Mutation 2",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(256.0), Val::Px(256.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(1570.0),
                            top: Val::Px(520.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Mutation 3",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .id();
    commands.insert_resource(MutationSelectionData {
        ui_entity,
        buttons_entity,
    });
}

pub fn mutation_selection_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::SetupLevel)
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn mutation_selection_cleanup(mut commands: Commands, menu_data: Res<MutationSelectionData>) {
    commands.entity(menu_data.ui_entity).despawn_recursive();
    commands
        .entity(menu_data.buttons_entity)
        .despawn_recursive();
}
