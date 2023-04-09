use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::app_state::loading::UiAssets;

use super::{
    mutation_manager::{Mutation, MutationManager, MutationType},
    GameState,
};

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
    pub bg_container: Entity,
    pub buttons_container: Entity,
    pub offered_mutations: Vec<(Entity, MutationType)>,
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
    mutation_manager: Res<MutationManager>,
) {
    // Generate 3 unique random mutations that the player hasn't acquired
    let mut rng = rand::thread_rng();
    let mut available_mutations: Vec<Mutation> = mutation_manager.unselected_mutations();
    available_mutations.shuffle(&mut rng);

    let offered_mutations = available_mutations
        .into_iter()
        .take(3)
        .collect::<Vec<Mutation>>();

    // BG image
    let bg_container = commands
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
                            texture: ui_assets.mutating.clone(),
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    let mut button_entities = Vec::new();
    // Buttons
    let buttons_container = commands
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
            button_entities.push(
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
                    })
                    .id(),
            );
            button_entities.push(
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
                    })
                    .id(),
            );
            button_entities.push(
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
                    })
                    .id(),
            );
        })
        .id();

    // Update the MutationSelectionData resource
    commands.insert_resource(MutationSelectionData {
        bg_container,
        buttons_container,
        offered_mutations: offered_mutations
            .into_iter()
            .enumerate()
            .map(|(i, mutation)| (button_entities[i], mutation.mutation_type))
            .collect(),
    });
}

pub fn mutation_selection_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Entity),
        (Changed<Interaction>, With<Button>),
    >,
    menu_data: Res<MutationSelectionData>,
    mut mutation_manager: ResMut<MutationManager>,
) {
    for (interaction, mut color, entity) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                if let Some((_, mutation_type)) = menu_data
                    .offered_mutations
                    .iter()
                    .find(|(e, _)| *e == entity)
                {
                    mutation_manager.add_mutation(*mutation_type);
                }
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
    commands.entity(menu_data.bg_container).despawn_recursive();
    commands
        .entity(menu_data.buttons_container)
        .despawn_recursive();
}
