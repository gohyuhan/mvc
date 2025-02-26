use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::{
    capture::take_snapshot, components::OrbitCamera, resource::OperationWindowRelatedEntities,
    states::OperationState,
};

pub fn interactive_orbit_camera(
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
    current_operation_state: Res<State<OperationState>>,
    commands: Commands,
    counter: Local<u32>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
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
                        let sensitivity = 0.005;
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
