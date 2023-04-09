use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RapierContext, RigidBody, Sensor};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap, VirtualDPad},
    Actionlike, InputManagerBundle,
};
use seldom_state::prelude::InputTriggerPlugin;

use crate::{
    animation::Animated,
    app_state::{loading::SpriteAssets, AppState},
    entity::creature::DealDamage,
    PIXELS_PER_METER,
};

use super::{
    creature::{Creature, CreatureBundle, Velocity},
    Enemy, ZSort,
};

#[derive(Component, Reflect)]
pub struct Player {
    pub attack_timer: Timer,
    pub attack_cooldown: Timer,
    pub roll_timer: Timer,
    pub roll_invulnerable_timer: Timer,
    pub roll_cooldown_timer: Timer,
    pub roll_speed_multiplier: f32,
    pub animation_state: PlayerAnimationState,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            attack_timer: Timer::from_seconds(0.4, TimerMode::Once),
            attack_cooldown: Timer::from_seconds(0.6, TimerMode::Once),
            roll_timer: Timer::from_seconds(0.5, TimerMode::Once),
            roll_invulnerable_timer: Timer::from_seconds(0.35, TimerMode::Once),
            roll_cooldown_timer: Timer::from_seconds(2.5, TimerMode::Once),
            roll_speed_multiplier: 5.0,
            animation_state: PlayerAnimationState::Idle,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub unit_bundle: CreatureBundle,
    pub player: Player,
    pub name: Name,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            unit_bundle: CreatureBundle::default(),
            player: Player::default(),
            name: Name::new("Player"),
        }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerHurtbox {
    pub collider: Collider,
    pub damage: PlayerHurtboxDamage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub sensor: Sensor,
}

#[derive(Component, Default)]
pub struct PlayerHurtboxDamage(u32);

pub struct PlayerPlugin;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Attack,
    Roll,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_plugin(InputTriggerPlugin::<PlayerAction>::default())
            .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
            .add_system(player_movement_system)
            .add_system(player_rolling_state_system)
            .add_system(player_rolling_behavior_system)
            .add_system(player_attacking_state_system)
            .add_system(player_attacking_behavior_system)
            .add_system(player_animation_system)
            .add_system(player_damage_system);
    }
}

#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum PlayerAnimationState {
    Idle,
    Rolling,
    Attacking,
}

pub fn player_attacking_state_system(
    time: Res<Time>,
    mut player_info: Query<(Entity, &mut Player, &ActionState<PlayerAction>)>,
    hurtbox_query: Query<Entity, With<PlayerHurtboxDamage>>,
    attacking_query: Query<&Attacking>,
    rolling_query: Query<&Rolling>,
    mut commands: Commands,
) {
    for (entity, mut player, action_state) in player_info.iter_mut() {
        let is_rolling = rolling_query.get(entity).is_ok();
        let is_attacking = attacking_query.get(entity).is_ok();

        player.attack_cooldown.tick(time.delta());
        // If the attack cooldown is finished, and the attack button is just pressed, and the player is not rolling, start attacking
        if player.attack_cooldown.finished()
            && action_state.just_pressed(PlayerAction::Attack)
            && !is_rolling
        {
            commands
                .entity(entity)
                .insert(Attacking)
                .with_children(|parent| {
                    parent.spawn((
                        PlayerHurtbox {
                            collider: Collider::cuboid(
                                PIXELS_PER_METER * 4.5,
                                PIXELS_PER_METER * 1.5,
                            ),
                            damage: PlayerHurtboxDamage(20),
                            sensor: Sensor,
                            ..default()
                        },
                        RigidBody::Fixed,
                    ));
                });
            player.attack_cooldown.reset();
            player.attack_timer.reset();
        }

        // If attacking, tick the attack timer
        if is_attacking {
            player.attack_timer.tick(time.delta());
        }
        // If the attack_timer is finished, remove the Attacking and PlayerHurtbox components from the player and reset the cooldown timer
        if player.attack_timer.finished() && is_attacking {
            commands.entity(entity).remove::<Attacking>();
            hurtbox_query.iter().for_each(|hurtbox_entity| {
                commands.entity(hurtbox_entity).despawn_recursive();
            });

            player.attack_timer.reset();
        }
    }
}
pub fn player_attacking_behavior_system(
    _time: Res<Time>,
    _player_info: Query<(Entity, &Player, &Attacking, &mut Transform, &mut Collider)>,
) {
}

