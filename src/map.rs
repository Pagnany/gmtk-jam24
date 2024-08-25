use bevy::prelude::*;
use rand::Rng;

#[derive(PartialEq, Eq)]
pub enum BlockType {
    /// for Mouse
    /// 60 x 60
    MouseHole,
    /// for Kangaroo
    Tree,
    /// for Elephant
    WoodWall,
}

/// wall: 720 x 60
#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Blockage {
    block_type: BlockType,
}

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
        let rand_object = rng.gen_range(1..=10);

        let left = -1.0 * crate::SCREEN_WIDTH / 2.0 + 100.0;
        let right = crate::SCREEN_WIDTH / 2.0 - 100.0;
        let rand_x = rng.gen_range(left..right);
        let mut gap_size = 100.0;
        let mut wall_left_x = 0.0;
        let mut wall_right_x = 0.0;

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
                            wall_left_x + 360.0 + 30.0,
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
                //Tree
                gap_size = 60.0;
                wall_left_x = rand_x - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = rand_x + (720.0 / 2.0) + (gap_size / 2.0);
            }
            3 => {
                //WoodWall
                gap_size = 60.0;
                wall_left_x = rand_x - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = rand_x + (720.0 / 2.0) + (gap_size / 2.0);
            }
            4 => {
                //Water
                gap_size = 60.0;
                wall_left_x = rand_x - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = rand_x + (720.0 / 2.0) + (gap_size / 2.0);
            }
            5..10 => {
                gap_size = rng.gen_range(60.0..300.0);
                wall_left_x = rand_x - (720.0 / 2.0) - (gap_size / 2.0);
                wall_right_x = rand_x + (720.0 / 2.0) + (gap_size / 2.0);
            }
            _ => (),
        }

        //println!("left: {} right: {}", left, right);
        println!("rand: {}", rand_x);
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
