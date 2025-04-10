use bevy::{
    prelude::*,
    input::{
        ButtonInput,
        keyboard::KeyCode,
    },
    sprite::{Wireframe2dConfig, Wireframe2dPlugin}
};

use crate::{
    common::{despawn_screen,StageSelect},
    settings::globals::{DisplayQuality, Volume},
    // graphics::{ATTRIBUTE_BLEND_COLOR, CustomMaterial, DumbyMatrial},
    menus::{
        settings::SettingsState,
    },
    graphics::{graphics_plugin,CustomMaterial},
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
    app
        .add_plugins((graphics_plugin,Wireframe2dPlugin::default(),))
        .init_state::<PlayState>() 


        .add_systems(OnEnter(StageSelect::Game), game_setup)
        .add_systems(Update, camera_control_system_2d.run_if(in_state(PlayState::Play)))
        .add_systems(Update, (toggle_settings_with_escape,toggle_wireframe).run_if(in_state(StageSelect::Game)))
        .add_systems(OnExit(StageSelect::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;


fn game_setup(
    mut commands: Commands,
    _display_quality: Res<DisplayQuality>,
    _volume: Res<Volume>,

    asset_server: Res<AssetServer>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,

    mut next_play_state: ResMut<NextState<PlayState>>,
) {
    next_play_state.set(PlayState::Play);

    //simple shapes
    let circle = meshes.add(Circle::new(50.0));

    commands.spawn((
        Mesh2d(circle.clone()),
        MeshMaterial2d(color_materials.add(Color::srgb(0.15, 0.3, 0.9))),
        Transform::from_xyz(39.0, 40.0, 100.0),

    ));

    commands.spawn((
        Mesh2d(circle),
        MeshMaterial2d(color_materials.add(Color::srgba(0.3, 0.2, 0.9,0.3))),
        Transform::from_xyz(0.0,0.0,40.0),

    ));

    //custom shader
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(custom_materials.add(CustomMaterial {
            color: LinearRgba::RED,
            color_texture: Some(asset_server.load("bevy_examples/branding/icon.png")),
        })),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));

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

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}


fn camera_control_system_2d(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut transform = match query.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let delta = time.delta_secs();
    let speed = 300.0;

    let mut direction = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        transform.translation += direction.extend(0.0) * speed * delta;
    }
}