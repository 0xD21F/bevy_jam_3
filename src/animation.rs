use bevy::prelude::*;

pub struct SpriteSheetAnimationPlugin;

impl Plugin for SpriteSheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite);
    }
}

#[derive(Component, Default, Clone, Debug, Reflect)]
pub struct Animated {
    pub timer: Timer,
    pub first: usize,
    pub last: usize,
    pub play_once: bool,
}

fn animate_sprite(time: Res<Time>, mut query: Query<(&mut Animated, &mut TextureAtlasSprite)>) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            sprite.index = if sprite.index >= animation.last {
                animation.first
            } else {
                sprite.index + 1
            };
        }
    }
}
