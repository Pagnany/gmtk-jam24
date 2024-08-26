use bevy::prelude::*;
use rand::Rng;

#[derive(PartialEq, Eq)]
pub enum BlockType {
    /// for Mouse
    /// 60 x 60
    MouseHole,
    /// for Kangaroo
    /// 100 x 60
    GroundHole,
    /// for Elephant
    /// 200 x 60
    WoodWall,
    // for Whale
    Water,
}

pub fn get_block_size(block_type: &BlockType) -> (f32, f32) {
    match block_type {
        BlockType::MouseHole => (60.0, 60.0),
        BlockType::GroundHole => (100.0, 60.0),
        BlockType::WoodWall => (200.0, 60.0),
        BlockType::Water => (0.0, 0.0),
    }
}

/// wall: 720 x 60
/// for collision
#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Blockage {
    pub block_type: BlockType,
}

/// All objects that move down when playing
#[derive(Component)]
pub struct MapObject;

pub fn move_map(time: Res<Time>, mut query: Query<(&MapObject, &mut Transform)>) {
    for (_map_obj, mut transform) in &mut query {
        transform.translation.y -= 150.0 * time.delta_seconds();
    }
}

pub fn check_spawn_destroy_map_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &MapObject, &Transform)>,
) {
    // checks if a new object should be spawned
    // checks if an object is out of the screen to the bottom
    // and should be despawned
    let mut highes_obj = -1000.0;
    for (entity, _map_obj, transform) in &query {
        if transform.translation.y > highes_obj {
            highes_obj = transform.translation.y;
        }
        if transform.translation.y < -1.0 * crate::SCREEN_HEIGHT / 2.0 - 200.0 {
            commands.entity(entity).despawn();
        }
    }

    if highes_obj <= 100.0 {
        let mut rng = rand::thread_rng();
        let rand_object = rng.gen_range(1..=8);

        // gap pos limit left and right
        // 100px buffer
        let left = -1.0 * crate::SCREEN_WIDTH / 2.0 + 100.0;
        let right = crate::SCREEN_WIDTH / 2.0 - 100.0;

        // gap pos
        let rand_x = rng.gen_range(left..right);

        let gap_size;
        let wall_left_x;
        let wall_right_x;

        match rand_object {
            1 => {
                // MouseHole
                gap_size = 60.0;
                wall_left_x = rand_x - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = rand_x + (720.0 / 2.0) + (gap_size / 2.0);

                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("textures/mouse_hole_01.png"),
                        transform: Transform::from_xyz(
                            wall_left_x + 360.0 + (gap_size / 2.0),
                            crate::SCREEN_HEIGHT / 2.0 + 200.0,
                            0.9,
                        ),
                        ..default()
                    },
                    Blockage {
                        block_type: BlockType::MouseHole,
                    },
                    MapObject,
                    crate::InGameEntity,
                ));
            }
            2 => {
                // GroundHole
                gap_size = 100.0;
                wall_left_x = rand_x - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = rand_x + (720.0 / 2.0) + (gap_size / 2.0);

                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("textures/ground_hole_01.png"),
                        transform: Transform::from_xyz(
                            wall_left_x + 360.0 + (gap_size / 2.0),
                            crate::SCREEN_HEIGHT / 2.0 + 200.0,
                            0.9,
                        ),
                        ..default()
                    },
                    Blockage {
                        block_type: BlockType::GroundHole,
                    },
                    MapObject,
                    crate::InGameEntity,
                ));
            }
            3 => {
                // WoodWall
                gap_size = 200.0;
                wall_left_x = rand_x - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = rand_x + (720.0 / 2.0) + (gap_size / 2.0);

                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("textures/wood_wall_01.png"),
                        transform: Transform::from_xyz(
                            wall_left_x + 360.0 + (gap_size / 2.0),
                            crate::SCREEN_HEIGHT / 2.0 + 200.0,
                            0.9,
                        ),
                        ..default()
                    },
                    Blockage {
                        block_type: BlockType::WoodWall,
                    },
                    MapObject,
                    crate::InGameEntity,
                ));
            }
            4 => {
                // Water
                // always in the middle
                gap_size = 650.0;
                wall_left_x = 0.0 - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = 0.0 + (720.0 / 2.0) + (gap_size / 2.0);
            }
            _ => {
                // Normal Walls
                gap_size = rng.gen_range(60.0..300.0);
                wall_left_x = rand_x - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = rand_x + (720.0 / 2.0) + (gap_size / 2.0);
            }
        }

        //println!(
        //    "rand: {}, gap_size: {}, wall_left: {}, wall_right: {}",
        //    rand_x, gap_size, wall_left_x, wall_right_x
        //);

        // spawn walls left and right
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/wall_02.png"),
                transform: Transform::from_xyz(
                    wall_left_x,
                    crate::SCREEN_HEIGHT / 2.0 + 200.0,
                    0.0,
                ),
                ..default()
            },
            MapObject,
            Wall,
            crate::InGameEntity,
        ));

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("textures/wall_02.png"),
                transform: Transform::from_xyz(
                    wall_right_x,
                    crate::SCREEN_HEIGHT / 2.0 + 200.0,
                    0.0,
                ),
                ..default()
            },
            MapObject,
            Wall,
            crate::InGameEntity,
        ));
    }
}
