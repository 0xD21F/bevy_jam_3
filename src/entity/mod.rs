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
pub mod sorcerian;
pub mod spawner;

use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_kira_audio::AudioChannel;
use bevy_kira_audio::AudioControl;
use bevy_rapier2d::prelude::{Collider, RapierContext, Sensor};
use skuller::SkullerPlugin;

use crate::entity::creature::Bleed;
use crate::game::mutation_manager;
use crate::game::mutation_manager::MutationManager;
use crate::game::mutation_manager::MutationType;
use crate::{
    app_state::{
        loading::{SfxAssets, SoundEffects},
        AppState,
    },
    entity::creature::DealDamage,
};

use self::{
    adept::AdeptPlugin,
    creature::Creature,
    goblin::GoblinPlugin,
    lab_boss::LabBossPlugin,
    level_exit::LevelExitPlugin,
    level_start::LevelStartPlugin,
    mutant::MutantPlugin,
    player::{Immune, Player, PlayerHurtboxDamage},
    slimer::SlimerPlugin,
    sorcerian::SorcerianPlugin,
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
        app.add_plugins(EnemyEntityPlugins)
            .add_system(enemy_damage_system.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Bundle, Default)]
pub struct EnemyHurtbox {
    pub collider: Collider,
    pub damage: EnemyHurtboxDamage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub sensor: Sensor,
}

#[derive(Component, Default)]
pub struct EnemyHurtboxDamage(pub u32);

fn enemy_damage_system(
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut player_hitbox_query: Query<
        (
            Entity,
            &Transform,
            &mut Creature,
            &Collider,
            Option<&Immune>,
        ),
        With<Player>,
    >,
    mut enemy_hurtbox_query: Query<(Entity, &GlobalTransform, &Collider, &EnemyHurtboxDamage)>,
    mutation_manager: Res<MutationManager>,
) {
    for (player_hitbox_entity, enemy_transform, _enemy_creature, _enemy_collider, immune) in
        player_hitbox_query.iter_mut()
    {
        if immune.is_some() {
            continue;
        }
        for (player_hurtbox_entity, player_transform, _player_collider, player_hurtbox_damage) in
            enemy_hurtbox_query.iter_mut()
        {
            if rapier_context.intersection_pair(player_hurtbox_entity, player_hitbox_entity)
                == Some(true)
            {
                // Get direction to knock enemy back
                commands.entity(player_hitbox_entity).insert(DealDamage {
                    amount: player_hurtbox_damage.0 as f32,
                    knockback_direction: (enemy_transform.translation.truncate()
                        - player_transform.translation().truncate())
                    .normalize_or_zero(),
                    knockback_force: 250.0,
                });

                // If the player has Hemophilia
                if mutation_manager.has_mutation(MutationType::Hemophilia) {
                    commands.entity(player_hitbox_entity).insert(Bleed {
                        damage: 1.0,
                        ticks: 3,
                        tick_timer: Timer::from_seconds(1.5, TimerMode::Once),
                    });
                }
            }
        }
    }
}

#[derive(Component, Default)]
pub struct Die;
