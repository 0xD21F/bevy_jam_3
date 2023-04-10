use bevy::prelude::*;
use bevy_mod_ui_texture_atlas_image::{AtlasImageBundle, UiAtlasImage};

use crate::{
    app_state::{loading::UiAssets, AppState},
    entity::{creature::Creature, player::Player},
};

use super::mutation_manager::{mutation_manager_setup, MutationManager};

pub struct UiPlugin;

#[derive(Debug, PartialEq, Clone, Resource)]
pub struct UiState {
    player_mutations_version: u32,
    ui_root_node: Entity,
    last_index: usize,
    last_color: Color,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(
            ui_setup
                .in_schedule(OnEnter(AppState::InGame))
                .after(mutation_manager_setup),
        )
        .add_system(ui_cleanup.in_schedule(OnExit(AppState::InGame)))
        .add_system(
            ui_system
                .in_set(OnUpdate(AppState::InGame))
                .after(ui_setup)
                .after(mutation_manager_setup),
        );
    }
}

fn ui_setup(
    mut commands: Commands,
    mutation_manager: Res<MutationManager>,
    _ui_assets: Res<UiAssets>,
    _texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let ui_entity = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(64.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .id();

    commands.insert_resource(UiState {
        player_mutations_version: mutation_manager.player_mutations_version,
        ui_root_node: ui_entity,
        last_index: 0,
        last_color: Color::default(),
    });
}

fn ui_cleanup(mut commands: Commands, ui_state: Res<UiState>) {
    commands.entity(ui_state.ui_root_node).despawn_recursive();
}

#[derive(Component)]
pub struct MutationIcon;

#[derive(Component)]
pub struct HealthIcon;

fn health_icon(creature_health: f32, max_health: f32) -> usize {
    // Clamp the creature's health between 0 and max_health
    let clamped_health = creature_health.min(max_health).max(0.0);

    // Calculate the health percentage
    let health_percentage = clamped_health / max_health * 100.0;

    // Determine which icon to display based on health_percentage
    if health_percentage > 75.0 {
        0
    } else if health_percentage > 50.0 {
        1
    } else if health_percentage > 25.0 {
        2
    } else {
        3
    }
}

fn ui_system(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut ui_state: ResMut<UiState>,
    mutation_manager: Res<MutationManager>,
    mut icon_query: Query<Entity, With<MutationIcon>>,
    mut health_icon_query: Query<Entity, With<HealthIcon>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut player_query: Query<&mut Creature, With<Player>>,
) {
    let player = player_query.get_single_mut();
    // check the result of the query
    if let Ok(player) = player {
        let index;

        if !player.damage_invulnerability.finished() && player.health < player.max_health {
            index = 2;
        } else {
            index = health_icon(player.health, player.max_health);
        }

        let color = Color::rgba(
            1.0 - (player.health / player.max_health),
            player.health / player.max_health,
            0.0,
            0.8,
        );

        if !(ui_state.last_color == color && ui_state.last_index == index) {
            ui_state.last_index = index;
            ui_state.last_color = color;

            // Remove existing Health icon
            for entity in health_icon_query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }

            let image = ui_assets.portrait.clone();
            let texture_atlas = TextureAtlas::from_grid(image, 64. * Vec2::ONE, 4, 1, None, None);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let container = ui_state.ui_root_node;
            commands.entity(container).with_children(|parent| {
                parent
                    .spawn(AtlasImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(128.0), Val::Px(128.0)),
                            margin: UiRect {
                                left: Val::Px(64.0_f32),
                                ..default()
                            },
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                top: Val::Px(900.0),
                                left: Val::Px(0.0),
                                ..Default::default()
                            },
                            ..default()
                        },
                        atlas_image: UiAtlasImage::new(texture_atlas_handle.clone(), index),
                        ..default()
                    })
                    .insert(BackgroundColor(color))
                    .insert(HealthIcon);
            });
        }
    }

    if ui_state.player_mutations_version == mutation_manager.player_mutations_version {
        return;
    }

    // Remove existing mutation icons
    for entity in icon_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    let image = ui_assets.mutation_icons.clone();
    let texture_atlas = TextureAtlas::from_grid(image, 32. * Vec2::ONE, 17, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let container = ui_state.ui_root_node;

    for (i, mutation) in mutation_manager.player_mutations.iter().enumerate() {
        let icon_index = mutation.icon_index;

        // Add children to the container
        commands.entity(container).with_children(|parent| {
            parent
                .spawn(AtlasImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                        margin: UiRect {
                            left: Val::Px(40.0 * i as f32),
                            ..default()
                        },
                        ..default()
                    },
                    atlas_image: UiAtlasImage::new(texture_atlas_handle.clone(), icon_index),
                    ..default()
                })
                .insert(MutationIcon);
        });
    }
}
