use crate::{
    map::{get_block_size, BlockType, Blockage, Wall},
    player::{get_player_size, Animal, Player},
};
use bevy::prelude::*;

pub fn check_collision_wall(
    mut query_player: Query<(&Player, &Transform)>,
    query_mapobject: Query<(&Transform, &Wall)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    let player = query_player.single_mut();
    let player_x = player.1.translation.x;
    let player_y = player.1.translation.y;
    let (width, height) = get_player_size(&player.0.animal);

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

pub fn check_collision_blockage(
    mut query_player: Query<(&Player, &Transform)>,
    query_blockage: Query<(Entity, &Transform, &Blockage)>,
    mut next_state: ResMut<NextState<crate::GameState>>,
) {
    let player = query_player.single_mut();
    let player_x = player.1.translation.x;
    let player_y = player.1.translation.y;
    let (width, height) = get_player_size(&player.0.animal);

    for (_entity, transform, block_obj) in &query_blockage {
        let block_obj_x = transform.translation.x;
        let block_obj_y = transform.translation.y;
        let (block_width, block_height) = get_block_size(&block_obj.block_type);

        if player_x + width / 2.0 > block_obj_x - block_width / 2.0
            && player_x - width / 2.0 < block_obj_x + block_width / 2.0
            && player_y + height / 2.0 > block_obj_y - block_height / 2.0
            && player_y - height / 2.0 < block_obj_y + block_height / 2.0
        {
            match player.0.animal {
                Animal::Mouse => {
                    if block_obj.block_type != BlockType::MouseHole {
                        next_state.set(crate::GameState::GameOver);
                    }
                }
                Animal::Dog => {
                    next_state.set(crate::GameState::GameOver);
                }
                Animal::Kangaroo => {
                    if block_obj.block_type != BlockType::GroundHole {
                        next_state.set(crate::GameState::GameOver);
                    }
                }
                Animal::Elephant => {
                    if block_obj.block_type != BlockType::WoodWall {
                        next_state.set(crate::GameState::GameOver);
                    }
                }
                Animal::Whale => {
                    if block_obj.block_type != BlockType::Water {
                        next_state.set(crate::GameState::GameOver);
                    }
                }
            }
        }
    }
}
