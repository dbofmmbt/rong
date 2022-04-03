use bevy::prelude::*;

use self::{pad::pad_bundle, scoreboard::scoreboard};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello).add_startup_system(setup);
    }
}

fn hello() {
    info!("Hello, world!");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load("fonts/ComicNeue-Regular.ttf");
    commands.spawn_bundle(scoreboard(font));

    commands.spawn_batch([pad_bundle(-500.0, 0.0), pad_bundle(500.0, 0.0)]);
}

mod pad;
mod scoreboard;
