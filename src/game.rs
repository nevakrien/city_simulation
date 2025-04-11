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
        .insert_resource(DragTarget::default())

        .add_systems(OnEnter(StageSelect::Game), game_setup)
        .add_systems(Update, (
                
                camera_control_system_2d,
                select_drag_target_system,
                apply_drag_target_system,
                update_lines_system
        
        ).run_if(in_state(PlayState::Play)))
        
        .add_systems(Update, (toggle_settings_with_escape,toggle_wireframe).run_if(in_state(StageSelect::Game)))
        .add_systems(OnExit(StageSelect::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Line {
    pub from: Entity,
    pub to: Entity,
}


fn line_between(a: &Vec2, b: &Vec2) -> (Vec2, f32, f32) {
    let mid = (*a + *b) / 2.0;
    let dir = *b - *a;
    (mid, dir.y.atan2(dir.x), dir.length())
}

fn spawn_circle(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    position: Vec2,
    z: f32,
    color: Color,
) -> Entity {
    let mesh = meshes.add(Circle::new(50.0));
    commands
        .spawn((
            OnGameScreen,
            Draggable,
            Mesh2d(mesh),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(position.x, position.y, z),
        ))
        .id()
}

fn spawn_line(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    a_entity: Entity,
    b_entity: Entity,
    a_pos: Vec2,
    b_pos: Vec2,
) {
    let (mid, angle, length) = line_between(&a_pos, &b_pos);
    let mesh = meshes.add(Rectangle::new(length, 4.0));
    commands.spawn((
        OnGameScreen,
        Line { from: a_entity, to: b_entity },
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform {
            translation: mid.extend(-10.0),
            rotation: Quat::from_rotation_z(angle),
            ..Default::default()
        },
    ));
}

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

    let z = 40.0;
    let color1 = Color::srgb(0.15, 0.3, 0.9);
    let color2 = Color::srgb(0.3, 0.2, 0.9);

    let positions = [
        Vec2::new(0.0, 0.0),
        Vec2::new(150.0, 80.0),
        Vec2::new(150.0, -80.0),
        Vec2::new(-150.0, 80.0),
        Vec2::new(-150.0, -80.0),
    ];

    let mut entities = vec![];
    for (i, pos) in positions.iter().enumerate() {
        let color = if i % 2 == 0 { color1 } else { color2 };
        let e = spawn_circle(&mut commands, &mut meshes, &mut color_materials, *pos, z, color);
        entities.push((e, *pos));
    }

    // Draw edges from center node (0) to all others
    for (target_entity, target_pos) in &entities[1..] {
        spawn_line(
            &mut commands,
            &mut meshes,
            &mut color_materials,
            entities[0].0,
            *target_entity,
            entities[0].1,
            *target_pos,
        );
    }

    // Custom shader rectangle
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(custom_materials.add(CustomMaterial {
            color: LinearRgba::RED,
            color_texture: Some(asset_server.load("bevy_examples/branding/icon.png")),
        })),
        Transform::from_xyz(300.0, 100.0, z).with_scale(Vec3::splat(128.)),
    ));
}


#[derive(Resource, Default, Debug)]
pub struct DragTarget(pub Option<Entity>);


//TODO make this less janky
fn select_drag_target_system(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    draggable_q: Query<(Entity, &Transform), With<Draggable>>,
    mut drag_target: ResMut<DragTarget>,
) {
    let Ok((camera, cam_transform)) = camera_q.get_single() else { return };
    let Ok(window) = windows.get_single() else { return };
    let Some(cursor) = window.cursor_position() else { return };

    let world_pos = camera.viewport_to_world_2d(cam_transform, cursor);
    let Ok(world_pos) = world_pos else { return };

    if buttons.just_pressed(MouseButton::Left) {
        let closest = draggable_q
            .iter()
            .filter(|(_, transform)| transform.translation.truncate().distance(world_pos) < 50.0)
            .min_by(|(_, a), (_, b)| {
                a.translation
                    .truncate()
                    .distance_squared(world_pos)
                    .partial_cmp(&b.translation.truncate().distance_squared(world_pos))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        drag_target.0 = closest.map(|(e, _)| e);
    }

    if buttons.just_released(MouseButton::Left) {
        drag_target.0 = None;
    }
}

fn apply_drag_target_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut draggable_q: Query<&mut Transform, With<Draggable>>,
    drag_target: Res<DragTarget>,
) {
    let Some(target_entity) = drag_target.0 else { return };

    let Ok((camera, cam_transform)) = camera_q.get_single() else { return };
    let Ok(window) = windows.get_single() else { return };
    let Some(cursor) = window.cursor_position() else { return };

    let world_pos = match camera.viewport_to_world_2d(cam_transform, cursor).ok() {
        Some(p) => p,
        None => return,
    };

    if let Ok(mut transform) = draggable_q.get_mut(target_entity) {
        let current = transform.translation.truncate();
        let target = world_pos;

        // Smooth follow
        let speed = 20.0;
        let dt = 1.0 / 60.0; // could use Time.delta_seconds()
        let new_pos = current + (target - current) * ((speed * dt) as f32 ).min(1.0);

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}


fn update_lines_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_q: Query<(&Line, &mut Mesh2d, &mut Transform),Without<Draggable>>,
    transform_q: Query<&Transform, With<Draggable>>,
) {
    for (line, mut mesh, mut transform) in &mut line_q {
        let Ok(from) = transform_q.get(line.from) else { continue };
        let Ok(to) = transform_q.get(line.to) else { continue };

        let from_pos = from.translation.truncate();
        let to_pos = to.translation.truncate();

        let (mid, angle, length) = line_between(&from_pos, &to_pos);

        *mesh = Mesh2d(meshes.add(Rectangle::new(length, 4.0))); // Resize mesh
        transform.translation = mid.extend(transform.translation.z); // Update position
        transform.rotation = Quat::from_rotation_z(angle); // Update rotation
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
    let zoom_speed: f32 = 1.5;

    let mut direction = Vec3::ZERO;

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

    if direction != Vec3::ZERO {
        transform.translation += direction * speed * delta;
    }


     // Zoom (scale adjustment)
    let mut scale_change = 0.0;
    if keys.pressed(KeyCode::KeyQ) {
        scale_change += 1.0;
    }
    if keys.pressed(KeyCode::KeyE) {
        scale_change -= 1.0;
    }
    if scale_change != 0.0 {
        let scale_delta = zoom_speed.powf(scale_change * delta);
        transform.scale *= Vec3::splat(scale_delta);

        // Optional: clamp zoom range
        transform.scale = transform.scale.clamp(
            Vec3::splat(0.1),
            Vec3::splat(10.0),
        );
    }
}