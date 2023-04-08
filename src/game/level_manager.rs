use bevy::prelude::*;

use crate::{
    app_state::{AppState},
    entity::{
        spawner::{EnemyType, Spawner},
    },
};

// pub struct LevelManagerPlugin;

// impl Plugin for LevelManagerPlugin {
//     fn build(&self, app: &mut bevy::prelude::App) {
//         app.add_system(level_manager_setup.in_schedule(OnEnter(AppState::InGame)))
//             .add_system(level_setup.in_schedule(OnEnter(GameState::SetupLevel)))
//             .add_system(level_cleanup.in_schedule(OnExit(GameState::InLevel)));
//     }
// }

