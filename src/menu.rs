use std::path::Path;

use bevy::prelude::*;

use crate::{
    components::{InteractiveMode, PathLabel, QuitButton},
    render::interactive,
    resource::{AssetPath, OperationWindowRelatedEntities},
    states::AppState,
};

const MENU_FONT_SIZE: f32 = 50.;
const FONT_SIZE: f32 = 30.;

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
            // horizontal scroll example
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

                    parent.spawn((
                        Text::new(path.path.clone()),
                        Node {
                            top: Val::Px(20.),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        TextFont {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            ..default()
                        },
                        PathLabel,
                    ));

                    // spawn Relative Mode Button
                    parent
                        .spawn((
                            Button,
                            Node {
                                top: Val::Px(25.),
                                height: Val::Px(50.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(Color::BLACK),
                            BorderRadius::MAX,
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                            InteractiveMode,
                        ))
                        .with_child((
                            Text::new("Interactive Mode"),
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

pub fn button_click_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut path: ResMut<AssetPath>,
    interactive_mode: Query<&Interaction, (Changed<Interaction>, With<InteractiveMode>)>,
    quit_button: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut query: Query<(&mut Text, &PathLabel)>,
    mut text_color_query: Query<(&mut TextColor, &PathLabel)>,
    mut operationWindow: ResMut<OperationWindowRelatedEntities>,
    mut state: ResMut<NextState<AppState>>,
) {
    // Check Play button
    if let Ok(Interaction::Pressed) = interactive_mode.get_single() {
        println!("Enter Interactive Mode");
        let p = path.path.clone();
        println!("{}", p);
        if check_file(&p) {
            state.set(AppState::OperationStart);
            interactive(commands, asset_server, p, operationWindow);
        } else {
            for (mut text, _) in &mut query {
                text.0 = "Please Provide a valid file".to_string();
            }
            for (mut text, _) in &mut text_color_query {
                text.0 = Color::srgb(255., 0.0, 0.0);
            }
        }
    }

    // Check Quit button
    if let Ok(Interaction::Pressed) = quit_button.get_single() {
        println!("Quit Button Pressed!");
    }
}

pub fn file_drag_and_drop_system(
    mut events: EventReader<FileDragAndDrop>,
    mut path: ResMut<AssetPath>,
    mut query: Query<(&mut Text, &PathLabel)>,
    mut text_color_query: Query<(&mut TextColor, &PathLabel)>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { window, path_buf } = event {
            println!(
                "Dropped file with path: {:?}, in window id: {:?}",
                path_buf, window
            );
            path.path = path_buf.to_str().unwrap().to_string();
            let p = path.path.clone();
            if !check_file(&p) {
                for (mut text, _) in &mut query {
                    text.0 = format!("{} is not a valid file", p.clone());
                }
                for (mut text, _) in &mut text_color_query {
                    text.0 = Color::srgb(255., 0.0, 0.0);
                }
            } else {
                for (mut text, _) in &mut query {
                    text.0 = p.clone()
                }
                for (mut text, _) in &mut text_color_query {
                    text.0 = Color::srgb(255., 255., 255.);
                }
            }
        }
    }
}

fn check_file(file_path: &str) -> bool {
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
