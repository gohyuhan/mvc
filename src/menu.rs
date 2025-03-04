use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use bevy::prelude::*;

use crate::{
    components::{
        ClearModelAssetsButton, ClearSkyboxAssetsButton, InteractiveMode, ModelPathLabel,
        SkyboxPathLabel,
    },
    render::interactive,
    resource::{
        AssetPath, LiveCaptureOperationSettings, OperationSettings, OperationWindowRelatedEntities,
        SavePathList, SkyboxAttribute,
    },
    states::{AppState, OperationState},
    types::{AppSettings, SavePath},
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
                    margin: UiRect {
                        top: Val::Px(5.0),
                        ..Default::default()
                    },
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

                    // to the amount of 3d files
                    parent.spawn((
                        Text::new("[ 3d model asset(s) ]: -"),
                        Node {
                            margin: UiRect {
                                top: Val::Px(5.0),
                                ..Default::default()
                            },
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

                    // to label the path to the skybox file to let user know which model will be render
                    parent.spawn((
                        Text::new("[ skybox asset ]: -"),
                        Node {
                            margin: UiRect {
                                top: Val::Px(5.0),
                                ..Default::default()
                            },
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
                                margin: UiRect {
                                    top: Val::Px(10.0),
                                    ..Default::default()
                                },
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

                    // Button to clear all current models assets path
                    parent
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect {
                                    top: Val::Px(10.0),
                                    ..Default::default()
                                },
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
                            ClearModelAssetsButton,
                        ))
                        .with_child((
                            Text::new("Clear Model Assets"),
                            TextFont {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.0, 0.0)),
                        ));

                    // Button to clear all current skybox assets path
                    parent
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect {
                                    top: Val::Px(10.0),
                                    ..Default::default()
                                },
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
                            ClearSkyboxAssetsButton,
                        ))
                        .with_child((
                            Text::new("Clear Skybox Asset"),
                            TextFont {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.0, 0.0)),
                        ));

                    // to label the keyboard commands
                    parent.spawn((
                        Text::new("Keyboard and Mouse Commands:"),
                        Node {
                            margin: UiRect {
                                top: Val::Px(5.0),
                                bottom: Val::Px(5.0),
                                ..Default::default()
                            },
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("C : capture 1 snapshot"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("L : enter or exit live preview mode"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("I : enter or exit interactive mode"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("Q : render the previous model"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("E : render the next model"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("SPACE : enter or exit live capture mode"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("ARROW KEY : rotate model"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("WASD : move the model"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("MOUSE WHEEL : zoom in or out"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Text::new("MOUSE DRAG : rotate the model"),
                        TextFont {
                            font: font.clone(),
                            font_size: PATH_FONT_SIZE,
                            ..default()
                        },
                    ));
                });
        });
}

// button click system to handle the entering of operation mode
pub fn button_click_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_path: ResMut<AssetPath>,
    interactive_mode: Query<&Interaction, (Changed<Interaction>, With<InteractiveMode>)>,
    clear_model_assets: Query<&Interaction, (Changed<Interaction>, With<ClearModelAssetsButton>)>,
    clear_skybox_assets: Query<&Interaction, (Changed<Interaction>, With<ClearSkyboxAssetsButton>)>,
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
    mut save_settings: ResMut<SavePathList>,
) {
    // Check if the files and all were valid then enter window to render 3d model or warn user about invalid file
    if let Ok(Interaction::Pressed) = interactive_mode.get_single() {
        let mut proceed = true;

        if asset_path.models_path.len() <= 0 {
            for (mut text, _) in path_label_param_set.p0().iter_mut() {
                text.0 = "[ 3d model asset(s) ]: Please Provide at least 1 file".to_string();
            }
            for (mut text, _) in &mut path_label_color_param_set.p0().iter_mut() {
                text.0 = Color::srgb(255., 0.0, 0.0);
            }

            proceed = false;
        }

        if !check_skybox_file(&asset_path.skybox_path) {
            for (mut text, _) in path_label_param_set.p1().iter_mut() {
                text.0 = "[ skybox asset ]: Please Provide a valid file".to_string();
            }
            for (mut text, _) in &mut path_label_color_param_set.p1().iter_mut() {
                text.0 = Color::srgb(255., 0.0, 0.0);
            }

            proceed = false;
        }

        println!("Enter Opration Mode 👷‍♂️");
        if proceed {
            if asset_server.is_loaded(skybox_attributes.skybox_handler.as_ref().unwrap()) {
                interactive(
                    commands,
                    asset_server,
                    asset_path.models_path[asset_path.current_model_path_count as usize].clone(),
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

    if let Ok(Interaction::Pressed) = clear_model_assets.get_single() {
        println!("Clearing 3d model assets 🗑️");
        // remove all save path list and reset the current path count
        save_settings.current_path_count = 0;
        save_settings.save_path_list = vec![];

        // remove all model path and reset the current model path count
        asset_path.models_path = vec![];
        asset_path.current_model_path_count = 0;

        for (mut text, _) in path_label_param_set.p0().iter_mut() {
            text.0 = "[ 3d model asset(s) ]: -".to_string();
        }
    }

    if let Ok(Interaction::Pressed) = clear_skybox_assets.get_single() {
        println!("Clearing skybox assets 🗑️");
        asset_path.skybox_path = "".to_string();
        for (mut text, _) in path_label_param_set.p1().iter_mut() {
            text.0 = "[ skybox asset ]: -".to_string();
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
    mut save_settings: ResMut<SavePathList>,
    mut operation_settings: ResMut<OperationSettings>,
    mut live_capture_settings: ResMut<LiveCaptureOperationSettings>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { window, path_buf } = event {
            println!(
                "Dropped file 📂 with path: {:?}, in window id: {:?}",
                path_buf, window
            );
            let p = path_buf.to_str().unwrap().to_string();

            if check_model_file(&p) {
                let string_path = path_buf.to_str().unwrap().to_string();

                // check if we already have the file path info within the models_path array to prevent duplicate model rendering
                if !three_d_model_asset_path.models_path.contains(&string_path) {
                    // to check if model file then save the path, the directory name and file prefix
                    three_d_model_asset_path.models_path.push(string_path);
                    let filename = path_buf.file_stem().unwrap().to_str().unwrap().to_string();
                    let current_dir_path = PathBuf::from(&save_settings.base_dir_path)
                        .join(&filename)
                        .to_str()
                        .unwrap()
                        .to_string();
                    let file_name_prefix = filename;

                    let image_save_path = SavePath {
                        current_dir_path,
                        file_name_prefix,
                    };

                    save_settings.save_path_list.push(image_save_path);
                    println!("save setting: {:?}", save_settings);

                    for (mut text, _) in &mut path_label_param_set.p0().iter_mut() {
                        text.0 = format!(
                            "[ 3d model asset(s) ]: {} files",
                            three_d_model_asset_path.models_path.len()
                        )
                    }
                    for (mut text, _) in &mut path_label_color_param_set.p0().iter_mut() {
                        text.0 = Color::srgb(255., 255., 255.);
                    }
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
                    text.0 = format!("[ skybox asset ]: {}", p.clone())
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

                save_settings.base_dir_path = new_json_setting.image_save_dir;
            }
        }
    }
}
