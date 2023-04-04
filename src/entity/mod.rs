pub mod creature;
pub mod player;
pub mod skuller;
pub mod slimer;
pub mod spawner;

use bevy::{app::PluginGroupBuilder, prelude::*};

use skuller::SkullerPlugin;

pub struct EnemyEntityPlugins;

impl PluginGroup for EnemyEntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(SkullerPlugin);
        group
    }
}

#[derive(Component, Reflect, Default)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnemyEntityPlugins);
    }
}
