use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};

use crate::globals::GameState;
use crate::resources::{DisplayQuality, Volume};
use crate::systems::{
    despawn_screen,
    ui::{
        button_system, create_slider_text, drag_slider_system, setting_button,
        spawn_slider_system, update_resource_text, COLOR_GRAY, COLOR_MAROON, COLOR_RED,
        COLOR_WHITE, NORMAL_BUTTON, SelectedOption, TEXT_COLOR,
    },
};

// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub fn menu_plugin(app: &mut App) {
    app
        // At start, the menu is not enabled. This will be changed in `menu_setup` when
        // entering the `GameState::Menu` state.
        // Current screen in the menu is handled by an independent state from `GameState`
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        // Systems to handle the settings menu screen
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        // Systems to handle the display settings screen
        .add_systems(
            OnEnter(MenuState::SettingsDisplay),
            display_settings_menu_setup,
        )
        .add_systems(
            Update,
            (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
        )
        .add_systems(
            OnExit(MenuState::SettingsDisplay),
            despawn_screen::<OnDisplaySettingsMenuScreen>,
        )
        // Systems to handle the sound settings screen
        .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
        .add_systems(
            Update,
            (   
                drag_slider_system::<Volume>,
                update_resource_text::<Volume>.run_if(resource_changed::<Volume>)
            )
            .run_if(in_state(MenuState::SettingsSound)),
        )
        .add_systems(
            OnExit(MenuState::SettingsSound),
            despawn_screen::<OnSoundSettingsMenuScreen>,
        )
        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        );
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    #[default]
    Disabled,
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component,Clone,Copy)]
pub struct OnMainMenuScreen;

// Tag component used to tag entities added on the settings menu screen
#[derive(Component,Clone,Copy)]
pub struct OnSettingsMenuScreen;

// Tag component used to tag entities added on the display settings menu screen
#[derive(Component,Clone,Copy)]
pub struct OnDisplaySettingsMenuScreen;

// Tag component used to tag entities added on the sound settings menu screen
#[derive(Component,Clone,Copy)]
pub struct OnSoundSettingsMenuScreen;


// All actions that can be triggered from a button click
#[derive(Component,Clone,Copy)]
pub enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
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
                    // Display the game name
                    parent.spawn((
                        Text::new("Bevy Game Menu UI"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ));

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("bevy_examples/textures/Game Icons/right.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                            parent.spawn((
                                Text::new("New Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("bevy_examples/textures/Game Icons/wrench.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node.clone()));
                            parent.spawn((
                                Text::new("Settings"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ));
                        });
                    parent
                        .spawn((
                            Button,
                            button_node,
                            BackgroundColor(NORMAL_BUTTON),
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("bevy_examples/textures/Game Icons/exitRight.png");
                            parent.spawn((ImageNode::new(icon), button_icon_node));
                            parent.spawn((
                                Text::new("Quit"),
                                button_text_font,
                                TextColor(TEXT_COLOR),
                            ));
                        });
                });
        });
}

fn settings_menu_setup(mut commands: Commands) {
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
                    for (action, text) in [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(NORMAL_BUTTON),
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn((Text::new(text), button_text_style.clone()));
                            });
                    }
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

                            // (Optional) If you want to keep the old volume loop for buttons,
                            // you can keep it. If not, remove it.
                            /*
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(30.0),
                                        height: Val::Px(65.0),
                                        ..button_node.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    Volume(volume_setting),
                                ));
                                if *volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                            */
                        });

                    // "Back" button
                    parent
                        .spawn((
                            Button,
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



// fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
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
//             OnSoundSettingsMenuScreen,
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
//                     parent
//                         .spawn((
//                             Node {
//                                 align_items: AlignItems::Center,
//                                 ..default()
//                             },
//                             BackgroundColor(CRIMSON.into()),
//                         ))
//                         .with_children(|parent| {
//                             parent.spawn((Text::new("Volume"), button_text_style.clone()));
//                             for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
//                                 let mut entity = parent.spawn((
//                                     Button,
//                                     Node {
//                                         width: Val::Px(30.0),
//                                         height: Val::Px(65.0),
//                                         ..button_node.clone()
//                                     },
//                                     BackgroundColor(NORMAL_BUTTON),
//                                     Volume(volume_setting),
//                                 ));
//                                 if *volume == Volume(volume_setting) {
//                                     entity.insert(SelectedOption);
//                                 }
//                             }
//                         });
//                     parent
//                         .spawn((
//                             Button,
//                             button_node,
//                             BackgroundColor(NORMAL_BUTTON),
//                             MenuButtonAction::BackToSettings,
//                         ))
//                         .with_child((Text::new("Back"), button_text_style));
//                 });
//         });
// }

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}