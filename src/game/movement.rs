use bevy::prelude::*;
use heron::prelude::*;
#[derive(Debug, Component, Default)]
pub struct MoveControls {
    pub up: Option<KeyCode>,
    pub down: Option<KeyCode>,
    pub left: Option<KeyCode>,
    pub right: Option<KeyCode>,
}

pub fn movement_system(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    query: Query<(Entity, &Transform, &MoveControls)>,
) {
    let speed = 1000.;

    for (entity, transform, controls) in query.iter() {
        macro_rules! respond_to_input {
            ($($control:tt)*) => {
                $(
                    if let Some(control) = controls.$control {
                        if input.just_pressed(control) {
                            let $control = transform.$control();
                            commands.entity(entity).insert(Velocity::from($control * speed));
                        }

                        else if input.just_released(control) {
                            commands.entity(entity).remove::<Velocity>();
                        }
                    }
                )*
            };
        }
        respond_to_input! {up down left right };
    }
}
