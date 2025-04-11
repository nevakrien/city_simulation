//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

use bevy_framepace::debug::DiagnosticsPlugin;
use iyes_perf_ui::PerfUiPlugin;
use iyes_perf_ui::entries::PerfUiAllEntries;
use std::path::Path;
use bevy::{
    prelude::*,
    diagnostic::{LogDiagnosticsPlugin,FrameTimeDiagnosticsPlugin}
};

use city_simulation::{
    game,
    menus::{menu, splash},
    common::StageSelect,
    settings::{
        globals::{DisplayQuality, Volume},
        settings_io::SettingPlugin,
        framerate::{framerate_plugin}
    }
};



fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),

            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            bevy::render::diagnostic::RenderDiagnosticsPlugin,

            PerfUiPlugin,
            
            // LogDiagnosticsPlugin::default(),
        ))
        
        // Insert as resource the initial value for the settings resources
        

        .add_plugins((            
            framerate_plugin,
            SettingPlugin::new(Path::new("assets/settings/volume.json"),Volume(70)),
            SettingPlugin::new(Path::new("assets/settings/quality.json"),DisplayQuality::Medium),

        ))
        // .insert_resource(DisplayQuality::Medium)
        // .insert_resource(Volume(70))


        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<StageSelect>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(

    (Camera{
            order:0,
            ..default()
    },Camera2d
    )
    );

    // commands.spawn((
    //     PerfUiEntryFPS::default(),
    //     PerfUiEntryClock::default(),
    // ));
    commands.spawn(PerfUiAllEntries::default());
    // commands.spawn(Camera3d::default());
}