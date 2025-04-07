
// use crate::globals::GameState;
// use crate::menus::ui::button_system;
use crate::menus::ui::BasicButton;
use crate::globals::GameState;
use crate::settings_io::save_setting_system;
use crate::menus::ui::update_resource_text;
use crate::menus::ui::drag_slider_system;
use crate::menus::ui::setting_button;
use crate::menus::despawn_screen;
use crate::menus::menu::MenuButtonAction;
use bevy::{color::palettes::css::CRIMSON, prelude::*};

use crate::globals::{DisplayQuality, Volume};
use crate::menus::ui::{
        create_slider_text,
        spawn_slider_system, COLOR_GRAY, COLOR_MAROON, COLOR_RED,
        COLOR_WHITE, NORMAL_BUTTON, SelectedOption, TEXT_COLOR,
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
            setting_button::<DisplayQuality>.run_if(in_state(SettingsState::Display)),
        )
        .add_systems(
            OnExit(SettingsState::Display),
            (
                despawn_screen::<OnDisplaySettingsMenuScreen>,
                save_setting_system::<DisplayQuality>,
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
                update_resource_text::<Volume>.run_if(resource_changed::<Volume>)
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
    game_state: Res<State<GameState>>,
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
                    if *game_state.get() == GameState::Game {
                        parent
                            .spawn((
                                BasicButton,
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
                                BasicButton,
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
                        GameState::Menu => (MenuButtonAction::BackToMainMenu, "Back"),
                        _ => (MenuButtonAction::BackToMainMenu, "Main Menu"),
                    };

                    parent
                        .spawn((
                            BasicButton,
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


fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
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
                    // Create a new `Node`, this time not setting its `flex_direction`. It will
                    // use the default value, `FlexDirection::Row`, from left to right.
                    parent
                        .spawn((
                            Node {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(CRIMSON.into()),
                        ))
                        .with_children(|parent| {
                            // Display a label for the current setting
                            parent.spawn((
                                Text::new("Display Quality"),
                                button_text_style.clone(),
                            ));
                            // Display a button for each possible value
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    BasicButton,
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
                    // Display the back button to return to the settings screen
                    parent
                        .spawn((
                            Button,
                            BasicButton,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::BackToSettings,
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new("Back"), button_text_style));
                        });
                });
        });
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
                            create_slider_text(parent,&volume);
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
                            BasicButton,
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