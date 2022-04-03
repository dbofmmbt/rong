use bevy::prelude::*;

use self::pad::pad_bundle;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(hello).add_startup_system(setup);
    }
}

fn hello() {
    info!("Hello, world!");
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(pad_bundle(-500.0, 0.0));
    commands.spawn_bundle(pad_bundle(500.0, 0.0));
}

mod pad;
