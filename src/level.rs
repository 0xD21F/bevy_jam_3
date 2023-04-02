use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct LevelElement;

#[derive(Component, Copy, Clone)]
pub enum LevelElementType {
    Rectangle,
}

pub fn build_level(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    level_definition: &[LevelElementDefinition],
) {
    for element in level_definition {
        spawn_level_element(commands, asset_server, element);
    }
}

fn spawn_level_element(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    element: &LevelElementDefinition,
) {
    let texture_handle = asset_server.load("level/debug_element.png");

    let transform = Transform {
        translation: Vec3::new(element.position.x, element.position.y, 0.0),
        rotation: Quat::from_rotation_z(element.angle.to_radians()),
        ..default()
    };

    let sprite_bundle = SpriteBundle {
        texture: texture_handle,
        transform,
        sprite: Sprite {
            custom_size: Some(Vec2::new(element.size.x, element.size.y)),
            ..default()
        },
        ..default()
    };

    let collider = Collider::cuboid(element.size.x / 2.0, element.size.y / 2.0);

    let mut entity_builder = commands.spawn((
        sprite_bundle,
        collider,
        Sensor,
        LevelElement,
        element.element_type,
    ));
}

// TODO:
// - Add support for ramps, different types of level element
// - Add support for level elements with children (ramps....)

pub struct LevelElementDefinition {
    pub position: Vec2,
    pub size: Vec2,
    pub element_type: LevelElementType,
    pub angle: f32,
    pub parent: Option<Entity>,
}

impl Default for LevelElementDefinition {
    fn default() -> Self {
        LevelElementDefinition {
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(100.0, 100.0),
            element_type: LevelElementType::Rectangle,
            angle: 0.0,
            parent: None,
        }
    }
}