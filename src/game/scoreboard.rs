use bevy::prelude::*;

pub fn scoreboard(font: Handle<Font>) -> impl Bundle {
    let style = TextStyle {
        font,
        font_size: 50.,
        color: Color::WHITE,
    };
    let alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        ..default()
    };

    Text2dBundle {
        text: Text::with_section("Scoreboard", style, alignment),
        transform: Transform::from_xyz(0., 320., 0.),
        ..default()
    }
}
