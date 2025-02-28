use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use bevy::prelude::*;

use crate::{
    components::{InteractiveMode, ModelPathLabel, SkyboxPathLabel},
    render::interactive,
    resource::{
        AssetPath, LiveCaptureOperationSettings, OperationSettings, OperationWindowRelatedEntities,
        SavePath, SkyboxAttribute,
    },
    states::{AppState, OperationState},
    types::AppSettings,
    utils::{check_json_file, check_model_file, check_skybox_file, get_user_directory},
};

const MENU_FONT_SIZE: f32 = 50.;
const PATH_FONT_SIZE: f32 = 20.;
const FONT_SIZE: f32 = 30.;

// To render the Main Menu of MVC for user to interacte to begin operation and such...
pub fn menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load the font from embedded resource
    let font = asset_server.load("embedded://mvc/assets/fonts/FiraSans-Bold.ttf");

    // Camera
    commands.spawn(Camera2d);

    // root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::Center,
            justify_items: JustifyItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    top: Val::Px(10.),
                    ..default()
                })
                .with_children(|parent| {
                    // header (menu title)
                    parent.spawn((
                        Text::new("MVC MAIN MENU"),
                        TextFont {
                            font: font.clone(),
                            font_size: MENU_FONT_SIZE,
                            ..default()
                        },
                        Label,
                    ));

                    // to label the path to the 3d file to let user know which model will be render
                    parent.spawn((
                        Text::new("[3d model asset]: -"),
                        Node {
                            top: Val::Px(20.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                        ModelPathLabel,
                    ));

                    // to label the path to the 3d file to let user know which model will be render
                    parent.spawn((
                        Text::new("[skybox asset]: -"),
                        Node {
                            top: Val::Px(20.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                        SkyboxPathLabel,
                    ));

                    // Button to Start 3d render operation
                    parent
                        .spawn((
                            Button,
                            Node {
                                top: Val::Px(25.),
                                height: Val::Px(50.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            InteractiveMode,
                        ))
                        .with_child((
                            Text::new("Render Model"),
                            TextFont {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                });
        });
}

// button click system to handle the entering of operation mode
pub fn button_click_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    asset_path: ResMut<AssetPath>,
    interactive_mode: Query<&Interaction, (Changed<Interaction>, With<InteractiveMode>)>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
    mut app_state: ResMut<NextState<AppState>>,
    mut operation_state: ResMut<NextState<OperationState>>,
    images: ResMut<Assets<Image>>,
    skybox_attributes: Res<SkyboxAttribute>,
    // to query back those text for changing the inner text
    mut path_label_param_set: ParamSet<(
        Query<(&mut Text, &ModelPathLabel)>,
        Query<(&mut Text, &SkyboxPathLabel)>,
    )>,
    // to query back those text color for changing the text color
    mut path_label_color_param_set: ParamSet<(
        Query<(&mut TextColor, &ModelPathLabel)>,
        Query<(&mut TextColor, &SkyboxPathLabel)>,
    )>,
    operation_settings: Res<OperationSettings>,
) {
    // Check if the files and all were valid then enter window to render 3d model or warn user about invalid file
    if let Ok(Interaction::Pressed) = interactive_mode.get_single() {
        println!("Enter Opration Mode");
        let a_p = asset_path.clone();
        let mut proceed = true;
        if !check_model_file(&a_p.model_path) {
            for (mut text, _) in path_label_param_set.p0().iter_mut() {
                text.0 = "[3d model asset]: Please Provide a valid file".to_string();
            }
            for (mut text, _) in &mut path_label_color_param_set.p0().iter_mut() {
                text.0 = Color::srgb(255., 0.0, 0.0);
            }

            proceed = false;
        }

        if !check_skybox_file(&a_p.skybox_path) {
            for (mut text, _) in path_label_param_set.p1().iter_mut() {
                text.0 = "[skybox asset]: Please Provide a valid file".to_string();
            }
            for (mut text, _) in &mut path_label_color_param_set.p1().iter_mut() {
                text.0 = Color::srgb(255., 0.0, 0.0);
            }

            proceed = false;
        }

        if proceed {
            if asset_server.is_loaded(skybox_attributes.skybox_handler.as_ref().unwrap()) {
                interactive(
                    commands,
                    asset_server,
                    a_p.model_path.clone(),
                    images,
                    skybox_attributes,
                    operation_window,
                    operation_settings,
                );
                app_state.set(AppState::OperationMode);
                operation_state.set(OperationState::Interactive)
            }
        }
    }
}

// a drop file system to handle the load in of files through d & d
pub fn file_drag_and_drop_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<FileDragAndDrop>,
    mut three_d_model_asset_path: ResMut<AssetPath>,
    // to query back those text for changing the inner text
    mut path_label_param_set: ParamSet<(
        Query<(&mut Text, &ModelPathLabel)>,
        Query<(&mut Text, &SkyboxPathLabel)>,
    )>,
    // to query back those text color for changing the text color
    mut path_label_color_param_set: ParamSet<(
        Query<(&mut TextColor, &ModelPathLabel)>,
        Query<(&mut TextColor, &SkyboxPathLabel)>,
    )>,
    mut save_settings: ResMut<SavePath>,
    mut operation_settings: ResMut<OperationSettings>,
    mut live_capture_settings: ResMut<LiveCaptureOperationSettings>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { window, path_buf } = event {
            println!(
                "Dropped file with path: {:?}, in window id: {:?}",
                path_buf, window
            );
            let p = path_buf.to_str().unwrap().to_string();

            if check_model_file(&p) {
                // to check if model file then save the path, the directory name and file prefix
                three_d_model_asset_path.model_path = path_buf.to_str().unwrap().to_string();
                let filename = path_buf.file_stem().unwrap().to_str().unwrap().to_string();
                save_settings.current_dir_path = PathBuf::from(&save_settings.base_dir_path)
                    .join(&filename)
                    .to_str()
                    .unwrap()
                    .to_string();
                save_settings.file_name_prefix = filename;

                for (mut text, _) in &mut path_label_param_set.p0().iter_mut() {
                    text.0 = format!("[3d model asset]: {}", p.clone())
                }
                for (mut text, _) in &mut path_label_color_param_set.p0().iter_mut() {
                    text.0 = Color::srgb(255., 255., 255.);
                }
            } else if check_skybox_file(&p) {
                // to check if skybox file then save the path
                three_d_model_asset_path.skybox_path = path_buf.to_str().unwrap().to_string();
                // the skybox handler
                let skybox_handle: Handle<Image> =
                    asset_server.load(three_d_model_asset_path.skybox_path.clone());

                commands.insert_resource(SkyboxAttribute {
                    skybox_handler: Some(skybox_handle),
                });
                for (mut text, _) in &mut path_label_param_set.p1().iter_mut() {
                    text.0 = format!("[skybox asset]: {}", p.clone())
                }
                for (mut text, _) in &mut path_label_color_param_set.p1().iter_mut() {
                    text.0 = Color::srgb(255., 255., 255.);
                }
            } else if check_json_file(&p) {
                let file = OpenOptions::new()
                    .write(true)
                    .create(true) // Create the file if it doesn't exist
                    .truncate(true) // Truncate the file to ensure it's empty before writing
                    .open(get_user_directory().join(".mvc/settings.json"))
                    .unwrap();

                let new_json_file = File::open(&p).unwrap();
                let new_json_setting: AppSettings = serde_json::from_reader(new_json_file).unwrap();

                serde_json::to_writer(file, &new_json_setting).unwrap();

                operation_settings.yaw_min_value = new_json_setting.yaw_min_value;
                operation_settings.yaw_max_value = new_json_setting.yaw_max_value;
                operation_settings.pitch_min_value = new_json_setting.pitch_min_value;
                operation_settings.pitch_max_value = new_json_setting.pitch_max_value;
                operation_settings.radius_range = new_json_setting.radius_range;
                operation_settings.radius_start_position = 2.5;
                operation_settings.model_rotate_sensitivity =
                    new_json_setting.model_rotate_sensitivity;
                operation_settings.model_reposition_sensitivity =
                    new_json_setting.model_reposition_sensitivity;
                operation_settings.mouse_sensitivity = new_json_setting.mouse_sensitivity;
                operation_settings.zoom_sensitivity = new_json_setting.zoom_sensitivity;

                live_capture_settings.live_capture_iteration =
                    new_json_setting.live_capture_iteration;
            }
        }
    }
}
