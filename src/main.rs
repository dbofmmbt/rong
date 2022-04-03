use bevy::prelude::*;
use game::GamePlugin;
use util::default;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "RONG".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

mod game;
pub(crate) mod util;
