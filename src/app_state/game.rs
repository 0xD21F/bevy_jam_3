use bevy::prelude::*;

use crate::{
    entity::{player::Player, spawner::Spawner, Enemy},
    game::{GamePlugin, GameState},
};

use super::{AppState, loading::MusicAssets};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(game_init.in_schedule(OnEnter(AppState::InGame)))
            .add_system(game_teardown.in_schedule(OnExit(AppState::InGame)))
            .add_system(start_background_audio.in_schedule(OnEnter(AppState::InGame)))
            .add_plugin(GamePlugin);
    }
}

fn start_background_audio(music: Res<MusicAssets>, audio: Res<Audio>) {
    let menu_music = music.title.clone();
    let music_settings = PlaybackSettings {
        repeat: true,
        volume: 0.7,
        speed: 1.0,
    };
    audio.play_with_settings(menu_music, music_settings);
}

pub fn game_init(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::OpeningCutscene);
}

pub fn game_teardown(
    mut commands: Commands,
    spawner_query: Query<Entity, With<Spawner>>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
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
    next_state.set(GameState::Exited);
}
