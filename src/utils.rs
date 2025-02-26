use std::path::Path;

use bevy::{
    prelude::*,
    window::{WindowCloseRequested, WindowFocused},
};

use crate::{
    capture::take_snapshot,
    resource::{ActiveWindowId, OperationWindowRelatedEntities},
    states::{AppState, OperationState},
};

pub fn check_model_file(file_path: &str) -> bool {
    match Path::new(file_path).extension() {
        Some(ext) => {
            if ext == "glb" || ext == "gltf" {
                return true;
            } else {
                return false;
            }
        }
        None => false,
    }
}

pub fn check_skybox_file(file_path: &str) -> bool {
    match Path::new(file_path).extension() {
        Some(ext) => {
            if ext == "webp"
                || ext == "jpg"
                || ext == "jpeg"
                || ext == "png"
                || ext == "hdr"
                || ext == "exr"
            {
                return true;
            } else {
                return false;
            }
        }
        None => false,
    }
}

pub fn check_json_file(file_path: &str) -> bool {
    match Path::new(file_path).extension() {
        Some(ext) => {
            if ext == "json" {
                return true;
            } else {
                return false;
            }
        }
        None => false,
    }
}

pub fn track_active_window(
    mut events: EventReader<WindowFocused>,
    mut active_window_id: ResMut<ActiveWindowId>,
) {
    for event in events.read() {
        if event.focused {
            println!("Active window updated: {:?}", event.window);
            active_window_id.id = format!("{:?}", event.window);
        }
    }
}

pub fn switch_state_on_window_event(
    mut commands: Commands,
    _windows: Query<Entity, With<Window>>,
    _entities: Query<Entity>,
    mut window_close_requested_events: EventReader<WindowCloseRequested>,
    mut app_state: ResMut<NextState<AppState>>,
    mut operation_state: ResMut<NextState<OperationState>>,
    mut operation_window: ResMut<OperationWindowRelatedEntities>,
) {
    for ev in window_close_requested_events.read() {
        if ev.window == operation_window.window.unwrap() {
            app_state.set(AppState::MainMenu);
            operation_state.set(OperationState::None);
            for entity in operation_window.entities_list.as_mut().unwrap() {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }
}

// check keyboard in interactive mode for space bar to take snapshot
pub fn keyboard_interact(
    keys: Res<ButtonInput<KeyCode>>,
    mut operation_state: ResMut<NextState<OperationState>>,
    current_operation_state: Res<State<OperationState>>,

    // to pass to take_snapshot function
    commands: Commands,
    counter: Local<u32>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
) {
    let c_o_s = current_operation_state.as_ref().get();
    if *c_o_s == OperationState::LiveCapture {
        if keys.just_pressed(KeyCode::Space) {
            println!("stop live capturing");
            operation_state.set(OperationState::Interactive);
        } else if keys.just_pressed(KeyCode::KeyI) {
            println!("stop live capturing");
            operation_state.set(OperationState::Interactive);
        }
    } else if *c_o_s == OperationState::LivePreview {
        if keys.just_pressed(KeyCode::KeyL) {
            println!("stop live prviewing");
            operation_state.set(OperationState::Interactive);
        } else if keys.just_pressed(KeyCode::KeyI) {
            println!("stop live prviewing");
            operation_state.set(OperationState::Interactive);
        }
    } else if *c_o_s == OperationState::Interactive {
        if keys.just_pressed(KeyCode::Space) {
            println!("start live capturing");
            operation_state.set(OperationState::LiveCapture);
        } else if keys.just_pressed(KeyCode::KeyL) {
            println!("start live prviewing");
            operation_state.set(OperationState::LivePreview);
        }
    }

    // no matter the operation state, when key c is press capture 1 copy of current model snapshot
    if keys.just_pressed(KeyCode::KeyC) {
        take_snapshot(commands, counter, operation_window);
    }
}
