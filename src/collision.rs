use bevy::prelude::*;

use crate::{map::MapObject, player::Player};

pub fn check_collsion(
    mut query_player: Query<(&Player, &Transform)>,
    query_mapobject: Query<(&Transform, &MapObject)>,
) {
    let player = query_player.single_mut();

    for map_obj in &query_mapobject {}
}
