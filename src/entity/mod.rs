pub mod creature;
pub mod mutant;
pub mod player;
pub mod skuller;
pub mod slimer;
pub mod spawner;

use bevy::{app::PluginGroupBuilder, prelude::*};

use skuller::SkullerPlugin;

use self::{mutant::MutantPlugin, slimer::SlimerPlugin};

pub struct EnemyEntityPlugins;

impl PluginGroup for EnemyEntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group.add(SkullerPlugin).add(SlimerPlugin).add(MutantPlugin)
    }
}

#[derive(Component, Reflect, Default)]
pub struct ZSort {
    pub offset_y: f32,
}

#[derive(Component, Reflect, Default)]
pub struct Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnemyEntityPlugins);
    }
}
