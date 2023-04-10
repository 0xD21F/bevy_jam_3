use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveCollisionTypes, Collider, Sensor};

use crate::{
    animation::Animated,
    entity::{creature::Lifetime, lab_boss::LabBoss, EnemyHurtbox, EnemyHurtboxDamage},
    PIXELS_PER_METER,
};

use super::approach_and_keep_distance::ApproachAndKeepDistance;

// Entities in the `ApproachAndKeepDistance` state should move towards the given entity if they are
// too far away, and move away if they are too close
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct LabBossAttack;

#[derive(Clone, Bundle, Reflect)]
#[bundle(storage = "SparseSet")]
pub struct AttackAndKeepDistance {
    pub approach_and_keep_distance: ApproachAndKeepDistance,
    pub attack: LabBossAttack,
}

pub fn lab_boss_attack(
    mut commands: Commands,
    mut projectile_query: Query<(
        Entity,
        &LabBossAttack,
        &mut LabBoss,
        &mut Animated,
        &TextureAtlasSprite,
    )>,
) {
    for (entity, _, lab_boss, mut animated, sprite) in projectile_query.iter_mut() {
        // If the attack cooldown is finished, and the attack button is just pressed, and the player is not rolling, start attacking
        animated.first = 1;
        animated.last = 2;
        if (sprite.index) == 2 {
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    EnemyHurtbox {
                        collider: Collider::cuboid(PIXELS_PER_METER * 2.0, PIXELS_PER_METER * 1.0),
                        damage: EnemyHurtboxDamage(12),
                        sensor: Sensor,
                        transform: Transform::from_xyz(0.0, -PIXELS_PER_METER * 0.75, 0.0),
                        ..default()
                    },
                    ActiveCollisionTypes::STATIC_STATIC,
                    Lifetime {
                        timer: lab_boss.attack_timer.clone(),
                    },
                ));
            });
        }
    }
}

pub fn lab_boss_stop_attack(
    mut removals: RemovedComponents<LabBossAttack>,
    mut animated_query: Query<&mut Animated>,
) {
    for entity in removals.iter() {
        if let Ok(mut animated) = animated_query.get_mut(entity) {
            animated.first = 0;
            animated.last = 1;
        }
    }
}
