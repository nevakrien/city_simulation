use std::{fs, path::Path, time::Duration};

use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin,debug::DiagnosticsPlugin, FramepaceSettings, Limiter};
use serde::{Deserialize, Serialize};

/// Serializable version of the frame limiter setting
#[derive(Resource, Debug, Clone,PartialEq, Serialize, Deserialize)]
pub enum FramerateLimiter {
    Auto,
    Manual(f64), // FPS
    Off,
}

impl Default for FramerateLimiter {
    fn default() -> Self {
        Self::Auto
    }
}

/// Converts from your config type to the actual limiter
impl From<&FramerateLimiter> for Limiter {
    fn from(value: &FramerateLimiter) -> Self {
        match value {
            FramerateLimiter::Auto => Limiter::Auto,
            FramerateLimiter::Manual(fps) => {
                Limiter::Manual(Duration::from_secs_f64(1.0 / fps.max(1e-6)))
            }
            FramerateLimiter::Off => Limiter::Off,
        }
    }
}

/// Converts from `FramepaceSettings` back to your config
impl From<&Limiter> for FramerateLimiter {
    fn from(limiter: &Limiter) -> Self {
        match limiter {
            Limiter::Auto => FramerateLimiter::Auto,
            Limiter::Manual(duration) => {
                let secs = duration.as_secs_f64();
                if secs > 0.0 {
                    FramerateLimiter::Manual(1.0 / secs)
                } else {
                    FramerateLimiter::Manual(240.0)
                }
            }
            Limiter::Off => FramerateLimiter::Off,
        }
    }
}

/// Internal resource to track where the config is stored
#[derive(Resource)]
struct FramerateConfigPath(&'static Path);

/// Plugin for framerate limiter config loading and application
pub struct FrameratePluginWithPath {
    pub config_path: &'static Path,
    pub default: FramerateLimiter,
}
impl Default for FrameratePluginWithPath {
    fn default() -> Self {
        Self {
            config_path: Path::new("config/framerate.json"),
            default: FramerateLimiter::Manual(60.0),
        }
    }
}


impl Plugin for FrameratePluginWithPath {
    fn build(&self, app: &mut App) {
        app.add_plugins((FramepacePlugin,/*DiagnosticsPlugin*/))
            .insert_resource(self.default.clone())
            .insert_resource(FramerateConfigPath(self.config_path))
            .add_systems(Startup, (
                load_framerate_config,
                apply_framerate_config.run_if(resource_changed::<FramerateLimiter>),
            ));
    }
}

/// Loads config from disk (if present)
fn load_framerate_config(
    mut config: ResMut<FramerateLimiter>,
    path: Res<FramerateConfigPath>,
) {
    let path = path.0;
    if path.exists() {
        match fs::read_to_string(path) {
            Ok(contents) => match serde_json::from_str::<FramerateLimiter>(&contents) {
                Ok(loaded) => {
                    *config = loaded;
                    info!("Framerate config loaded from {}", path.display());
                }
                Err(e) => {
                    error!("Failed to parse framerate config: {e}");
                }
            },
            Err(e) => {
                error!("Failed to read framerate config: {e}");
            }
        }
    } else {
        info!("No framerate config found at {}, using default", path.display());
    }
}

/// Applies the loaded config to the actual `FramepaceSettings` resource
fn apply_framerate_config(
    config: Res<FramerateLimiter>,
    mut framepace: ResMut<FramepaceSettings>,
) {
    framepace.limiter = Limiter::from(&*config);
}

/// Manually save the framerate config to disk
pub fn save_framerate_config(world: &mut World) -> Result<(), String> {
    let settings = world.get_resource::<FramepaceSettings>()
        .ok_or("Missing FramepaceSettings")?;

    let path = world.get_resource::<FramerateConfigPath>()
        .ok_or("Missing FramerateConfigPath")?;

    let config = FramerateLimiter::from(&settings.limiter);
    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {e}"))?;

    if let Some(parent) = path.0.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {e}"))?;
    }

    fs::write(path.0, json)
        .map_err(|e| format!("Failed to write framerate config: {e}"))?;

    Ok(())
}

/// A system wrapper to call `save_framerate_config()` in schedules
pub fn save_framerate_config_system(world: &mut World) {
    if let Err(e) = save_framerate_config(world) {
        error!("Failed to save framerate config: {e}");
    } else {
        info!("Framerate config saved.");
    }
}
