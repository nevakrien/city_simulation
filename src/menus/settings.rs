use bevy::{
    color::palettes::css::CRIMSON, 
    prelude::*
};

use crate::{
    common::{despawn_screen,StageSelect},
    settings::globals::{DisplayQuality, Volume},
    menus::{
        menu::MenuButtonAction,
        ui::{
            SettingButton, COLOR_GRAY, COLOR_MAROON, COLOR_RED, COLOR_WHITE, NORMAL_BUTTON,
            SelectedOption, TEXT_COLOR, create_setting_text, drag_slider_system,
            setting_button, spawn_slider_system, update_resource_text,
        },
    },
    settings::{
        settings_io::save_setting_system,
        framerate::{
            ManualFpsCap,
            FramerateMode,
            VsyncMode,
        },
    }
};

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SettingsState {
    Settings,
    Display,
    Sound,
    #[default]
    Disabled,
}


pub fn settings_sub_plugin(app: &mut App) {
    app
        .init_state::<SettingsState>() 

        .add_systems(OnEnter(SettingsState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(SettingsState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        .add_systems(
            OnEnter(SettingsState::Display),
            display_settings_menu_setup,
        )
        .add_systems(
            Update,
            (
                setting_button::<DisplayQuality>,
                setting_button::<FramerateMode>,
                setting_button::<VsyncMode>,
                drag_slider_system::<ManualFpsCap>,
                update_resource_text::<ManualFpsCap>.run_if(resource_changed::<ManualFpsCap>),
            ).run_if(in_state(SettingsState::Display)),
        )
        .add_systems(
            OnExit(SettingsState::Display),
            (
                despawn_screen::<OnDisplaySettingsMenuScreen>,
                save_setting_system::<DisplayQuality>,
                save_setting_system::<FramerateMode>,
                save_setting_system::<VsyncMode>,
                save_setting_system::<ManualFpsCap>,                
            ),
        )
        .add_systems(
            OnEnter(SettingsState::Sound),
            sound_settings_menu_setup,
        )
        .add_systems(
            Update,
            (
                drag_slider_system::<Volume>,
                update_resource_text::<Volume>.run_if(resource_changed::<Volume>),
            )
            .run_if(in_state(SettingsState::Sound)),
        )
        .add_systems(
            OnExit(SettingsState::Sound),
            (
                despawn_screen::<OnSoundSettingsMenuScreen>,
                save_setting_system::<Volume>,
            ),
        );
}

// Tag component used to tag entities added on the settings menu screen
#[derive(Component,Clone,Copy)]
struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component,Clone,Copy)]
struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component,Clone,Copy)]
struct OnSoundSettingsMenuScreen;


fn settings_menu_setup(
    mut commands: Commands,
    game_state: Res<State<StageSelect>>,
) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    // --- TOP: Add Resume if in Game ---
                    if *game_state.get() == StageSelect::Game {
                        parent
                            .spawn((
                                SettingButton,
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                MenuButtonAction::ResumePlay,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Resume"),
                                    button_text_style.clone(),
                                ));
                            });
                    }

                    // --- MIDDLE: Display and Sound ---
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                    ] {
                        parent
                            .spawn((
                                SettingButton,
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn((Text::new(text), button_text_style.clone()));
                            });
                    }

                    // --- BOTTOM: Back or Main Menu depending on context ---
                    let (action, label) = match game_state.get() {
                        StageSelect::Menu => (MenuButtonAction::BackToMainMenu, "Back"),
                        _ => (MenuButtonAction::BackToMainMenu, "Main Menu"),
                    };

                    parent
                        .spawn((
                            SettingButton,
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            action,
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new(label), button_text_style.clone()));
                        });
                });
        });
}


// fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
//     let button_node = Node {
//         width: Val::Px(200.0),
//         height: Val::Px(65.0),
//         margin: UiRect::all(Val::Px(20.0)),
//         justify_content: JustifyContent::Center,
//         align_items: AlignItems::Center,
//         ..default()
//     };
//     let button_text_style = (
//         TextFont {
//             font_size: 33.0,
//             ..default()
//         },
//         TextColor(TEXT_COLOR),
//     );