fn player_animation_system(
    rolling_query: Query<&Rolling>,
    attacking_query: Query<&Attacking>,
    mut player_info: Query<(
        Entity,
        &mut Player,
        &mut Animated,
        &mut Collider,
        &mut TextureAtlasSprite,
    )>,
    _commands: Commands,
) {
    for (entity, mut player, mut animated, mut collider, _texture_atlas_sprite) in
        player_info.iter_mut()
    {
        let is_rolling = rolling_query.get(entity).is_ok();
        let is_attacking = attacking_query.get(entity).is_ok();

        // Determine the player's current animation state based on their rolling and attacking status
        let current_animation_state = if is_rolling {
            PlayerAnimationState::Rolling
        } else if is_attacking {
            PlayerAnimationState::Attacking
        } else {
            PlayerAnimationState::Idle
        };

        // Check if the animation state has changed
        if current_animation_state != player.animation_state {
            // Update the player's animation state
            player.animation_state = current_animation_state;

            // Get the Animated and Collider properties for the current animation state
            let state_animated = player.animation_state.animated();
            let state_collider = player.animation_state.collider();

            // Update the player's Animated, Collider, and TextureAtlasSprite components
            *animated = state_animated;
            *collider = state_collider;
        }
    }
}

impl PlayerAnimationState {
    fn animated(self) -> Animated {
        match self {
            PlayerAnimationState::Idle => Animated {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                first: 0,
                last: 0,
                ..default()
            },
            PlayerAnimationState::Rolling => Animated {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                first: 1, // Replace with the first frame of the rolling animation
                last: 1,  // Replace with the last frame of the rolling animation
                ..default()
            },
            PlayerAnimationState::Attacking => Animated {
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                first: 2, // Replace with the first frame of the attacking animation
                last: 3,  // Replace with the last frame of the attacking animation
                ..default()
            },
        }
    }

    fn collider(self) -> Collider {
        match self {
            PlayerAnimationState::Idle => Collider::ball(PIXELS_PER_METER * 1.0),
            PlayerAnimationState::Rolling => Collider::ball(PIXELS_PER_METER * 0.5),
            PlayerAnimationState::Attacking => Collider::ball(PIXELS_PER_METER * 1.0),
        }
    }
}

#[derive(Default, Component)]
pub struct Rolling {
    pub direction: Vec2,
}

#[derive(Default, Component)]
pub struct Attacking;

#[derive(Default, Component)]
pub struct Immune;

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<
        (Entity, &Creature, &mut Velocity, &ActionState<PlayerAction>),
        With<Player>,
    >,
    mut next_state: ResMut<NextState<AppState>>,
    rolling_query: Query<&Rolling>,
) {
    for (entity, creature, mut velocity, action_state) in player_info.iter_mut() {
        let quit = keyboard_input.any_pressed([KeyCode::Escape]);
        if quit {
            next_state.set(AppState::MainMenu);
        }

        let is_rolling = rolling_query.get(entity).is_ok();
        if !is_rolling {
            let axis_pair = action_state.axis_pair(PlayerAction::Move);

            let axis_pair = match axis_pair {
                Some(axis_pair) => axis_pair,
                None => continue,
            };

            if axis_pair.xy().length() > 0.0 {
                let input_magnitude = axis_pair.xy().length();
                let normalized_input_vector = axis_pair.xy() / input_magnitude;

                let acceleration_vector =
                    normalized_input_vector * creature.acceleration * time.delta_seconds();
                velocity.value += acceleration_vector;
            }
            // Limit maximum speed
            velocity.value = velocity.value.clamp_length_max(creature.max_speed);
        }
    }
}

