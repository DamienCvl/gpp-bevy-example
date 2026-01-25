use bevy::{
    ecs::system::{Query, Single},
    transform::components::Transform,
    window::Window,
};

use crate::{components::Mover, constants::WALL_PADDING};

pub fn wall_collision_system(mut query: Query<(&Transform, &mut Mover)>, window: Single<&Window>) {
    let width = window.width() / 2.0 - WALL_PADDING;
    let height = window.height() / 2.0 - WALL_PADDING;

    for (transform, mut mover) in query.iter_mut() {
        let pos = transform.translation;

        // Left wall
        if pos.x < -width && mover.velocity.x < 0.0 {
            mover.velocity.x = mover.velocity.x.abs();
        }
        // Right wall
        else if pos.x > width && mover.velocity.x > 0.0 {
            mover.velocity.x = -mover.velocity.x.abs();
        }

        // Bottom wall
        if pos.y < -height && mover.velocity.y < 0.0 {
            mover.velocity.y = mover.velocity.y.abs();
        }
        // Top wall
        else if pos.y > height && mover.velocity.y > 0.0 {
            mover.velocity.y = -mover.velocity.y.abs();
        }
    }
}
