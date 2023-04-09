pub mod adept;
pub mod creature;
pub mod goblin;
pub mod lab_boss;
pub mod level_exit;
pub mod level_start;
pub mod mutant;
pub mod player;
pub mod skuller;
pub mod slimer;
pub mod spawner;
pub mod sorcerian;

use bevy::{app::PluginGroupBuilder, prelude::*};

use skuller::SkullerPlugin;

use self::{
    adept::AdeptPlugin, goblin::GoblinPlugin, lab_boss::LabBossPlugin, level_exit::LevelExitPlugin,
    level_start::LevelStartPlugin, mutant::MutantPlugin, slimer::SlimerPlugin, sorcerian::SorcerianPlugin,
};

pub struct EnemyEntityPlugins;

impl PluginGroup for EnemyEntityPlugins {
    fn build(self) -> PluginGroupBuilder {
        let group = PluginGroupBuilder::start::<Self>();
        group
            .add(SkullerPlugin)
            .add(SlimerPlugin)
            .add(MutantPlugin)
            .add(GoblinPlugin)
            .add(AdeptPlugin)
            .add(LabBossPlugin)
            .add(LevelStartPlugin)
            .add(LevelExitPlugin)
            .add(SorcerianPlugin)
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
