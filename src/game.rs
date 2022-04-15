use bevy::prelude::*;
use heron::prelude::*;

use self::{
    ball::ball_bundle,
    movement::{movement_system, MoveControls},
    pad::pad_bundle,
    scoreboard::scoreboard,
    wall::create_window_boundaries,
};
use crate::util::default;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello)
            .add_startup_system(setup)
            .add_plugin(PhysicsPlugin::default())
            .add_system(movement_system)
            .add_system(detect_collisions);
    }
}

fn detect_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                println!(
                    "Entity {:?} and {:?} started to collide",
                    data1.rigid_body_entity(),
                    data2.rigid_body_entity()
                )
            }
            CollisionEvent::Stopped(data1, data2) => {
                println!(
                    "Entity {:?} and {:?} stopped to collide",
                    data1.rigid_body_entity(),
                    data2.rigid_body_entity()
                )
            }
        }
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

    commands.spawn_bundle(ball_bundle(0., 0.));

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
mod movement;
mod pad;
mod scoreboard;
mod wall;
