use bevy::prelude::*;

use self::{
    ball::ball_bundle,
    collision::{collision_detection, collision_log, CollisionEvent},
    movement::{movement_system, MoveControls},
    pad::pad_bundle,
    scoreboard::scoreboard,
    velocity::{velocity_system, Velocity},
    wall::create_window_boundaries,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello)
            .add_startup_system(setup)
            .add_event::<CollisionEvent>()
            .add_system(collision_detection)
            .add_system(collision_log)
            .add_system(movement_system)
            .add_system(velocity_system);
    }
}

fn hello() {
    info!("Hello, world!");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load("fonts/ComicNeue-Regular.ttf");
    commands.spawn_bundle(scoreboard(font));

    create_window_boundaries(&mut commands);

    commands
        .spawn_bundle(ball_bundle(0., 0.))
        .insert(Velocity(Vec3::new(0., 0., 0.)));

    commands
        .spawn_bundle(pad_bundle(500.0, 0.0))
        .insert(MoveControls {
            up: Some(KeyCode::Up),
            down: Some(KeyCode::Down),
            ..default()
        });
    commands
        .spawn_bundle(pad_bundle(-500.0, 0.0))
        .insert(MoveControls {
            up: Some(KeyCode::W),
            down: Some(KeyCode::S),
            ..default()
        });
}

mod ball;
mod collision;
mod movement;
mod pad;
mod scoreboard;
mod velocity;
mod wall;
