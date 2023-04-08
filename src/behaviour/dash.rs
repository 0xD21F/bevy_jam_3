

#[derive(Component, Reflect)]
pub struct Dash {
    pub range: f32, // how far the enemy can dash towards the player
    pub cooldown: Timer, // how long before the enemy can dash again
}

impl Default for Dash {
    fn default() -> Self {
        Self {
            range: 500.0,
            cooldown: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
        }
    }
}

fn dash_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Enemy, &mut Velocity, &mut Dash, &Transform), Without<Player>>,
) {
    let player_position = player_query.single().translation;

    for (mut _enemy, mut velocity, mut dash, transform) in enemy_query.iter_mut() {
        dash.cooldown.tick(time.delta());
        if dash.cooldown.just_finished() {
            dash.cooldown.pause();
            let distance = (player_position - transform.translation).length();

            if distance <= dash.range {
                dash.cooldown.unpause();
                let direction = (player_position - transform.translation).normalize();
                velocity.value = Vec2::new(direction.x * 1000.0, direction.y * 1000.0);
            }
        }
    }
}

