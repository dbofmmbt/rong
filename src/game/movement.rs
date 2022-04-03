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
    let speed = 10.;

    for (mut transform, controls) in query.iter_mut() {
        macro_rules! respond_to_input {
            ($($control:tt)*) => {
                $(
                    if input.pressed(controls.$control) {
                        let $control = transform.$control();
                        transform.translation += $control * speed;
                    }
                )*
            };
        }
        respond_to_input! {up down left right };
    }
}
