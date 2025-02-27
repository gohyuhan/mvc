use std::{fs::{create_dir_all, File, OpenOptions}, path::{Path, PathBuf}};

use bevy::{
    prelude::*,
    window::{WindowCloseRequested, WindowFocused},
};

use crate::{
    capture::take_snapshot,
    resource::{ActiveWindowId, LiveCameraPanNumber, OperationWindowRelatedEntities},
    states::{AppState, OperationState}, types::AppSettings,
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
    mut live_camera_pan_number: ResMut<LiveCameraPanNumber>,
) {
    for ev in window_close_requested_events.read() {
        if ev.window == operation_window.window.unwrap() {
            app_state.set(AppState::MainMenu);
            operation_state.set(OperationState::None);
            live_camera_pan_number.yaw = 1.0;
            live_camera_pan_number.pitch = 1.0;
            live_camera_pan_number.radius = 1.0;
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

pub fn init_app() -> AppSettings {
    let image_save_dir = get_user_directory().join("Downloads").to_string_lossy().to_string();
    const YAW_MIN_VALUE: f32 = -0.75;
    const YAW_MAX_VALUE: f32 =  0.75;
    const PITCH_MIN_VALUE: f32 = -0.20;
    const PITCH_MAX_VALUE: f32 = -0.60;
    const RADIUS_RANGE: f32 = 2.0;
    const MODEL_ROTATE_SENSITIVITY:f32 = 0.001;
    const MODEL_REPOSITION_SENSITIVITY: f32 = 0.01;
    const MOUSE_SENSITIVITY: f32 = 0.0025;
    const ZOOM_SENSITIVITY: f32 = 0.25;
    const LIVE_CAPTURE_ITERATION: u32 = 5000;

    // check if there is a settings file, if not create it
    let settings_file_path = get_user_directory().join(".mvc/settings.json");
    if !settings_file_path.exists(){
        let app_settings = AppSettings {
            image_save_dir: image_save_dir,
            yaw_min_value: YAW_MIN_VALUE,
            yaw_max_value: YAW_MAX_VALUE,
            pitch_min_value: PITCH_MIN_VALUE,
            pitch_max_value: PITCH_MAX_VALUE,
            radius_range: RADIUS_RANGE,
            model_rotate_sensitivity: MODEL_ROTATE_SENSITIVITY,
            model_reposition_sensitivity: MODEL_REPOSITION_SENSITIVITY,
            mouse_sensitivity: MOUSE_SENSITIVITY,
            zoom_sensitivity: ZOOM_SENSITIVITY,
            live_capture_iteration: LIVE_CAPTURE_ITERATION,
        };

        create_file_with_dirs(settings_file_path.to_str().unwrap());
        let file = OpenOptions::new()
        .write(true)
        .create(true)  // Create the file if it doesn't exist
        .truncate(true) // Truncate the file to ensure it's empty before writing
        .open(settings_file_path).unwrap();

        // write the data into the json file
        let _ = serde_json::to_writer(file, &app_settings);

        return app_settings;
    }

    // read the json file to configure the settings instead if it exist
    let file = File::open(settings_file_path).unwrap();
    let json_setting:AppSettings = serde_json::from_reader(file).unwrap();

    return json_setting;

    
}

fn get_user_directory() -> PathBuf {
    let home_dir = if cfg!(unix) {
        std::env::var("HOME").unwrap()
    } else {
        std::env::var("USERPROFILE").unwrap()
    };

    return PathBuf::from(home_dir);
}

fn create_file_with_dirs(path: &str) {
    // Create all missing directories in the path
    let _ = create_dir_all(std::path::Path::new(path).parent().unwrap());

    File::create(path).unwrap();
}