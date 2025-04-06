// use crate::systems::ui::TEXT_COLOR;
// use bevy::{
//     color::palettes::basic::{BLUE, LIME},
//     prelude::*,
// };

// use crate::globals::{GameState};
// use crate::resources::{DisplayQuality, Volume};
// use crate::systems::despawn_screen;

// // This plugin will contain the game. In this case, it's just be a screen that will
// // display the current settings for 5 seconds before returning to the menu
// pub fn game_plugin(app: &mut App) {
//     app.add_systems(OnEnter(GameState::Game), game_setup)
//         .add_systems(Update, game.run_if(in_state(GameState::Game)))
//         .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
// }

// // Tag component used to tag entities added on the game screen
// #[derive(Component)]
// pub struct OnGameScreen;

// #[derive(Resource, Deref, DerefMut)]
// struct GameTimer(Timer);

// fn game_setup(
//     mut commands: Commands,
//     display_quality: Res<DisplayQuality>,
//     volume: Res<Volume>,
// ) {
//     commands
//         .spawn((
//             Node {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 // center children
//                 align_items: AlignItems::Center,
//                 justify_content: JustifyContent::Center,
//                 ..default()
//             },
//             OnGameScreen,
//         ))
//         .with_children(|parent| {
//             // First create a `Node` for centering what we want to display
//             parent
//                 .spawn((
//                     Node {
//                         // This will display its children in a column, from top to bottom
//                         flex_direction: FlexDirection::Column,
//                         // `align_items` will align children on the cross axis. Here the main axis is
//                         // vertical (column), so the cross axis is horizontal. This will center the
//                         // children
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     BackgroundColor(Color::BLACK),
//                 ))
//                 .with_children(|p| {
//                     p.spawn((
//                         Text::new("Will be back to the menu shortly..."),
//                         TextFont {
//                             font_size: 67.0,
//                             ..default()
//                         },
//                         TextColor(TEXT_COLOR),
//                         Node {
//                             margin: UiRect::all(Val::Px(50.0)),
//                             ..default()
//                         },
//                     ));
//                     p.spawn((
//                         Text::default(),
//                         Node {
//                             margin: UiRect::all(Val::Px(50.0)),
//                             ..default()
//                         },
//                     ))
//                     .with_children(|p| {
//                         p.spawn((
//                             TextSpan(format!("quality: {:?}", *display_quality)),
//                             TextFont {
//                                 font_size: 50.0,
//                                 ..default()
//                             },
//                             TextColor(BLUE.into()),
//                         ));
//                         p.spawn((
//                             TextSpan::new(" - "),
//                             TextFont {
//                                 font_size: 50.0,
//                                 ..default()
//                             },
//                             TextColor(TEXT_COLOR),
//                         ));
//                         p.spawn((
//                             TextSpan(format!("volume: {:?}", *volume)),
//                             TextFont {
//                                 font_size: 50.0,
//                                 ..default()
//                             },
//                             TextColor(LIME.into()),
//                         ));
//                     });
//                 });
//         });
//     // Spawn a 5 seconds timer to trigger going back to the menu
//     commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
// }

// // Tick the timer, and change state when finished
// fn game(
//     time: Res<Time>,
//     mut game_state: ResMut<NextState<GameState>>,
//     mut timer: ResMut<GameTimer>,
// ) {
//     if timer.tick(time.delta()).finished() {
//         game_state.set(GameState::Menu);
//     }
// }


//copied from https://github.com/bevyengine/bevy/blob/main/examples/shader/compute_shader_game_of_life.rs
//! A compute shader that simulates Conway's Game of Life.
//!
//! Compute shaders use the GPU for computing arbitrary information, that may be independent of what
//! is rendered to the screen.

use crate::globals::RenderState;
use crate::GameState;

use bevy::{
    state::app::StatesPlugin,
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_asset::{RenderAssetUsages, RenderAssets},
        render_graph::{self, RenderGraph, RenderLabel},
        render_resource::{binding_types::texture_storage_2d, *},
        renderer::{RenderContext, RenderDevice},
        texture::GpuImage,
        Render, RenderApp, RenderSet,
    },
};
use std::borrow::Cow;

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/game_of_life.wgsl";

const DISPLAY_FACTOR: u32 = 4;
const SIZE: (u32, u32) = (1280 / DISPLAY_FACTOR, 720 / DISPLAY_FACTOR);
const WORKGROUP_SIZE: u32 = 8;


fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::R32Float,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let image0 = images.add(image.clone());
    let image1 = images.add(image);

    commands.spawn((
        Sprite {
            image: image0.clone(),
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        Transform::from_scale(Vec3::splat(DISPLAY_FACTOR as f32)),
    ));

    commands.insert_resource(GameOfLifeImages {
        texture_a: image0,
        texture_b: image1,
    });
}

// Switch texture to display every frame to show the one that was written to most recently.
fn switch_textures(images: Res<GameOfLifeImages>, mut sprite: Single<&mut Sprite>) {
    if sprite.image == images.texture_a {
        sprite.image = images.texture_b.clone_weak();
    } else {
        sprite.image = images.texture_a.clone_weak();
    }
}

