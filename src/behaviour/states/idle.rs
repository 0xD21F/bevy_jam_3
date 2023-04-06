use bevy::prelude::*;

// Entities in the `Idle` state should do nothing
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Idle;