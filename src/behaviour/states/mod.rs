use bevy::prelude::*;

use self::approach_and_keep_distance::approach_and_keep_distance;
use self::wander::wander;

pub mod approach_and_keep_distance;
pub mod wander;
pub mod idle;

pub struct BehaviourStatesPlugin;

impl Plugin for BehaviourStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(approach_and_keep_distance)
            .add_system(wander);
    }
}