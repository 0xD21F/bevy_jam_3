use bevy::prelude::*;
use seldom_state::prelude::*;

use self::{states::BehaviourStatesPlugin, triggers::BehaviourTriggerPlugin};

// pub mod approach_and_keep_distance;
pub mod separation;
pub mod states;
pub mod triggers;

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StateMachinePlugin)
            .add_plugin(BehaviourTriggerPlugin)
            .add_plugin(BehaviourStatesPlugin);
    }
}
