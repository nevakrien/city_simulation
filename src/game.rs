use bevy::{
    prelude::*,
    input::{
        ButtonInput,
        keyboard::KeyCode,
    },
};

use crate::{
    common::despawn_screen,
    globals::{GameState, DisplayQuality, Volume},
    graphics::{ATTRIBUTE_BLEND_COLOR, CustomMaterial, DumbyMatrial},
    menus::{
        settings::SettingsState,
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
        .add_systems(Update, camera_control_system.run_if(in_state(PlayState::Play)))
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
    // commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

// // Tick the timer, and change state when finished
// fn game(
//     time: Res<Time>,
//     mut game_state: ResMut<NextState<GameState>>,
//     mut play_state: ResMut<NextState<PlayState>>,
//     // mut timer: ResMut<GameTimer>,
// ) {
//     if timer.tick(time.delta()).finished() {
//         game_state.set(GameState::Menu);
//         play_state.set(PlayState::Disabled);
//     }
// }


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


// fn camera_movement_system(
//     time: Res<Time>,
//     keys: Res<ButtonInput<KeyCode>>,
//     mut query: Query<&mut Transform, With<Camera3d>>,
// ) {
//     let mut transform = match query.get_single_mut() {
//         Ok(t) => t,
//         Err(_) => return,
//     };

//     let mut direction = Vec3::ZERO;
//     let forward = *transform.forward();
//     let right = *transform.right();
//     let up = Vec3::Y;

//     let speed = 5.0;
//     let rotation_speed = std::f32::consts::PI; // radians per second
//     let delta = time.delta_secs();

//     // Translation movement (WASD + QE)
//     if keys.pressed(KeyCode::KeyW) {
//         direction += forward;
//     }
//     if keys.pressed(KeyCode::KeyS) {
//         direction -= forward;
//     }
//     if keys.pressed(KeyCode::KeyA) {
//         direction -= right;
//     }
//     if keys.pressed(KeyCode::KeyD) {
//         direction += right;
//     }
//     if keys.pressed(KeyCode::KeyE) {
//         direction += up;
//     }
//     if keys.pressed(KeyCode::KeyQ) {
//         direction -= up;
//     }

//     if direction.length_squared() > 0.0 {
//         transform.translation += direction.normalize() * speed * delta;
//     }

//     // Rotation with arrow keys
//     let mut yaw = 0.0;
//     let mut pitch = 0.0;

//     if keys.pressed(KeyCode::ArrowLeft) {
//         yaw += 1.0;
//     }
//     if keys.pressed(KeyCode::ArrowRight) {
//         yaw -= 1.0;
//     }
//     if keys.pressed(KeyCode::ArrowUp) {
//         pitch += 1.0;
//     }
//     if keys.pressed(KeyCode::ArrowDown) {
//         pitch -= 1.0;
//     }

//     if yaw != 0.0 {
//         transform.rotate_y(yaw * rotation_speed * delta);
//     }

//     if pitch != 0.0 {
//         // Local X axis for pitch
//         transform.rotate_local_x(pitch * rotation_speed * delta);
//     }
// }

fn camera_control_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut transform = match query.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let delta = time.delta_secs();
    let move_speed = 5.0;
    let rot_speed = 0.7*std::f32::consts::PI; // radians per sec

    let is_translation_mode = !keys.pressed(KeyCode::AltLeft);

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    if keys.pressed(KeyCode::KeyA) {
        x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        x += 1.0;
    }
    if keys.pressed(KeyCode::KeyW) {
        z += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        z -= 1.0;
    }
    if keys.pressed(KeyCode::KeyE) {
        y += 1.0;
    }
    if keys.pressed(KeyCode::KeyQ) {
        y -= 1.0;
    }

    if is_translation_mode {
        // move in world space (relative to camera)
        let forward = *transform.forward();
        let right = *transform.right();
        let up = *transform.up();

        let mut direction = Vec3::ZERO;
        direction += x * right;
        direction += y * up;
        direction += z * forward;

        if direction.length_squared() > 0.0 {
            transform.translation += direction/*.normalize()*/ * move_speed * delta;
        }
    } else {
        // rotate in local space
        // A/D → yaw, W/S → pitch, Q/E → roll

        // flip signs to match "natural" FPS-style camera feel
        if x != 0.0 {
            transform.rotate_local_y(-x * rot_speed * delta); // yaw (horizontal)
        }
        if z != 0.0 {
            transform.rotate_local_x(z * rot_speed * delta); // pitch (look up/down)
        }
        if y != 0.0 {
            transform.rotate_local_z(-y * rot_speed * delta); // roll (twist)
        }
    }
}
