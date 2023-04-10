use bevy::prelude::*;

use crate::app_state::AppState;

use self::approach_and_keep_distance::approach_and_keep_distance;
use self::attack::lab_boss_attack;
use self::attack::lab_boss_stop_attack;
use self::fire_projectile::fire_projectile;
use self::wander::wander;

pub mod approach_and_keep_distance;
pub mod attack;
pub mod fire_projectile;
pub mod idle;
pub mod wander;

pub struct BehaviourStatesPlugin;

impl Plugin for BehaviourStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(approach_and_keep_distance)
            .add_system(wander)
            .add_system(fire_projectile.in_set(OnUpdate(AppState::InGame)))
            .add_system(lab_boss_attack.in_set(OnUpdate(AppState::InGame)))
            .add_system(lab_boss_stop_attack);
    }
}
