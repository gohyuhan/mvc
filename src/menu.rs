use bevy::prelude::*;

use crate::{
    components::{InteractiveMode, PathLabel},
    render::interactive,
    resource::{AssetPath, OperationWindowRelatedEntities},
    states::{AppState, OperationState},
    utils::check_file,
};

const MENU_FONT_SIZE: f32 = 50.;
const PATH_FONT_SIZE: f32 = 20.;
const FONT_SIZE: f32 = 30.;

// To render the Main Menu of MVC for user to interacte to begin operation and such...
pub fn menu(mut commands: Commands, asset_server: Res<AssetServer>, path: Res<AssetPath>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

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
                        Text::new(path.path.clone()),
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
                        PathLabel,
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
    path: ResMut<AssetPath>,
    interactive_mode: Query<&Interaction, (Changed<Interaction>, With<InteractiveMode>)>,
    mut query: Query<(&mut Text, &PathLabel)>,
    mut text_color_query: Query<(&mut TextColor, &PathLabel)>,
    operation_window: ResMut<OperationWindowRelatedEntities>,
    mut app_state: ResMut<NextState<AppState>>,
    mut operation_state: ResMut<NextState<OperationState>>,
) {
    // Check if the files and all were valid then enter window to render 3d model or warn user about invalid file
    if let Ok(Interaction::Pressed) = interactive_mode.get_single() {
        println!("Enter Opration Mode");
        let p = path.path.clone();
        println!("{}", p);
        if check_file(&p) {
            interactive(commands, asset_server, p, operation_window);
            app_state.set(AppState::OperationMode);
            operation_state.set(OperationState::Interactive)
        } else {
            for (mut text, _) in &mut query {
                text.0 = "3d model asset: Please Provide a valid file".to_string();
            }
            for (mut text, _) in &mut text_color_query {
                text.0 = Color::srgb(255., 0.0, 0.0);
            }
        }
    }
}

// a drop file system to handle the load in of files through d & d
pub fn file_drag_and_drop_system(
    mut events: EventReader<FileDragAndDrop>,
    mut three_d_model_asset_path: ResMut<AssetPath>,
    mut query: Query<(&mut Text, &PathLabel)>,
    mut text_color_query: Query<(&mut TextColor, &PathLabel)>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { window, path_buf } = event {
            println!(
                "Dropped file with path: {:?}, in window id: {:?}",
                path_buf, window
            );
            three_d_model_asset_path.path = path_buf.to_str().unwrap().to_string();
            let p = three_d_model_asset_path.path.clone();
            if !check_file(&p) {
                for (mut text, _) in &mut query {
                    text.0 = format!("3d model asset: {} is not a valid file", p.clone());
                }
                for (mut text, _) in &mut text_color_query {
                    text.0 = Color::srgb(255., 0.0, 0.0);
                }
            } else {
                for (mut text, _) in &mut query {
                    text.0 = format!("3d model asset: {}", p.clone())
                }
                for (mut text, _) in &mut text_color_query {
                    text.0 = Color::srgb(255., 255., 255.);
                }
            }
        }
    }
}
