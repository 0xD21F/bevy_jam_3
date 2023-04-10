use bevy::prelude::*;

use crate::{
    entity::{player::Player, spawner::Spawner, Enemy},
    game::{level_manager::LevelObject, GamePlugin, GameState},
};

use super::AppState;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(game_init.in_schedule(OnEnter(AppState::InGame)))
            .add_system(game_teardown.in_schedule(OnExit(AppState::InGame)))
            .add_plugin(GamePlugin);
    }
}

pub fn game_init(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::OpeningCutscene);
}

pub fn game_teardown(
    mut commands: Commands,
    spawner_query: Query<Entity, With<Spawner>>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    level_objects: Query<Entity, With<LevelObject>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for entity in spawner_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in level_objects.iter() {
        commands.entity(entity).despawn_recursive();
    }
    next_state.set(GameState::Exited);
}
