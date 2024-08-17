use bevy::prelude::*;

use crate::{map::MapObject, player::Player};

pub fn check_collsion(
    mut query_player: Query<(&Player, &Transform)>,
    query_mapobject: Query<(&Transform, &MapObject)>,
) {
    let player = query_player.single_mut();
    let player_x = player.1.translation.x;
    let player_y = player.1.translation.y;
    let mut width = 0.0;
    let mut height = 0.0;
    match player.0.animal {
        crate::player::Animal::Mouse => {
            width = 20.0;
            height = 20.0;
        }
        crate::player::Animal::Dog => {
            width = 40.0;
            height = 100.0;
        }
        crate::player::Animal::Kangaroo => {
            width = 40.0;
            height = 60.0;
        }
        crate::player::Animal::Elephant => {
            width = 150.0;
            height = 200.0;
        }
        crate::player::Animal::Whale => {
            width = 300.0;
            height = 500.0;
        }
    }

    for (transform, map_obj) in &query_mapobject {
        if player_x + (width / 2.0) < transform.translation.x + 360.0
            && player_x - (width / 2.0) > transform.translation.x - 360.0
            && player_y + (height / 2.0) < transform.translation.y + 30.0
            && player_y - (height / 2.0) > transform.translation.y - 30.0
        {
            println!("Overlapp: {}", width);
        }
    }
}
