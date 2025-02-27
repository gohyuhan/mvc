use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::{
    capture::take_snapshot,
    components::OrbitCamera,
    resource::{LiveCameraPanNumber, OperationSettings, OperationWindowRelatedEntities},
    states::OperationState,
};

// orbit camera that was control by user
pub fn interactive_orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
    current_operation_state: Res<State<OperationState>>,
    commands: Commands,
    counter: Local<u32>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
    operation_settings: Res<OperationSettings>,
) {
    let orbit_query = query.get_single_mut();

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
                        let sensitivity = operation_settings.mouse_sensitivity;
                        orbit.yaw -= ev.delta.x * sensitivity;
                        orbit.pitch += ev.delta.y * sensitivity;

                        // Clamp pitch to avoid flipping
                        orbit.pitch = orbit.pitch.clamp(-1., 1.);
                    }
                }

                // Zoom with scroll wheel
                for ev in scroll_evr.read() {
                    orbit.radius -= ev.y * operation_settings.zoom_sensitivity;
                }

                // Calculate new camera position
                let yaw_rot = Quat::from_rotation_y(orbit.yaw);
                let pitch_rot = Quat::from_rotation_x(orbit.pitch);
                let offset = yaw_rot * pitch_rot * Vec3::new(0.0, 0.0, orbit.radius);

                transform.translation = Vec3::ZERO + offset;
                transform.look_at(Vec3::ZERO, Vec3::Y);
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

const YAW_SENSITIVITY: f32 = 0.004;
const PITCH_SENSITIVITY: f32 = 0.002;
const ZOOM_SENSITIVITY: f32 = 0.002;

// orbit camera that was control by system, user can't intefere when it was running
pub fn live_orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    current_operation_state: Res<State<OperationState>>,
    mut live_camera_pan_number: ResMut<LiveCameraPanNumber>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
    operation_settings: Res<OperationSettings>,
) {
    let orbit_query = query.get_single_mut();

    match orbit_query {
        Ok((mut transform, mut orbit)) => {
            let c_o_s = current_operation_state.as_ref().get();
            if orbit.window == operation_window.window.unwrap()
                && *c_o_s == OperationState::LivePreview
            {
                orbit.yaw += live_camera_pan_number.yaw * YAW_SENSITIVITY;
                if orbit.yaw >= operation_settings.yaw_max_value {
                    orbit.yaw = operation_settings.yaw_max_value;
                    live_camera_pan_number.yaw *= -1.0;
                } else if orbit.yaw <= operation_settings.yaw_min_value {
                    orbit.yaw = operation_settings.yaw_min_value;
                    live_camera_pan_number.yaw *= -1.0;
                }

                orbit.pitch += live_camera_pan_number.pitch * PITCH_SENSITIVITY;
                if orbit.pitch >= operation_settings.pitch_max_value {
                    orbit.pitch = operation_settings.pitch_max_value;
                    live_camera_pan_number.pitch *= -1.0;
                } else if orbit.pitch <= operation_settings.pitch_min_value {
                    orbit.pitch = operation_settings.pitch_min_value;
                    live_camera_pan_number.pitch *= -1.0;
                }

                orbit.radius += live_camera_pan_number.radius * ZOOM_SENSITIVITY;
                if orbit.radius
                    >= operation_settings.radius_start_position + operation_settings.radius_range
                {
                    orbit.radius =
                        operation_settings.radius_start_position + operation_settings.radius_range;
                    live_camera_pan_number.radius *= -1.0;
                } else if orbit.radius
                    <= operation_settings.radius_start_position - operation_settings.radius_range
                {
                    orbit.radius =
                        operation_settings.radius_start_position - operation_settings.radius_range;
                    live_camera_pan_number.radius *= -1.0;
                }

                // Calculate new camera position
                let yaw_rot = Quat::from_rotation_y(orbit.yaw);
                let pitch_rot = Quat::from_rotation_x(orbit.pitch);
                let offset = yaw_rot * pitch_rot * Vec3::new(0.0, 0.0, orbit.radius);

                transform.translation = Vec3::ZERO + offset;
                transform.look_at(Vec3::ZERO, Vec3::Y);
            }
        }
        Err(_) => {
            return;
        }
    }
}
