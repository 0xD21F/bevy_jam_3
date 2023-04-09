use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkLevel};
use bevy_rapier2d::prelude::*;

use crate::{animation::Animated, game::GameState};

use super::{player::Player, ZSort};

#[derive(Component, Reflect)]
pub struct Creature {
    pub acceleration: f32,
    pub friction: f32,
    pub max_speed: f32,
    pub health: f32,
}

impl Default for Creature {
    fn default() -> Self {
        Self {
            acceleration: 128.0,
            friction: 128.0,
            max_speed: 128.0,
            health: 128.0,
        }
    }
}

#[derive(Component, Reflect)]
pub struct Velocity {
    pub value: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            value: Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub animation: Animated,
    pub sprite: SpriteSheetBundle,
    pub collider: Collider,
    pub velocity: Velocity,
    pub zsort: ZSort,
    pub sensor: Sensor,
    pub hitbox: Hitbox,
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature::default(),
            animation: Animated::default(),
            sprite: Default::default(),
            collider: Collider::ball(1.0),
            sensor: Sensor::default(),
            hitbox: Hitbox,
            velocity: Velocity::default(),
            zsort: ZSort::default(),
        }
    }
}

#[derive(Component, Reflect)]
pub struct Hitbox;

#[derive(Component, Reflect)]
pub struct DontSetFacing;
pub struct CreaturePlugin;

#[derive(Component, Reflect)]
pub struct DealDamage {
    pub amount: f32,
    pub knockback_direction: Vec2,
    pub knockback_force: f32,
}

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(apply_friction_system)
            .add_system(apply_velocity_system)
            .add_system(z_ordering_system)
            .add_system(set_sprite_facing_system)
            .add_system(
                creature_clamp_to_current_level
                    .in_set(OnUpdate(GameState::InLevel))
                    .after(apply_velocity_system),
            )
            .add_system(
                creature_clamp_to_current_level
                    .in_set(OnUpdate(GameState::LevelComplete))
                    .after(apply_velocity_system),
            )
            .add_system(deal_damage_system);
    }
}

pub fn apply_velocity_system(time: Res<Time>, mut player_info: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in player_info.iter_mut() {
        let delta = velocity.value * time.delta_seconds();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;

        if delta == Vec2::ZERO {
            transform.translation.x = transform.translation.x.round();
            transform.translation.y = transform.translation.y.round();
        }
    }
}

pub fn apply_friction_system(time: Res<Time>, mut player_info: Query<(&mut Velocity, &Creature)>) {
    for (mut velocity, creature) in player_info.iter_mut() {
        if velocity.value != Vec2::ZERO {
            let friction = creature.friction * time.delta_seconds();
            let friction_vector = velocity.value.normalize() * friction;

            let new_velocity = velocity.value - friction_vector;
            if new_velocity.length() < friction {
                velocity.value = Vec2::ZERO;
            } else {
                velocity.value -= friction_vector;
            }
        }
    }
}

#[derive(Component, Reflect)]
pub struct FacePlayer;

pub fn set_sprite_facing_system(
    mut query: Query<(&mut TextureAtlasSprite, &Velocity, &Transform, Option<&FacePlayer>), Without<DontSetFacing>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (mut sprite, velocity, transform, face_player) in query.iter_mut() {
        if let Some(_face_player) = face_player {
            if let Ok(player_transform) = player_query.get_single() {
                // Flip the sprite to face the player
                let player_pos = player_transform.translation;
                let creature_pos = transform.translation;
                let direction = player_pos - creature_pos;
                if direction.x > 0.0 {
                    sprite.flip_x = false;
                } else if direction.x < 0.0 {
                    sprite.flip_x = true;
                }
            }
        } 
        else if velocity.value.x > 0.0 {
            sprite.flip_x = false;
        } else if velocity.value.x < 0.0 {
            sprite.flip_x = true;
        }
    }
}

pub fn z_ordering_system(
    mut z_sort_query: Query<(&mut Transform, &ZSort)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
) {
    // TODO: Handle multiple cameras
    let (camera_transform, camera) = camera_query.single();

    for (mut transform, zsort) in z_sort_query.iter_mut() {
        // Based on the screen space Y position, set the Z position to be in front of or behind other sprites
        let viewport_pos = camera.world_to_viewport(camera_transform, transform.translation);
        if let Some(pos) = viewport_pos {
            transform.translation.z = (-((pos.y * 2.0) - zsort.offset_y * 2.0) / 1000.0).min(0.0);
        } else {
            transform.translation.z = 0.0;
        }
    }
}

pub fn creature_clamp_to_current_level(
    mut creature_query: Query<&mut Transform, With<Creature>>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), Without<Creature>>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    for mut transform in creature_query.iter_mut() {
        for (level_transform, level_handle) in level_query.iter() {
            if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
                let level = &ldtk_level.level;
                let level_width = level.px_wid as f32;
                let level_height = level.px_hei as f32;

                // Calculate the min and max values for clamping
                let min_x = level_transform.translation.x;
                let max_x = level_transform.translation.x + level_width;
                let min_y = level_transform.translation.y;
                let max_y = level_transform.translation.y + level_height - 64.0;

                // Clamp the creature's position inside the level bounds
                transform.translation.x = transform.translation.x.clamp(min_x, max_x);
                transform.translation.y = transform.translation.y.clamp(min_y, max_y);
            }
        }
    }
}

pub fn deal_damage_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Creature, &mut Velocity, &DealDamage)>,
) {
    for (entity, mut creature, mut velocity, damage) in query.iter_mut() {
        creature.health -= damage.amount;
        if creature.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            velocity.value += damage.knockback_direction.normalize() * damage.knockback_force;
        }
        commands.entity(entity).remove::<DealDamage>();
    }
}
