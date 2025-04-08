use std::fmt;
use crate::settings_io::load_settings_system;
use crate::settings_io::SettingPlugin;
use crate::globals::Slidble;
use std::{path::Path, time::Duration};

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use serde::{Deserialize, Serialize};


#[derive(Resource,Component,Default, Debug, Clone,Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FramerateMode {
    #[default]
    Auto,
    Manual,
    Off,
}

// Separate VSync setting
#[derive(Resource,Component,Default, Debug, Clone,Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VsyncMode {
    #[default]
    Enabled,
    Disabled,
}

//only use for manual framerate mode
#[derive(Resource,Component, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ManualFpsCap(pub f64);


impl Default for ManualFpsCap {
    fn default() -> Self {
        ManualFpsCap(60.0)
    }
}

impl Slidble for ManualFpsCap{
    fn as_fraction(&self) -> f32{
        ((self.0-1.0)/199.0) as f32
    }
    fn from_fraction(fraction: f32) -> Self{
        ManualFpsCap(199.0 as f64*fraction as f64 +1.0)
    }
}

impl fmt::Display for ManualFpsCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.0} FPS", self.0)
    }
}

pub fn framerate_plugin(app: &mut App) {
    let fps_plugin = SettingPlugin::new(Path::new("assets/settings/fps_cap_slider.json"),ManualFpsCap(60.0)); 
    let mode_plugin = SettingPlugin::new(Path::new("assets/settings/fps_cap_mode.json"),FramerateMode::default()); 
    let vsync_plugin = SettingPlugin::new(Path::new("assets/settings/vsync_mode.json"),VsyncMode::default());

    app.add_plugins((FramepacePlugin,/*DiagnosticsPlugin,*/fps_plugin,mode_plugin,vsync_plugin))
        .add_systems(Startup, 
            (set_framerate, apply_vsync)
                .after(load_settings_system::<FramerateMode>)
                .after(load_settings_system::<ManualFpsCap>)
                .after(load_settings_system::<VsyncMode>))
        .add_systems(Update, 
            set_framerate.run_if(
                resource_changed::<FramerateMode>
                .or(resource_changed::<ManualFpsCap>)
            ))
        .add_systems(Update,
            apply_vsync.run_if(resource_changed::<VsyncMode>))
    ;
}

pub fn set_framerate(
    mode: Res<FramerateMode>,
    cap: Res<ManualFpsCap>,
    mut framepace: ResMut<FramepaceSettings>,
) {
    framepace.limiter = match *mode {
        FramerateMode::Auto => Limiter::Auto,
        FramerateMode::Manual => {
            Limiter::Manual(Duration::from_secs_f64(1.0 / cap.0.max(1e-6)))
        }
        FramerateMode::Off => Limiter::Off,
    };
}

pub fn apply_vsync(
    vsync_mode: Res<VsyncMode>,
    mut window: Single<&mut Window>,
) {
    window.present_mode = match *vsync_mode {
        VsyncMode::Enabled => PresentMode::AutoVsync,
        VsyncMode::Disabled => PresentMode::AutoNoVsync,
    };
}