//     commands
//         .spawn((
//             Node {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 ..default()
//             },
//             OnDisplaySettingsMenuScreen,
//         ))
//         .with_children(|parent| {
//             parent
//                 .spawn((
//                     Node {
//                         flex_direction: FlexDirection::Column,
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     BackgroundColor(CRIMSON.into()),
//                 ))
//                 .with_children(|parent| {
//                     // Create a new `Node`, this time not setting its `flex_direction`. It will
//                     // use the default value, `FlexDirection::Row`, from left to right.
//                     parent
//                         .spawn((
//                             Node {
//                                 align_items: AlignItems::Center,
//                                 ..default()
//                             },
//                             BackgroundColor(CRIMSON.into()),
//                         ))
//                         .with_children(|parent| {
//                             // Display a label for the current setting
//                             parent.spawn((
//                                 Text::new("Display Quality"),
//                                 button_text_style.clone(),
//                             ));
//                             // Display a button for each possible value
//                             for quality_setting in [
//                                 DisplayQuality::Low,
//                                 DisplayQuality::Medium,
//                                 DisplayQuality::High,
//                             ] {
//                                 let mut entity = parent.spawn((
//                                     SettingButton,
//                                     Button,
//                                     Node {
//                                         width: Val::Px(150.0),
//                                         height: Val::Px(65.0),
//                                         ..button_node.clone()
//                                     },
//                                     BackgroundColor(NORMAL_BUTTON),
//                                     quality_setting,
//                                 ));
//                                 entity.with_children(|parent| {
//                                     parent.spawn((
//                                         Text::new(format!("{quality_setting:?}")),
//                                         button_text_style.clone(),
//                                     ));
//                                 });
//                                 if *display_quality == quality_setting {
//                                     entity.insert(SelectedOption);
//                                 }
//                             }
//                         });
//                     // Display the back button to return to the settings screen
//                     parent
//                         .spawn((
//                             Button,
//                             SettingButton,
//                             button_node,
//                             BackgroundColor(NORMAL_BUTTON),
//                             MenuButtonAction::BackToSettings,
//                         ))
//                         .with_children(|parent| {
//                             parent.spawn((Text::new("Back"), button_text_style));
//                         });
//                 });
//         });
// }

