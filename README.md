# Bevy Smooth Orbit Camera

A smooth orbit camera plugin for the Bevy game engine that provides intuitive camera controls for 3D scenes. Perfect for
inspection views, model viewers, and interactive 3D applications.

## Features

- **Smooth Orbit Controls**: Click and drag to orbit around a target entity
- **Zoom Support**: Mouse wheel zooming with configurable limits
- **Friction System**: Natural movement decay when not actively dragging
- **Pitch Constraints**: Configurable vertical rotation limits
- **Entity Targeting**: Orbit around any entity in your scene
- **Customizable Parameters**: Speed, friction, zoom limits, and more

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
smooth_orbit_camera = { git = "https://github.com/FloppyDisck/bevy_smooth_orbit_camera.git" }
bevy = "0.16.0"
```

## Quick Start

```rust
use bevy::prelude::*;
use smooth_orbit_camera::{Orbit, OrbitPlugin, Orbiting};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, OrbitPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a target object to orbit around
    let target = commands
        .spawn((
            Name::new("Cube"),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ))
        .id();

    // Spawn the orbit camera
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Orbit::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Orbiting(target), // Target entity to orbit around
    ));
}
```

## Controls

- **Left Mouse Button + Drag**: Orbit around the target
- **Mouse Wheel**: Zoom in/out

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## Compatibility

| bevy_smooth_orbit_camera | Bevy |
|--------------------------|------|
| 0.1.0                    | 0.16 |