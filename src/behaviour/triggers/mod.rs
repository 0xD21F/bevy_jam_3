use bevy::prelude::*;

use seldom_state::prelude::*;

pub struct BehaviourTriggerPlugin;

impl Plugin for BehaviourTriggerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TriggerPlugin::<Near>::default());
    }
}

// This trigger checks if the entity is within the the given range of the target
// Example: https://github.com/Seldom-SE/seldom_state/blob/main/examples/chase.rs
#[derive(Clone, Copy, Reflect)]
pub struct Near {
    pub target: Entity,
    pub range: f32,
}

impl Trigger for Near {
    // Put the parameters that your trigger needs here
    // For concision, you may use `bevy_ecs::system::system_param::lifetimeless` variants of system
    // params, like so:
    // `type Param<'w, 's> = (SQuery<&'static Transform>, SRes<Time>);`
    // Triggers are immutable; you may not access system params mutably
    // Do not query for the `StateMachine` component in this type. This, unfortunately, will panic.
    // `Time` is included here to demonstrate how to get multiple system params
    type Param<'w, 's> = (Query<'w, 's, &'static Transform>, Res<'w, Time>);
    // These types are used by transition builders, for dataflow from triggers to transitions
    // See `StateMachine::trans_builder`
    type Ok = f32;
    type Err = f32;

    // This function checks if the given entity should trigger
    // It runs once per frame for each entity that is in a state that can transition
    // on this trigger
    // Return `Ok` to trigger and `Err` to not trigger
    fn trigger(
        &self,
        entity: Entity,
        (transforms, _time): &Self::Param<'_, '_>,
    ) -> Result<f32, f32> {
        // Find the displacement between the target and this entity
        let delta = transforms.get(self.target).unwrap().translation.truncate()
            - transforms.get(entity).unwrap().translation.truncate();

        // Use the Pythagorean Theorem to determine whether the target is within range
        // If it is, return `Ok` to trigger!
        let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
        (distance <= self.range).then_some(distance).ok_or(distance)
    }
}
