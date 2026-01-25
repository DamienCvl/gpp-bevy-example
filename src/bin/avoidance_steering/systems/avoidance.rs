use bevy::{
    ecs::{entity::Entity, system::Query},
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};

use crate::{
    components::{Mover, Obstacle},
    constants::AVOID_FORCE,
};

pub fn avoidance_system(
    mut mover_query: Query<(Entity, &Transform, &mut Mover, Option<&Obstacle>)>,
    obstacle_query: Query<(Entity, &Transform, &Obstacle)>,
) {
    for (mover_entity, mover_transform, mut mover, mover_obstacle) in mover_query.iter_mut() {
        let mut avoid_forces = Vec2::ZERO;
        let mut obstacle_count = 0;

        for (obstacle_entity, obstacle_transform, obstacle) in obstacle_query.iter() {
            if obstacle_entity == mover_entity {
                continue;
            }

            let diff = mover_transform.translation - obstacle_transform.translation;
            let distance = diff.length();

            let avoid_distance = obstacle.radius + mover_obstacle.map(|o| o.radius).unwrap_or(0.0);

            if distance < avoid_distance {
                let mut force = diff.xy().normalize_or_zero();
                force *= 1.0 - (distance / avoid_distance).powf(2.0);
                avoid_forces += force;
                obstacle_count += 1;
            }
        }

        if obstacle_count > 0 {
            avoid_forces /= obstacle_count as f32;
            avoid_forces *= AVOID_FORCE;
        }

        mover.acceleration += avoid_forces;
    }
}
