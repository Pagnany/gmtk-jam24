use bevy::prelude::*;

pub const MOVESPEED: f32 = 250.0;

// mouse: 20 x 20
// dog: 40 x 100
// kangaroo: 40 x 60
#[derive(Default)]
pub enum Animal {
    Mouse,
    #[default]
    Dog,
    Kangaroo,
    Elephant,
    Whale,
}

#[derive(Default, Component)]
pub struct Player {
    pub animal: Animal,
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let mut player = query.single_mut();

    if keys.pressed(KeyCode::KeyW) {
        player.1.translation.y += time.delta_seconds() * MOVESPEED;
    }
    if keys.pressed(KeyCode::KeyA) {
        player.1.translation.x -= time.delta_seconds() * MOVESPEED;
    }
    if keys.pressed(KeyCode::KeyS) {
        player.1.translation.y -= time.delta_seconds() * MOVESPEED;
    }
    if keys.pressed(KeyCode::KeyD) {
        player.1.translation.x += time.delta_seconds() * MOVESPEED;
    }

    if player.1.translation.x < -1.0 * crate::SCREEN_WIDTH / 2.0 {
        player.1.translation.x = -1.0 * crate::SCREEN_WIDTH / 2.0;
    } else if player.1.translation.x > crate::SCREEN_WIDTH / 2.0 {
        player.1.translation.x = crate::SCREEN_WIDTH / 2.0;
    }
    if player.1.translation.y < -1.0 * crate::SCREEN_HEIGHT / 2.0 {
        player.1.translation.y = -1.0 * crate::SCREEN_HEIGHT / 2.0;
    } else if player.1.translation.y > crate::SCREEN_HEIGHT / 2.0 {
        player.1.translation.y = crate::SCREEN_HEIGHT / 2.0;
    }
}