pub fn player_rolling_state_system(
    time: Res<Time>,
    mut player_info: Query<(Entity, &mut Player, &Velocity, &ActionState<PlayerAction>)>,
    rolling_query: Query<&Rolling>,
    attacking_query: Query<&Attacking>,
    immune_query: Query<&Immune>,
    mut commands: Commands,
    hurtbox_query: Query<Entity, With<PlayerHurtboxDamage>>,
) {
    for (entity, mut player, _velocity, action_state) in player_info.iter_mut() {
        let is_rolling = rolling_query.get(entity).is_ok();
        let is_immune = immune_query.get(entity).is_ok();
        let is_attacking = attacking_query.get(entity).is_ok();

        player.roll_cooldown_timer.tick(time.delta());
        // If the roll cooldown is finished, and the roll button is just pressed, and the player is not rolling, start rolling and set the entity as immune
        if player.roll_cooldown_timer.finished()
            && action_state.just_pressed(PlayerAction::Roll)
            && !is_rolling
        {
            let axis_pair = action_state.axis_pair(PlayerAction::Move);
            let axis_pair = match axis_pair {
                Some(axis_pair) => axis_pair,
                None => continue,
            };

            commands.entity(entity).insert(Rolling {
                direction: axis_pair.xy().normalize_or_zero(),
            });
            commands.entity(entity).insert(Immune);
            player.roll_timer.reset();
            player.roll_invulnerable_timer.reset();
            player.roll_cooldown_timer.reset();
            if is_attacking {
                commands.entity(entity).remove::<Attacking>();
                hurtbox_query.iter().for_each(|hurtbox_entity| {
                    commands.entity(hurtbox_entity).despawn_recursive();
                });
            }
        }

        // If rolling, tick the roll timer
        if is_rolling {
            player.roll_timer.tick(time.delta());
        }
        // If the roll timer is finished, remove the rolling component from the player and reset the cooldown timer
        if player.roll_timer.finished() && is_rolling {
            commands.entity(entity).remove::<Rolling>();
            player.roll_timer.reset();
        }

        // Tick the invulnerable timer if the player is immune
        if is_immune {
            player.roll_invulnerable_timer.tick(time.delta());
        }
        // If invulnerability timer is finished, remove the immune component from the player and reset the cooldown timer
        if player.roll_invulnerable_timer.finished() && is_immune {
            commands.entity(entity).remove::<Immune>();
            player.roll_invulnerable_timer.reset();
        }
    }
}

pub fn player_rolling_behavior_system(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Rolling, &Creature, &Player)>,
) {
    for (mut velocity, rolling, creature, player) in query.iter_mut() {
        if rolling.direction.length() > 0.0 {
            let acceleration_vector = rolling.direction
                * creature.acceleration
                * player.roll_speed_multiplier
                * time.delta_seconds();
            velocity.value += acceleration_vector;

            // Limit maximum speed
            velocity.value = velocity
                .value
                .clamp_length_max(creature.max_speed * player.roll_speed_multiplier);
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_size = PIXELS_PER_METER * 2.0;

    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        sprites.player.clone(),
        Vec2::new(128.0, 80.0),
        4,
        1,
        None,
        None,
    ));

    let animation = Animated {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        first: 0,
        last: 0,
        ..default()
    };

    let _player_entity = commands.spawn((
        PlayerBundle {
            unit_bundle: CreatureBundle {
                sprite: SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation.first),
                    transform: Transform::from_xyz(300.0, 200.0, 0.0),
                    ..default()
                },
                animation,
                collider: Collider::ball(sprite_size / 3.0),
                zsort: ZSort {
                    offset_y: -(sprite_size / 2.0 - 20.0),
                },
                creature: Creature {
                    max_speed: 128.0,
                    acceleration: 2048.0,
                    friction: 512.0,
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        InputManagerBundle {
            input_map: InputMap::default()
                .insert(VirtualDPad::wasd(), PlayerAction::Move)
                .insert(VirtualDPad::arrow_keys(), PlayerAction::Move)
                .insert(KeyCode::Z, PlayerAction::Attack)
                .insert(KeyCode::O, PlayerAction::Attack)
                .insert(GamepadButtonType::South, PlayerAction::Attack)
                .insert(GamepadButtonType::West, PlayerAction::Attack)
                .insert(KeyCode::X, PlayerAction::Roll)
                .insert(KeyCode::P, PlayerAction::Roll)
                .insert(GamepadButtonType::East, PlayerAction::Roll)
                .insert(GamepadButtonType::North, PlayerAction::Roll)
                .build(),
            ..default()
        },
    ));
}

fn player_damage_system(
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
    mut enemy_hitbox_query: Query<(Entity, &Transform, &mut Creature, &Collider), With<Enemy>>,
    mut player_hurtbox_query: Query<(Entity, &Transform, &Collider, &PlayerHurtboxDamage)>,
) {
    for (enemy_hitbox_entity, enemy_transform, _enemy_creature, _enemy_collider) in
        enemy_hitbox_query.iter_mut()
    {
        for (player_hurtbox_entity, player_transform, _player_collider, player_hurtbox_damage) in
            player_hurtbox_query.iter_mut()
        {
            if rapier_context.intersection_pair(player_hurtbox_entity, enemy_hitbox_entity)
                == Some(true)
            {
                println!("Player hit enemy for {} damage", player_hurtbox_damage.0);
                // Get direction to knock enemy back
                commands.entity(enemy_hitbox_entity).insert(DealDamage {
                    amount: player_hurtbox_damage.0 as f32,
                    knockback_direction: (enemy_transform.translation.truncate()
                        - player_transform.translation.truncate())
                    .normalize_or_zero(),
                    knockback_force: 100.0,
                });
            }
        }
    }
}
