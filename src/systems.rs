use crate::orbit::{Orbit, OrbitedBy, Orbiting};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::{
    BevyError, ButtonInput, EulerRot, GlobalTransform, MouseButton, Quat, Query, Res, Time,
    Transform, With,
};

pub fn update_orbit(
    mut orbit: Query<(&mut Orbit, &mut Transform, &Orbiting)>,
    target_query: Query<&GlobalTransform, With<OrbitedBy>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) -> Result<(), BevyError> {
    for (mut orbit, mut camera, orbiting) in orbit.iter_mut() {
        let target = target_query.get(**orbiting)?.translation();

        // Handle dragging
        if mouse_buttons.just_pressed(MouseButton::Left) {
            orbit.dragging = true;
        } else if mouse_buttons.just_released(MouseButton::Left) {
            orbit.dragging = false;
        }

        // Update orbit movement
        if orbit.dragging {
            let speed = time.delta_secs() * orbit.speed;
            let delta = mouse_motion.delta * speed;

            orbit.movement = delta.normalize_or_zero() * (delta.length().min(orbit.max_speed))
        } else {
            // Friction
            let friction = orbit.friction * time.delta_secs();
            let new_distance = (orbit.movement.length() - friction).max(0.0);
            orbit.movement = orbit.movement.normalize_or_zero() * new_distance;
        }

        // Handle zooming
        let scroll_delta = mouse_wheel.delta.y * orbit.zoom_steps;
        let (zoom_min, zoom_max) = orbit.zoom_limits;
        orbit.zoom = (orbit.zoom + scroll_delta).clamp(zoom_min, zoom_max);

        let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

        let (pitch_min, pitch_max) = orbit.pitch_limits;
        camera.rotation = Quat::from_euler(
            EulerRot::YXZ,
            yaw + orbit.movement.x,
            (pitch + orbit.movement.y).clamp(pitch_min, pitch_max),
            roll,
        );

        camera.translation = target + camera.back() * orbit.zoom;
    }

    Ok(())
}
