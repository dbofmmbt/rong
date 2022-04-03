use bevy::prelude::*;

use self::{
    movement::{movement_system, MoveControls},
    pad::pad_bundle,
    scoreboard::scoreboard,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello)
            .add_startup_system(setup)
            .add_system(movement_system);
    }
}

fn hello() {
    info!("Hello, world!");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load("fonts/ComicNeue-Regular.ttf");
    commands.spawn_bundle(scoreboard(font));

    commands
        .spawn_bundle(pad_bundle(500.0, 0.0))
        .insert(MoveControls {
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
        });
    commands
        .spawn_bundle(pad_bundle(-500.0, 0.0))
        .insert(MoveControls {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
        });
}

mod movement;
mod pad;
mod scoreboard;
