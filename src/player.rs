use bevy::prelude::*;

pub const MOVESPEED: f32 = 250.0;
pub const COOLDOWN: f32 = 0.5;

// mouse: 20 x 20
// dog: 40 x 100
// kangaroo: 40 x 60
// elephant: 150 x 200
// whale: 300 x 500
#[derive(PartialEq, Eq)]
pub enum Animal {
    Mouse,
    Dog,
    Kangaroo,
    Elephant,
    Whale,
}

#[derive(Component)]
pub struct Player {
    pub animal: Animal,
    pub change_key_donw: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct CooldownTimer(pub Timer);

pub fn player_change_animal(
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Handle<Image>, &mut CooldownTimer)>,
) {
    let mut player = query.single_mut();
    player.2.tick(time.delta());

    if !keys.pressed(KeyCode::KeyU) && !keys.pressed(KeyCode::KeyJ) {
        player.0.change_key_donw = false;
    }

    if player.2.finished() && !player.0.change_key_donw {
        if keys.pressed(KeyCode::KeyU) && player.0.animal != Animal::Whale {
            let mut handle = player.1;
            match player.0.animal {
                Animal::Mouse => {
                    player.0.animal = Animal::Dog;
                    *handle = asset_server.load("textures/dog_01.png");
                }
                Animal::Dog => {
                    player.0.animal = Animal::Kangaroo;
                    *handle = asset_server.load("textures/kangaroo_01.png");
                }
                Animal::Kangaroo => {
                    player.0.animal = Animal::Elephant;
                    *handle = asset_server.load("textures/elephant_01.png");
                }
                Animal::Elephant => {
                    player.0.animal = Animal::Whale;
                    *handle = asset_server.load("textures/whale_01.png");
                }
                _ => (),
            }
            player.2.reset();
            player.0.change_key_donw = true;
        } else if keys.pressed(KeyCode::KeyJ) && player.0.animal != Animal::Mouse {
            let mut handle = player.1;
            match player.0.animal {
                Animal::Whale => {
                    player.0.animal = Animal::Elephant;
                    *handle = asset_server.load("textures/elephant_01.png");
                }
                Animal::Elephant => {
                    player.0.animal = Animal::Kangaroo;
                    *handle = asset_server.load("textures/kangaroo_01.png");
                }
                Animal::Kangaroo => {
                    player.0.animal = Animal::Dog;
                    *handle = asset_server.load("textures/dog_01.png");
                }
                Animal::Dog => {
                    player.0.animal = Animal::Mouse;
                    *handle = asset_server.load("textures/mouse_01.png");
                }
                _ => (),
            }
            player.2.reset();
            player.0.change_key_donw = true;
        }
    }
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
