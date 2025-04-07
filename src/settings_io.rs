use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

/// Plugin for handling settings persistence
pub struct SettingPlugin<T: 'static + Resource + Serialize + for<'de> Deserialize<'de> + Clone> {
    path: &'static Path,
    default_value: T,
    _phantom: PhantomData<T>,
}

impl<T: 'static + Resource + Serialize + for<'de> Deserialize<'de> + Clone> SettingPlugin<T> {
    pub fn new(path: &'static Path, default_value: T) -> Self {
        Self {
            path,
            default_value,
            _phantom: PhantomData,
        }
    }
}

impl<T: 'static + Resource + Serialize + for<'de> Deserialize<'de> + Clone> Plugin for SettingPlugin<T> {
    fn build(&self, app: &mut App) {
        // Insert the resource with the provided default value
        app.insert_resource(self.default_value.clone())
           // Insert the path
           .insert_resource(SettingsPath::<T>::new(self.path))
           // Load settings during startup
           .add_systems(Startup, load_settings_system::<T>);
    }
}

/// Resource to store the path for a settings type
#[derive(Resource)]
pub struct SettingsPath<T> {
    pub path: &'static Path,
    _phantom: PhantomData<T>,
}

impl<T> SettingsPath<T> {
    pub fn new(path: &'static Path) -> Self {
        Self {
            path,
            _phantom: PhantomData,
        }
    }
}

/// System for loading settings from file
pub fn load_settings_system<T>(
    mut settings: ResMut<T>,
    path: Res<SettingsPath<T>>,
)
where
    T: Resource + Serialize + for<'de> Deserialize<'de> + Clone,
{
    let path = path.path;
    
    if path.exists() {
        match fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str::<T>(&contents) {
                    Ok(loaded_settings) => {
                        *settings = loaded_settings;
                        info!("Settings loaded from {}", path.display());
                    }
                    Err(e) => {
                        error!("Failed to parse settings from {}: {}", path.display(), e);
                        // Keep default settings
                        info!("Using default settings instead");
                    }
                }
            }
            Err(e) => {
                error!("Failed to read settings file {}: {}", path.display(), e);
                // Keep default settings
                info!("Using default settings instead");
            }
        }
    } else {
        // Don't create the file yet, we'll create it when saving
        info!("Settings file not found at {}, using defaults", path.display());
    }
}

/// Save settings to file
pub fn save_setting<T: Resource + Serialize>(world: &mut World) -> Result<(), String> {
    if let (Some(settings), Some(path_holder)) = (
        world.get_resource::<T>(),
        world.get_resource::<SettingsPath<T>>(),
    ) {
        let path = path_holder.path;
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                if let Err(e) = fs::create_dir_all(parent) {
                    return Err(format!("Failed to create settings directory: {}", e));
                }
            }
        }
        
        match serde_json::to_string_pretty(settings) {
            Ok(json) => {
                match fs::write(path, json) {
                    Ok(_) => {
                        debug!("Settings saved to {}", path.display());
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to write settings to {}: {}", path.display(), e);
                        Err(format!("Failed to write settings: {}", e))
                    }
                }
            }
            Err(e) => {
                error!("Failed to serialize settings: {}", e);
                Err(format!("Failed to serialize settings: {}", e))
            }
        }
    } else {
        Err("Settings or path resource not found".to_string())
    }
}

pub fn save_setting_system<T: Resource + Serialize>(world: &mut World){
    save_setting::<T>(world).unwrap();
}

/// Add a save system to run on exit from a state
pub fn add_setting_save_on_exit<T: 'static + Resource + Serialize, S: States + std::marker::Copy>(
    app: &mut App,
    state: S,
) {
    app.add_systems(OnExit(state), move |world: &mut World| {
        if let Err(e) = save_setting::<T>(world) {
            error!("Failed to save settings on exit from state {:?}: {}", state, e);
        } else {
            info!("Settings saved on exit from state {:?}", state);
        }
    });
}