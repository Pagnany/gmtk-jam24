use bevy::prelude::*;
use rand::Rng;

/// wall: 720 x 60
#[derive(Component)]
pub struct Wall;

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
        let left = -1.0 * crate::SCREEN_WIDTH / 2.0;
        let right = crate::SCREEN_WIDTH / 2.0;
        let rand_x = rng.gen_range(left..right);
        let rand_gap_size = rng.gen_range(60.0..300.0);

        let wall_left_x = rand_x - (720.0 / 2.0) - (rand_gap_size / 2.0);
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

        let wall_right_x = rand_x + (720.0 / 2.0) + (rand_gap_size / 2.0);
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
