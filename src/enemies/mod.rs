pub mod skuller;
pub mod slimer;
pub mod sorcerian;

use bevy::{app::PluginGroupBuilder, prelude::*};
use std::time::Duration;

use skuller::SkullerPlugin;

use crate::unit::UnitBundle;

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

#[derive(Bundle)]
pub struct EnemyBundle {
    pub unit_bundle: UnitBundle,
    pub enemy: Enemy,
    pub name: Name,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            unit_bundle: UnitBundle::default(),
            enemy: Enemy::default(),
            name: Name::new("Enemy"),
        }
    }
}

#[derive(Resource, Reflect)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnemyEntityPlugins)
            .add_system(setup_enemy_spawn_timer.on_startup());
    }
}

pub fn setup_enemy_spawn_timer(mut commands: Commands) {
    commands.insert_resource(EnemySpawnTimer {
        timer: Timer::new(Duration::from_millis(150), TimerMode::Repeating),
    });
}
