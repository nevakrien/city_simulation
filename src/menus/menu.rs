use bevy::{
    app::AppExit, 
    color::palettes::css::CRIMSON, 
    prelude::*
};

use crate::{
    game::PlayState,
    globals::GameState,
    menus::{
        despawn_screen,
        settings::{SettingsState, settings_sub_plugin},
        ui::{BasicButton, button_system, NORMAL_BUTTON, TEXT_COLOR},
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

        .add_plugins(settings_sub_plugin)

        .add_systems(OnEnter(GameState::Menu), menu_setup)
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)


        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system)//.run_if(in_state(GameState::Menu)),
        );
}

// // State used for the current menu screen
// #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
// pub enum MenuState {
//     Main,
//     Settings,
//     SettingsDisplay,
//     SettingsSound,
//     #[default]
//     Disabled,
// }

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}


// Tag component used to tag entities added on the main menu screen
#[derive(Component,Clone,Copy)]
pub struct OnMainMenuScreen;






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
                            BasicButton,
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
                            BasicButton,
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
                            BasicButton,
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


// All actions that can be triggered from a button click
#[derive(Component,Clone,Copy)]
pub enum MenuButtonAction {
    Play,
    ResumePlay,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

#[allow(clippy::type_complexity)]
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut settings_state: ResMut<NextState<SettingsState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut play_state: ResMut<NextState<PlayState>>,
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
                    settings_state.set(SettingsState::Disabled);
                }
                
                MenuButtonAction::ResumePlay => {
                    settings_state.set(SettingsState::Disabled);
                    play_state.set(PlayState::Play);
                }

                MenuButtonAction::Settings => {
                    menu_state.set(MenuState::Settings);
                    settings_state.set(SettingsState::Settings);
                }
                MenuButtonAction::SettingsDisplay => {
                    settings_state.set(SettingsState::Display);
                }
                MenuButtonAction::SettingsSound => {
                    settings_state.set(SettingsState::Sound);
                }
                MenuButtonAction::BackToMainMenu => {
                    game_state.set(GameState::Menu);
                    menu_state.set(MenuState::Main);
                    play_state.set(PlayState::Disabled);
                    settings_state.set(SettingsState::Disabled);
                }
                MenuButtonAction::BackToSettings => {
                    settings_state.set(SettingsState::Settings);
                },
            }
        }
    }
}
