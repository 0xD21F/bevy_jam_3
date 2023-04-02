use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    enemies::enemy_entity_plugins::EnemyEntityPlugins,
    player::Player,
    unit::{Unit, UnitBundle, Velocity},
    PIXELS_PER_METER,
};

#[derive(Component, Reflect)]
pub struct Enemy;

impl Default for Enemy {
    fn default() -> Self {
        Self {}
    }
}

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
        timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
    });
}
