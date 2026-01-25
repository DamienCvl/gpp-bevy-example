use bevy::prelude::*;

use crate::{
    components::{Mover, Obstacle, Target},
    systems::{
        apply_movement_system, avoidance_system, goto_target_system, target_reached_system, wall_collision_system
    },
};

mod components;
mod systems;

mod constants {
    pub const ENTITY_SPEED: f32 = 30.0;
    pub const ENTITY_ACCELERATION_TIME_TOWARD_TARGET: f32 = 0.2;
    pub const ENTITY_DESELERATION_TIME_AFTER_TARGET_REACHED: f32 = 0.1;

    pub const AVOID_FORCE: f32 = 400.0;
    pub const AVOID_RADIUS: f32 = 12.0;
    pub const TARGET_REACHED_DISTANCE: f32 = 50.0;

    pub const WALL_PADDING: f32 = 20.0;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                apply_movement_system,
                target_reached_system,
                goto_target_system,
                avoidance_system,
                wall_collision_system,
                draw_gizmos_system,
            ),
        )
        .run();
}

fn setup(
    mut gizmo_config_store: ResMut<GizmoConfigStore>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    gizmo_config_store
        .config_mut::<DefaultGizmoConfigGroup>()
        .0
        .line
        .width = 1.0;

    commands.spawn(Camera2d);

    let target_shape = meshes.add(Circle::new(10.0));
    let target_color = materials.add(Color::srgb(1.0, 0.0, 0.0));

    let target_left = commands
        .spawn((
            Mesh2d(target_shape.clone()),
            MeshMaterial2d(target_color.clone()),
            Transform::from_xyz(-400.0, 0.0, 0.0),
            Target,
        ))
        .id();

    let target_right = commands
        .spawn((
            Mesh2d(target_shape.clone()),
            MeshMaterial2d(target_color.clone()),
            Transform::from_xyz(400.0, 0.0, 0.0),
            Target,
        ))
        .id();

    let mover_shape = meshes.add(CircularSector::new(10.0, 22.5_f32.to_radians()));
    let mover_color = materials.add(Color::srgb(0.0, 0.8, 0.8));

    // Spawn left to right movers
    commands.spawn_batch(core::array::from_fn::<_, 5, _>(|i| {
        let y = -180.0 + (i as f32) * 100.0;
        (
            Mesh2d(mover_shape.clone()),
            MeshMaterial2d(mover_color.clone()),
            Transform::from_isometry(Isometry3d::new(
                Vec3::new(-330.0, y, 0.0),
                Quat::from_rotation_z(-90_f32.to_radians()),
            )),
            Mover {
                acceleration: Vec2::ZERO,
                velocity: Vec2::ZERO,
                target: Some(target_right),
            },
            Obstacle {
                radius: constants::AVOID_RADIUS,
            },
        )
    }));

    // Spawn right to left movers
    commands.spawn_batch(core::array::from_fn::<_, 5, _>(|i| {
        let y = -200.0 + (i as f32) * 100.0;
        (
            Mesh2d(mover_shape.clone()),
            MeshMaterial2d(mover_color.clone()),
            Transform::from_isometry(Isometry3d::new(
                Vec3::new(250.0, y, 0.0),
                Quat::from_rotation_z(90_f32.to_radians()),
            )),
            Mover {
                acceleration: Vec2::ZERO,
                velocity: Vec2::ZERO,
                target: Some(target_left),
            },
            Obstacle {
                radius: constants::AVOID_RADIUS,
            },
        )
    }));
}

fn draw_gizmos_system(mut gizmos: Gizmos, mover_query: Query<(&Transform, &Obstacle)>) {
    for (transform, obstacle) in mover_query.iter() {
        gizmos.circle_2d(
            transform.translation.xy(),
            obstacle.radius,
            Color::srgb(0.0, 1.0, 0.0),
        );
    }
}
