use bevy::prelude::*;
use smooth_orbit_camera::{Orbit, OrbitPlugin, Orbiting};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, OrbitPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            // Turning off culling keeps the plane visible when viewed from beneath.
            cull_mode: None,
            ..default()
        })),
    ));

    let target = commands
        .spawn((
            Name::new("Cube"),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(1.5, 0.51, 1.5),
        ))
        .id();

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Orbit::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Orbiting(target),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));
}
