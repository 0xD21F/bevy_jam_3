use bevy::{app::PluginGroupBuilder, prelude::*};

use super::skuller::SkullerPlugin;

pub struct EnemyEntityPlugins;

impl PluginGroup for EnemyEntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(SkullerPlugin);
        group
    }
}
