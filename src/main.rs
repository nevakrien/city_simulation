//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

use std::path::Path;
use city_simulation::settings_io::SettingPlugin;
use bevy::prelude::*;


use city_simulation::globals::GameState;
use city_simulation::game;
use city_simulation::globals::{DisplayQuality, Volume};
use city_simulation::menus::{menu, splash};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        // Insert as resource the initial value for the settings resources
        

        .add_plugins((
            SettingPlugin::new(Path::new("assets/settings/volume.json"),Volume(70)),
            SettingPlugin::new(Path::new("assets/settings/quality.json"),DisplayQuality::Medium),

        ))
        // .insert_resource(DisplayQuality::Medium)
        // .insert_resource(Volume(70))


        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
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
    // commands.spawn(Camera3d::default());
}