pub struct GameOfLifeComputePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct GameOfLifeLabel;

impl Plugin for GameOfLifeComputePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update, switch_textures.run_if(in_state(GameState::Game)))

        .add_systems(OnEnter(GameState::Game),|mut r:ResMut<RenderState>| {*r=RenderState::GameOfLife})
        .add_systems(OnExit(GameState::Game),|mut r:ResMut<RenderState>| {*r=RenderState::Off})
        ;

        // Extract the game of life image resource from the main world into the render world
        // for operation on by the compute shader and display on the sprite.
        app.add_plugins(ExtractResourcePlugin::<GameOfLifeImages>::default())
        .add_plugins(ExtractResourcePlugin::<RenderState>::default())

        ;

        let render_app = app.sub_app_mut(RenderApp);
        render_app

        .add_plugins(StatesPlugin)
        .init_state::<RenderState>()
        .add_systems(
            Update,
            |command:Res<RenderState>,mut state:ResMut<NextState<RenderState>>| {state.set(*command)}
        )
        .add_systems(
            Render,
            prepare_bind_group.run_if(in_state(RenderState::GameOfLife)).in_set(RenderSet::PrepareBindGroups),
        )
        .add_systems(
            OnEnter(RenderState::GameOfLife),
            |mut render_graph : ResMut<RenderGraph>| {
                println!("runing enter");
                render_graph.add_node(GameOfLifeLabel, GameOfLifeNode::default());
            }
        )
        .add_systems(
            OnExit(RenderState::GameOfLife),
            |mut render_graph : ResMut<RenderGraph>| {
                println!("runing exit");
                render_graph.remove_node(GameOfLifeLabel).unwrap();
            }
        )


        ;

        // let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        // render_graph.add_node(GameOfLifeLabel, GameOfLifeNode::default());
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<GameOfLifePipeline>();
    }
}

#[derive(Resource, Clone, ExtractResource)]
struct GameOfLifeImages {
    texture_a: Handle<Image>,
    texture_b: Handle<Image>,
}

#[derive(Resource)]
struct GameOfLifeImageBindGroups([BindGroup; 2]);

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<GameOfLifePipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    game_of_life_images: Res<GameOfLifeImages>,
    render_device: Res<RenderDevice>,
) {
    let view_a = gpu_images.get(&game_of_life_images.texture_a).unwrap();
    let view_b = gpu_images.get(&game_of_life_images.texture_b).unwrap();
    let bind_group_0 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_a.texture_view, &view_b.texture_view)),
    );
    let bind_group_1 = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((&view_b.texture_view, &view_a.texture_view)),
    );
    commands.insert_resource(GameOfLifeImageBindGroups([bind_group_0, bind_group_1]));
}

#[derive(Resource)]
struct GameOfLifePipeline {
    texture_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for GameOfLifePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = render_device.create_bind_group_layout(
            "GameOfLifeImages",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    texture_storage_2d(TextureFormat::R32Float, StorageTextureAccess::ReadOnly),
                    texture_storage_2d(TextureFormat::R32Float, StorageTextureAccess::WriteOnly),
                ),
            ),
        );
        let shader = world.load_asset(SHADER_ASSET_PATH);
        let pipeline_cache = world.resource::<PipelineCache>();
        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
            zero_initialize_workgroup_memory: false,
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
            zero_initialize_workgroup_memory: false,
        });

        GameOfLifePipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum GameOfLifeState {
    Loading,
    Init,
    Update(usize),
}

struct GameOfLifeNode {
    state: GameOfLifeState,
}

impl Default for GameOfLifeNode {
    fn default() -> Self {
        Self {
            state: GameOfLifeState::Loading,
        }
    }
}

impl render_graph::Node for GameOfLifeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<GameOfLifePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            GameOfLifeState::Loading => {
                match pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline) {
                    CachedPipelineState::Ok(_) => {
                        self.state = GameOfLifeState::Init;
                    }
                    CachedPipelineState::Err(err) => {
                        panic!("Initializing assets/{SHADER_ASSET_PATH}:\n{err}")
                    }
                    _ => {}
                }
            }
            GameOfLifeState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = GameOfLifeState::Update(1);
                }
            }
            GameOfLifeState::Update(0) => {
                self.state = GameOfLifeState::Update(1);
            }
            GameOfLifeState::Update(1) => {
                self.state = GameOfLifeState::Update(0);
            }
            GameOfLifeState::Update(_) => unreachable!(),
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_groups = &world.resource::<GameOfLifeImageBindGroups>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<GameOfLifePipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        // select the pipeline based on the current state
        match self.state {
            GameOfLifeState::Loading => {}
            GameOfLifeState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[0], &[]);
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            GameOfLifeState::Update(index) => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_bind_group(0, &bind_groups[index], &[]);
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
        }

        Ok(())
    }
}