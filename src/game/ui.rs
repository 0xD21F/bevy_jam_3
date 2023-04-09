use bevy::prelude::*;
use bevy_mod_ui_texture_atlas_image::{AtlasImageBundle, UiAtlasImage};

use crate::app_state::{loading::UiAssets, AppState};

use super::mutation_manager::{mutation_manager_setup, MutationManager};

pub struct UiPlugin;

#[derive(Debug, PartialEq, Clone, Resource)]
pub struct UiState {
    player_mutations_version: u32,
    ui_root_node: Entity,
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

fn ui_setup(mut commands: Commands, mutation_manager: Res<MutationManager>) {
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
    });
}

fn ui_cleanup(mut commands: Commands, ui_state: Res<UiState>) {
    commands.entity(ui_state.ui_root_node).despawn_recursive();
}

#[derive(Component)]
pub struct MutationIcon;

fn ui_system(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    ui_state: Res<UiState>,
    mutation_manager: Res<MutationManager>,
    mut icon_query: Query<Entity, With<MutationIcon>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
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
