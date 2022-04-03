use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct MoveControls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

pub fn movement_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &MoveControls)>,
) {
    macro_rules! control {
        ($input:ident, $controls:ident, $transform:ident, $speed:ident | $($control:ident)*) => {
            $(
                if $input.pressed($controls.$control) {
                    let $control = $transform.$control();
                    $transform.translation += $control * $speed;
                }
            )*
        };
    }

    let speed = 10.;

    for (mut transform, controls) in query.iter_mut() {
        control!(input, controls, transform, speed | up down left right);
    }
}
