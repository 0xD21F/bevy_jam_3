use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_mod_ui_texture_atlas_image::{AtlasImageBundle, UiAtlasImage};
use rand::seq::SliceRandom;
use bevy_kira_audio::AudioControl;
use crate::app_state::loading::{UiAssets, MusicAssets, Background};

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
            )
            .add_system(
                sine_wave_movement_ui_system.in_set(OnUpdate(GameState::MutationSelection)),
            );
    }
}

#[derive(Resource)]
pub struct MutationSelectionData {
    pub bg_container: Entity,
    pub button_entities: Vec<Entity>,
    pub offered_mutations: Vec<(Entity, MutationType)>,
}

#[derive(Component)]
pub struct MutationSelectionUiNode;

pub fn mutation_selection_setup(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mutation_manager: Res<MutationManager>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    background: Res<AudioChannel<Background>>,
    music_assets: Res<MusicAssets>,
) {
    background.stop();
    background.play(music_assets.mutate.clone());
    let mutation_icon_image = ui_assets.mutation_icons.clone();
    let mutation_icon_texture_atlas = TextureAtlas::from_grid(
        mutation_icon_image.clone(),
        16. * Vec2::ONE,
        17,
        1,
        None,
        None,
    );
    let mutation_icon_texture_atlas_handle = texture_atlases.add(mutation_icon_texture_atlas);

    // Generate 3 unique random mutations that the player hasn't acquired
    let mut rng = rand::thread_rng();
    let mut available_mutations: Vec<Mutation> = mutation_manager.unselected_mutations();
    available_mutations.shuffle(&mut rng);

    let offered_mutations = available_mutations
        .into_iter()
        .take(3)
        .collect::<Vec<Mutation>>();

    let button_positions = vec![
        Vec2::new(225.0, 310.0),
        Vec2::new(1270.0, 70.0),
        Vec2::new(1570.0, 520.0),
    ];

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

    for (i, mutation) in offered_mutations.iter().enumerate() {
        let button_entity = spawn_image_button(
            &mut commands,
            &button_positions[i],
            mutation_icon_texture_atlas_handle.clone(),
            mutation.icon_index as usize,
        );
        button_entities.push(button_entity);
    }

    // Update the MutationSelectionData resource
    commands.insert_resource(MutationSelectionData {
        bg_container,
        button_entities: button_entities.clone(),
        offered_mutations: offered_mutations
            .into_iter()
            .enumerate()
            .map(|(i, mutation)| (button_entities[i], mutation.mutation_type))
            .collect(),
    });
}

fn spawn_image_button(
    commands: &mut Commands,
    position: &Vec2,
    texture_atlas_handle: Handle<TextureAtlas>,
    icon_index: usize,
) -> Entity {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(256.0), Val::Px(256.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(position.x),
                    top: Val::Px(position.y),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)))
        .insert(SineWaveMovementButton {
            amplitude: 10.0,
            frequency: 2.0,
            initial_position_top: position.y,
        })
        .with_children(|parent| {
            parent.spawn(AtlasImageBundle {
                style: Style {
                    size: Size::new(Val::Px(256.0), Val::Px(256.0)),
                    ..default()
                },
                atlas_image: UiAtlasImage::new(texture_atlas_handle, icon_index),
                ..default()
            });
        })
        .id()
}

#[derive(Component)]
pub struct SineWaveMovementButton {
    pub amplitude: f32,
    pub frequency: f32,
    pub initial_position_top: f32,
}

pub fn mutation_selection_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>)>,
    menu_data: Res<MutationSelectionData>,
    mut mutation_manager: ResMut<MutationManager>,
) {
    for (interaction, entity) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if let Some((_, mutation_type)) = menu_data
                    .offered_mutations
                    .iter()
                    .find(|(e, _)| *e == entity)
                {
                    mutation_manager.add_mutation(*mutation_type);
                }
                next_state.set(GameState::SetupLevel)
            }
            _ => {}
        }
    }
}

pub fn mutation_selection_cleanup(mut commands: Commands, menu_data: Res<MutationSelectionData>) {
    commands.entity(menu_data.bg_container).despawn_recursive();
    for button_entity in &menu_data.button_entities {
        commands.entity(*button_entity).despawn_recursive();
    }
}

pub fn sine_wave_movement_ui_system(
    time: Res<Time>,
    mut query: Query<(&SineWaveMovementButton, &mut Style)>,
) {
    let elapsed_time = time.elapsed_seconds();
    for (sine_wave_movement, mut style) in query.iter_mut() {
        style.position.top = Val::Px(
            sine_wave_movement.initial_position_top
                + sine_wave_movement.amplitude
                    * (elapsed_time as f32 * sine_wave_movement.frequency).sin(),
        );
    }
}
