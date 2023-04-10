use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveCollisionTypes, Collider};

use crate::{
    app_state::loading::SpriteAssets,
    entity::{
        creature::{Creature, Lifetime, Velocity},
        spawner::Projectile,
        EnemyHurtboxDamage,
    },
    PIXELS_PER_METER,
};

use super::approach_and_keep_distance::ApproachAndKeepDistance;

// Entities in the `ApproachAndKeepDistance` state should move towards the given entity if they are
// too far away, and move away if they are too close
#[derive(Clone, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct FireProjectile {
    pub target: Entity,
    pub projectile: Projectile,
}

#[derive(Clone, Bundle, Reflect)]
#[bundle(storage = "SparseSet")]
pub struct FireProjectileAndKeepDistance {
    pub fire_projectile: FireProjectile,
    pub keep_distance: ApproachAndKeepDistance,
}

pub fn fire_projectile(
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut projectile_query: Query<(
        Entity,
        &mut Velocity,
        &Creature,
        &Transform,
        &FireProjectile,
    )>,
    _time: Res<Time>,
    sprite_assets: Res<SpriteAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (entity, _velocity, _creature, transform, projectile) in projectile_query.iter_mut() {
        // Get the target entity's Transform
        if let Ok(target_transform) = transforms.get(projectile.target) {
            // Calculate the direction vector from the entity to the target
            let direction = target_transform.translation - transform.translation;

            // Spawn the projectile
            match projectile.projectile {
                Projectile::MutantProjectile => {
                    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
                        sprite_assets.mutant.clone(),
                        Vec2::new(64.0, 64.0),
                        4,
                        1,
                        None,
                        None,
                    ));

                    let sprite = SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,
                        sprite: TextureAtlasSprite::new(3),
                        transform: *transform,
                        ..default()
                    };

                    commands
                        .spawn(sprite)
                        .insert(ActiveCollisionTypes::STATIC_STATIC)
                        .insert(Collider::ball(PIXELS_PER_METER * 0.25))
                        .insert(Velocity {
                            value: direction.truncate().normalize() * 120.0,
                        })
                        .insert(EnemyHurtboxDamage(4))
                        .insert(Lifetime {
                            timer: Timer::from_seconds(3.0, TimerMode::Once),
                        });
                }
            }

            // Remove the FireProjectile component from the entity
            commands.entity(entity).remove::<FireProjectile>();
        }
    }
}
