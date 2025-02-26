use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::{
    capture::take_snapshot,
    components::OrbitCamera,
    resource::{LiveCameraPanNumber, OperationWindowRelatedEntities},
    states::OperationState,
};

// orbit camera that was control by user
pub fn interactive_orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    current_operation_state: Res<State<OperationState>>,
    commands: Commands,
    counter: Local<u32>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
) {
    let orbit_query = query.get_single_mut();
    const CAMERA_POSITION_SENSITIVITY: f32 = 0.001;

    match orbit_query {
        Ok((mut transform, mut orbit)) => {
            if orbit.window == operation_window.window.unwrap() {
                // Handle left mouse button for drag
                if buttons.just_pressed(MouseButton::Left) {
                    orbit.is_dragging = true;
                }
                if buttons.just_released(MouseButton::Left) {
                    orbit.is_dragging = false;
                }

                // Orbiting when dragging
                if orbit.is_dragging {
                    for ev in motion_evr.read() {
                        let sensitivity = 0.0025;
                        orbit.yaw -= ev.delta.x * sensitivity;
                        orbit.pitch += ev.delta.y * sensitivity;

                        // Clamp pitch to avoid flipping
                        orbit.pitch = orbit.pitch.clamp(-1., 1.);
                    }
                }

                // Zoom with scroll wheel
                for ev in scroll_evr.read() {
                    orbit.radius -= ev.y * 0.5;
                    // orbit.radius = orbit.radius.clamp(2.0, 50.0);
                }

                // Calculate new camera position
                let yaw_rot = Quat::from_rotation_y(orbit.yaw);
                let pitch_rot = Quat::from_rotation_x(orbit.pitch);
                let offset = yaw_rot * pitch_rot * Vec3::new(0.0, 0.0, orbit.radius);

                // Reposition camera
                if keys.just_pressed(KeyCode::KeyW) {
                    println!(
                        "Moved orbit camera upward by {}",
                        CAMERA_POSITION_SENSITIVITY
                    );
                    orbit.position_y += CAMERA_POSITION_SENSITIVITY;
                } else if keys.just_pressed(KeyCode::KeyS) {
                    println!(
                        "Moved orbit camera downward by {}",
                        CAMERA_POSITION_SENSITIVITY
                    );
                    orbit.position_y -= CAMERA_POSITION_SENSITIVITY;
                } else if keys.just_pressed(KeyCode::KeyD) {
                    println!(
                        "Moved orbit camera to the right by {}",
                        CAMERA_POSITION_SENSITIVITY
                    );
                    orbit.position_x += CAMERA_POSITION_SENSITIVITY;
                } else if keys.just_pressed(KeyCode::KeyA) {
                    println!(
                        "Moved orbit camera to the left by {}",
                        CAMERA_POSITION_SENSITIVITY
                    );
                    orbit.position_x -= CAMERA_POSITION_SENSITIVITY;
                }

                transform.translation = Vec3::new(orbit.position_x, orbit.position_y, 0.0) + offset;
                transform.look_at(Vec3::new(orbit.position_x, orbit.position_y, 0.0), Vec3::Y);
                let c_o_s = current_operation_state.as_ref().get();
                if *c_o_s == OperationState::LiveCapture {
                    take_snapshot(commands, counter, operation_window);
                }
            }
        }
        Err(_) => {
            return;
        }
    }
}

// orbit camera that was control by system, user can't intefere when it was running
pub fn live_orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    current_operation_state: Res<State<OperationState>>,
    mut live_camera_pan_number: ResMut<LiveCameraPanNumber>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
) {
    let orbit_query = query.get_single_mut();

    match orbit_query {
        Ok((mut transform, mut orbit)) => {
            let c_o_s = current_operation_state.as_ref().get();
            if orbit.window == operation_window.window.unwrap()
                && *c_o_s == OperationState::LivePreview
            {
                orbit.yaw += live_camera_pan_number.yaw * 0.002;
                if orbit.yaw >= 0.75 {
                    orbit.yaw = 0.75;
                    live_camera_pan_number.yaw *= -1.0;
                } else if orbit.yaw <= -0.75 {
                    orbit.yaw = -0.75;
                    live_camera_pan_number.yaw *= -1.0;
                }

                orbit.pitch += live_camera_pan_number.pitch * 0.001;
                if orbit.pitch >= -0.25 {
                    orbit.pitch = -0.25;
                    live_camera_pan_number.pitch *= -1.0;
                } else if orbit.pitch <= -0.70 {
                    orbit.pitch = -0.70;
                    live_camera_pan_number.pitch *= -1.0;
                }

                orbit.radius += live_camera_pan_number.radius * 0.001;
                if orbit.radius >= 7.0 {
                    orbit.radius = 7.0;
                    live_camera_pan_number.radius *= -1.0;
                } else if orbit.radius <= 3.0 {
                    orbit.radius = 3.0;
                    live_camera_pan_number.radius *= -1.0;
                }

                // Calculate new camera position
                let yaw_rot = Quat::from_rotation_y(orbit.yaw);
                let pitch_rot = Quat::from_rotation_x(orbit.pitch);
                let offset = yaw_rot * pitch_rot * Vec3::new(0.0, 0.0, orbit.radius);

                transform.translation = Vec3::new(orbit.position_x, orbit.position_y, 0.0) + offset;
                transform.look_at(Vec3::new(orbit.position_x, orbit.position_y, 0.0), Vec3::Y);
            }
        }
        Err(_) => {
            return;
        }
    }
}
