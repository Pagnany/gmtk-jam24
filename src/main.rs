use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod collision;
pub mod map;
pub mod player;
pub mod system;

pub const SCREEN_WIDTH: f32 = 720.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
const TICK_TIME: f64 = 1.0 / 50.0;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    LoadingScreen,
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "animal scale".into(),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
        FrameTimeDiagnosticsPlugin,
    ));
    app.insert_resource(Time::<Fixed>::from_seconds(TICK_TIME));
    app.add_systems(
        FixedUpdate,
        (
            player::player_movement,
            player::player_change_animal,
            map::move_map,
            map::check_spawn_destroy_map_objects,
            system::kill_game_on_esc,
            system::fps_update_system,
        ),
    );
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                ..Default::default()
            }),
        ]),
        system::FpsText,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/dog_01.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        player::Player {
            animal: player::Animal::Dog,
            change_key_donw: false,
        },
        player::CooldownTimer(Timer::from_seconds(0.1, TimerMode::Once)),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/wall_01.png"),
            transform: Transform::from_xyz(0.0, SCREEN_HEIGHT / 2.0, 0.0),
            ..default()
        },
        map::MapObject,
        map::Wall,
    ));
}
