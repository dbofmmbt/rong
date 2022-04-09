use bevy::prelude::*;

#[derive(Debug, Component, Default)]
pub struct MoveControls {
    pub up: Option<KeyCode>,
    pub down: Option<KeyCode>,
    pub left: Option<KeyCode>,
    pub right: Option<KeyCode>,
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
                    if let Some(control) = controls.$control {
                        if input.pressed(control) {
                            let $control = transform.$control();
                            transform.translation += $control * speed;
                        }
                    }
                )*
            };
        }
        respond_to_input! {up down left right };
    }
}
