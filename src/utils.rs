use std::{
    fs::{create_dir_all, File, OpenOptions},
    path::{Path, PathBuf},
};

use bevy::{prelude::*, window::WindowCloseRequested};

use crate::{
    capture::take_snapshot,
    components::OrbitCamera,
    resource::{
        LiveCameraPanNumber, LiveCaptureOperationSettings, OperationSettings,
        OperationWindowRelatedEntities,
    },
    states::{AppState, OperationState},
    types::AppSettings,
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
    query: Query<&OrbitCamera>,
    mut operation_settings: ResMut<OperationSettings>,
    mut live_capture_settings: ResMut<LiveCaptureOperationSettings>,
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
        operation_settings.radius_start_position = query.get_single().unwrap().radius;
        if keys.just_pressed(KeyCode::Space) {
            println!("start live capturing");
            let coordinates_list = generate_points(
                live_capture_settings.live_capture_iteration,
                (
                    operation_settings.yaw_min_value,
                    operation_settings.yaw_max_value,
                ),
                (
                    operation_settings.pitch_min_value,
                    operation_settings.pitch_max_value,
                ),
                (
                    operation_settings.radius_start_position,
                    operation_settings.radius_start_position + operation_settings.radius_range,
                ),
            );
            println!("{:?}", coordinates_list);
            live_capture_settings.live_capture_iteration = coordinates_list.len();
            live_capture_settings.live_capture_coordinate_list = Some(coordinates_list);
            live_capture_settings.live_capture_iteration_current_counter = 0;
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
    let image_save_dir = get_user_directory()
        .join("Downloads")
        .to_string_lossy()
        .to_string();
    const YAW_MIN_VALUE: f32 = -0.75;
    const YAW_MAX_VALUE: f32 = 0.75;
    const PITCH_MIN_VALUE: f32 = -0.60;
    const PITCH_MAX_VALUE: f32 = -0.20;
    const RADIUS_RANGE: f32 = 2.0;
    const MODEL_ROTATE_SENSITIVITY: f32 = 0.05;
    const MODEL_REPOSITION_SENSITIVITY: f32 = 0.05;
    const MOUSE_SENSITIVITY: f32 = 0.0025;
    const ZOOM_SENSITIVITY: f32 = 0.25;
    const LIVE_CAPTURE_ITERATION: usize = 5000;

    // check if there is a settings file, if not create it
    let settings_file_path = get_user_directory().join(".mvc/settings.json");
    if !settings_file_path.exists() {
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
            .create(true) // Create the file if it doesn't exist
            .truncate(true) // Truncate the file to ensure it's empty before writing
            .open(settings_file_path)
            .unwrap();

        // write the data into the json file
        let _ = serde_json::to_writer(file, &app_settings);

        return app_settings;
    }

    // read the json file to configure the settings instead if it exist
    let file = File::open(settings_file_path).unwrap();
    let json_setting: AppSettings = serde_json::from_reader(file).unwrap();

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

// the following part was created with the help of AI, futher validation is needed

// Generate the n-th term of a Halton sequence for a given base
fn halton(index: u32, base: u32) -> f32 {
    let mut result = 0.0;
    let mut f = 1.0;
    let mut i = index;

    while i > 0 {
        f /= base as f32;
        result += f * (i % base) as f32;
        i /= base;
    }

    return result;
}

// Generate 3D points within specified ranges using the Halton sequence
fn generate_points(
    count: usize,
    x_range: (f32, f32),
    y_range: (f32, f32),
    z_range: (f32, f32),
) -> Vec<(f32, f32, f32)> {
    return (0..count)
        .map(|i| {
            let x = x_range.0 + (x_range.1 - x_range.0) * halton(i as u32 + 1, 2);
            let y = y_range.0 + (y_range.1 - y_range.0) * halton(i as u32 + 1, 3);
            let z = z_range.0 + (z_range.1 - z_range.0) * halton(i as u32 + 1, 5);
            return (x, y, z);
        })
        .collect();
}
