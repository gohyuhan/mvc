use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*, window::PrimaryWindow,
};

use crate::{
    capture::take_snapshot,
    components::OrbitCamera,
    resource::{
        LiveCameraPanNumber, LiveCaptureOperationSettings, OperationSettings,
        OperationWindowRelatedEntities, SavePath,
    },
    states::OperationState,
};

// orbit camera that was control by user
pub fn interactive_orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
    operation_window: Res<OperationWindowRelatedEntities>,
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

// orbit camera that was control by system, user can't intefere when it was running [like orbiting using mouse],
// will loop infinitly until exit live preview mode
pub fn live_orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    mut live_camera_pan_number: ResMut<LiveCameraPanNumber>,
    operation_settings: Res<OperationSettings>,
) {
    let orbit_query = query.get_single_mut();

    match orbit_query {
        Ok((mut transform, mut orbit)) => {
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
        Err(_) => {
            return;
        }
    }
}

// camera that was control by the system that move to the coordinates that was calculated by fibonacci sphere and capture a screen shot
// will end once it reach the end of the list and will switch to interactive mode once end
pub fn live_capture_camera(
    commands: Commands,
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    mut operation_state: ResMut<NextState<OperationState>>,
    operation_window: Res<OperationWindowRelatedEntities>,
    mut live_capture_settings: ResMut<LiveCaptureOperationSettings>,
    save_settings: Res<SavePath>,
    mut window_query: Query<&mut Window, Without<PrimaryWindow>>,
) {
    let orbit_query = query.get_single_mut();
    match orbit_query {
        Ok((mut transform, mut orbit)) => {
            let current_coordinates = live_capture_settings.live_capture_coordinate_list
                [live_capture_settings.live_capture_iteration_current_counter as usize];
            orbit.yaw = current_coordinates.0;
            orbit.pitch = current_coordinates.1;
            orbit.radius = current_coordinates.2;

            // Calculate new camera position
            let yaw_rot = Quat::from_rotation_y(orbit.yaw);
            let pitch_rot = Quat::from_rotation_x(orbit.pitch);
            let offset = yaw_rot * (pitch_rot * Vec3::new(0.0, 0.0, orbit.radius));

            transform.translation = offset;
            transform.look_at(Vec3::ZERO, Vec3::Y);

            take_snapshot(
                commands,
                operation_window,
                &save_settings,
                orbit.yaw,
                orbit.pitch,
                orbit.radius,
            );

            live_capture_settings.live_capture_iteration_current_counter += 1;
            for mut window in window_query.iter_mut() {
                window.title = format!("Live Capturing 🎥 [{}/{}]", live_capture_settings.live_capture_iteration_current_counter, live_capture_settings.live_capture_iteration);
            }
            if live_capture_settings.live_capture_iteration_current_counter
                >= live_capture_settings.live_capture_iteration
            {   
                operation_state.set(OperationState::Interactive);
                for mut window in window_query.iter_mut() {
                    window.title = "Interactive 📱".to_string();
                }
            }
        }
        Err(_) => {
            return;
        }
    }
}