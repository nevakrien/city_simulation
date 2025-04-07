use crate::menus::ui::create_setting_text;
use bevy::{
    prelude::*,
    input::{
        ButtonInput,
        keyboard::KeyCode,
    },
    color::palettes::basic::{BLUE, LIME},
};

use crate::{
    common::despawn_screen,
    globals::{GameState, DisplayQuality, Volume},
    graphics::{ATTRIBUTE_BLEND_COLOR, CustomMaterial, DumbyMatrial},
    menus::{
        settings::SettingsState,
        ui::TEXT_COLOR,
    },
};


// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PlayState {
    Settings,
    Play,
    #[default]
    Disabled,
}


/// trying to run some intresting shaders


// This plugin will contain the game with an animated shader
pub fn game_plugin(app: &mut App) {
    app.add_plugins((
        MaterialPlugin::<DumbyMatrial>::default(),
        MaterialPlugin::<CustomMaterial>::default(),
        ))

        .init_state::<PlayState>() 


        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, game.run_if(in_state(PlayState::Play)))
        .add_systems(Update, toggle_settings_with_escape.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);



fn game_setup(
    mut commands: Commands,
    display_quality: Res<DisplayQuality>,
    volume: Res<Volume>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut dumby_materials: ResMut<Assets<DumbyMatrial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,

    mut next_play_state: ResMut<NextState<PlayState>>,
) {
    next_play_state.set(PlayState::Play);

    // Spawn the 3D cube with custom shader material
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(dumby_materials.add(DumbyMatrial {})),
        Transform::from_xyz(1.0, 1.2, 0.4),
        OnGameScreen, // Tag it so it gets despawned correctly
    ));

    //spawn custom shader cube
    let mesh = Mesh::from(Cuboid::default())
        // Sets the custom attribute
        .with_inserted_attribute(
            ATTRIBUTE_BLEND_COLOR,
            // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
            vec![[1.0, 0.0, 0.0, 1.0]; 24],
        );

    // cube
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(custom_materials.add(CustomMaterial {
            color: LinearRgba::WHITE,
        })),
        Transform::from_xyz(-1.0, 1.2, 0.4),
        OnGameScreen, // Tag it so it gets despawned correctly
    ));

    // Add a camera to view the 3D scene
    commands.spawn((
        Camera{
            order:10,
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        OnGameScreen, // Tag it so it gets despawned correctly
    ));

    // Spawn a 5 seconds timer to trigger going back to the menu
    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut play_state: ResMut<NextState<PlayState>>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
        play_state.set(PlayState::Disabled);
    }
}


fn toggle_settings_with_escape(
    keys: Res<ButtonInput<KeyCode>>,
    play_state: Res<State<PlayState>>,
    settings_state: Res<State<SettingsState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
    mut next_settings_state: ResMut<NextState<SettingsState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match (play_state.get(), settings_state.get()) {
            (PlayState::Play, SettingsState::Disabled) => {
                next_play_state.set(PlayState::Settings);
                next_settings_state.set(SettingsState::Settings);
            }
            (PlayState::Settings, SettingsState::Settings) => {
                next_settings_state.set(SettingsState::Disabled);
                next_play_state.set(PlayState::Play);
            }
            _ => {}
        }
    }
}


