use bevy::asset::{AssetMetaCheck, AssetPlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub mod collision;
pub mod map;
pub mod menu;
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

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct GameplaySet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MainMenuSet;

/// Objects spawn when start playing
/// When gameover objects despawn
#[derive(Component)]
struct InGameEntity;

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
    app.insert_state(GameState::MainMenu);
    app.add_systems(
        FixedUpdate,
        (
            (
                player::player_movement,
                player::player_change_animal,
                map::move_map,
                map::check_spawn_destroy_map_objects,
                collision::check_collsion,
            )
                .in_set(GameplaySet),
            (menu::button_system).in_set(MainMenuSet),
            system::kill_game_on_esc,
            system::fps_update_system,
        ),
    );
    app.add_systems(Startup, setup);
    app.add_systems(OnEnter(GameState::MainMenu), menu::spawn_main_menu);
    app.add_systems(OnEnter(GameState::InGame), spawn_ingame);
    app.add_systems(OnExit(GameState::MainMenu), menu::despawn_main_menu);
    app.add_systems(OnExit(GameState::GameOver), menu::despawn_main_menu);
    app.add_systems(
        OnEnter(GameState::GameOver),
        (menu::spawn_gameover_menu, despawn_ingame),
    );
    app.configure_sets(
        FixedUpdate,
        (
            GameplaySet.run_if(in_state(GameState::InGame)),
            MainMenuSet
                .run_if(in_state(GameState::MainMenu).or_else(in_state(GameState::GameOver))),
        ),
    );
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
}

fn spawn_ingame(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("textures/dog_01.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        player::Player {
            animal: player::Animal::Dog,
            change_key_down: false,
        },
        player::CooldownTimer(Timer::from_seconds(0.1, TimerMode::Once)),
        InGameEntity,
    ));
}

fn despawn_ingame(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
