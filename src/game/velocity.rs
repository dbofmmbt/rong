use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec3);

pub fn velocity_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0;
    }
}