fn display_settings_menu_setup(
    mut commands: Commands,
    display_quality: Res<DisplayQuality>,
    fps_cap: Res<ManualFpsCap>,
    framerate_mode: Res<FramerateMode>,
    vsync_mode: Res<VsyncMode>,
) {

    //for later when we do fps slider
    let mut slider_container = None;

    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnDisplaySettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    // --- Display Quality Row ---
                    parent
                        .spawn((
                            Node {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(CRIMSON.into()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Display Quality"), button_text_style.clone()));
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    SettingButton,
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
                                        ..button_node.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    quality_setting,
                                ));
                                entity.with_children(|parent| {
                                    parent.spawn((
                                        Text::new(format!("{quality_setting:?}")),
                                        button_text_style.clone(),
                                    ));
                                });
                                if *display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });

                    // --- VSync Setting ---
                    parent
                        .spawn((
                            Node {
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(15.0)),
                                ..default()
                            },
                            BackgroundColor(CRIMSON.into()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("VSync"), button_text_style.clone()));
                            for mode in [
                                VsyncMode::Enabled,
                                VsyncMode::Disabled,
                            ] {
                                let mut entity = parent.spawn((
                                    SettingButton,
                                    Button,
                                    Node {
                                        width: Val::Px(180.0), // Increased width
                                        height: Val::Px(60.0), // Increased height
                                        margin: UiRect::all(Val::Px(10.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    mode,
                                ));
                                entity.with_children(|parent| {
                                    let label = match mode {
                                        VsyncMode::Enabled => "Enabled",
                                        VsyncMode::Disabled => "Disabled",
                                    };
                                    parent.spawn((
                                        Text::new(label),
                                        button_text_style.clone(),
                                    ));
                                });
                                if *vsync_mode == mode {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });

                    // --- Framerate Mode Buttons ---
                    parent
                        .spawn((
                            Node {
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(15.0)),
                                ..default()
                            },
                            BackgroundColor(CRIMSON.into()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Framerate Mode"), button_text_style.clone()));
                            for mode in [
                                FramerateMode::Auto,
                                FramerateMode::Manual,
                                FramerateMode::Off,
                            ] {
                                let mut entity = parent.spawn((
                                    SettingButton,
                                    Button,
                                    Node {
                                        width: Val::Px(140.0),
                                        height: Val::Px(55.0),
                                        margin: UiRect::all(Val::Px(8.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    mode,
                                ));
                                entity.with_children(|parent| {
                                    parent.spawn((
                                        Text::new(format!("{mode:?}")),
                                        button_text_style.clone(),
                                    ));
                                });
                                if *framerate_mode == mode {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });

                    // --- Manual FPS Slider (only in Manual mode) ---

                    // Use a scope to release the borrow on `parent`
                    slider_container = Some(
                        parent
                            .spawn(Node {
                                align_items: AlignItems::Center,
                                ..default()
                            })
                            .id()
                    );

                    // Continue using `parent` safely
                    parent.spawn((Text::new("FPS Limit"), button_text_style.clone()));
                    create_setting_text(parent, &fps_cap);

                        

                    // --- Back Button ---
                    parent
                        .spawn((
                            Button,
                            SettingButton,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Back"), button_text_style.clone()));
                        });
                });
        });
        spawn_slider_system::<ManualFpsCap, OnDisplaySettingsMenuScreen>(
                            commands,
                            Node {
                                width: Val::Px(250.0),
                                height: Val::Px(20.0),
                                margin: UiRect::all(Val::Px(20.0)),
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            Some(Node {
                                width: Val::Px(20.0),
                                height: Val::Px(30.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(0.0),
                                top: Val::Px(-5.0),
                                ..default()
                            }),
                            OnDisplaySettingsMenuScreen,
                            Some(slider_container.unwrap()),
                            Some(fps_cap),
                        );
}




fn sound_settings_menu_setup(
    mut commands: Commands,
    volume: Res<Volume>,
) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont { font_size: 33.0, ..default() },
        TextColor(COLOR_WHITE),
    );

    // 1) UI layout for the “Volume Menu” background, text, etc.
    let mut volume_container = None;

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSoundSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(COLOR_RED), // or CRIMSON, etc.
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(COLOR_MAROON),
                        ))
                        .with_children(|parent| {
                            
                            // parent.spawn((Text::new("Volume"), button_text_style.clone()));
                            parent.spawn((Text::new("Volume:"), button_text_style.clone()));
                            create_setting_text(parent,&volume);
                            // parent.spawn((
                            //     SliderValueText::<Volume>(PhantomData),
                            //     Text::new(format!("{:.2}", volume.as_fraction())),
                            //     button_text_style.clone()));

                                
                            volume_container=Some(
                                parent.spawn((
                                    Node {
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                ))
                                .id()
                            );

                        });

                    // "Back" button
                    parent
                        .spawn((
                            Button,
                            SettingButton,
                            button_node,
                            BackgroundColor(COLOR_GRAY),
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_child((Text::new("Back"), button_text_style));
                });
        });

    // 2) Spawn the slider with proper positioning
    //    We map [0..1] → Volume(0..10).
    //    Get node reference to the volume parent container
    spawn_slider_system::<Volume, OnSoundSettingsMenuScreen>(
        commands,
        Node {
            width: Val::Px(250.0),
            height: Val::Px(20.0),
            // Center slider and add proper margins
            margin: UiRect::all(Val::Px(20.0)),
            align_self: AlignSelf::Center,
            ..default()
        },
        Some(Node {
            width: Val::Px(20.0),
            height: Val::Px(30.0),
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(-5.0),
            ..default()
        }),
        OnSoundSettingsMenuScreen,
        Some(volume_container.unwrap()),
        Some(volume)
    );
}