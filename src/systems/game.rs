use crate::systems::ui::TEXT_COLOR;
use bevy::{
    color::palettes::basic::{BLUE, LIME},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::globals::GameState;
use crate::resources::{DisplayQuality, Volume};
use crate::systems::despawn_screen;

/// Path to the shader file in assets directory
const SHADER_ASSET_PATH: &str = "shaders/animate_shader.wgsl";

// This plugin will contain the game with an animated shader
pub fn game_plugin(app: &mut App) {
    app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

// Define the custom material for our shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

fn game_setup(
    mut commands: Commands,
    display_quality: Res<DisplayQuality>,
    volume: Res<Volume>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // Setup the UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // center children
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            // First create a `Node` for centering what we want to display
            parent
                .spawn((
                    Node {
                        // This will display its children in a column, from top to bottom
                        flex_direction: FlexDirection::Column,
                        // `align_items` will align children on the cross axis. Here the main axis is
                        // vertical (column), so the cross axis is horizontal. This will center the
                        // children
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Will be back to the menu shortly..."),
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
                    p.spawn((
                        Text::default(),
                        Node {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        },
                    ))
                    .with_children(|p| {
                        p.spawn((
                            TextSpan(format!("quality: {:?}", *display_quality)),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                            TextColor(BLUE.into()),
                        ));
                        p.spawn((
                            TextSpan::new(" - "),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ));
                        p.spawn((
                            TextSpan(format!("volume: {:?}", *volume)),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                            TextColor(LIME.into()),
                        ));
                    });
                });
        });

    // Spawn the 3D cube with custom shader material
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {})),
        Transform::from_xyz(1.0, 1.2, 0.4),
        OnGameScreen, // Tag it so it gets despawned correctly
    ));

    // Add a camera to view the 3D scene
    commands.spawn((
        Camera{
            order:-10,
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
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}