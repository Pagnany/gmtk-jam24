use bevy::prelude::*;

use crate::{map::MapObject, player::Player};

pub fn check_collsion(
    mut query_player: Query<(&Player, &Transform)>,
    query_mapobject: Query<(&Transform, &MapObject)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    let player = query_player.single_mut();
    let player_x = player.1.translation.x;
    let player_y = player.1.translation.y;
    let (width, height) = match player.0.animal {
        crate::player::Animal::Mouse => (20.0, 20.0),
        crate::player::Animal::Dog => (40.0, 100.0),
        crate::player::Animal::Kangaroo => (40.0, 60.0),
        crate::player::Animal::Elephant => (150.0, 200.0),
        crate::player::Animal::Whale => (300.0, 500.0),
    };

    for (transform, _map_obj) in &query_mapobject {
        let map_x = transform.translation.x;
        let map_y = transform.translation.y;
        let map_width = 720.0;
        let map_height = 60.0;

        if player_x + width / 2.0 > map_x - map_width / 2.0
            && player_x - width / 2.0 < map_x + map_width / 2.0
            && player_y + height / 2.0 > map_y - map_height / 2.0
            && player_y - height / 2.0 < map_y + map_height / 2.0
        {
            next_state.set(crate::GameState::GameOver);
        }
    }
}
