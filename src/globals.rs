use bevy::prelude::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

use bevy::render::extract_resource::ExtractResource;
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States,Resource)]
pub enum RenderState {
    #[default]
    Off,
    GameOfLife,
}

impl ExtractResource for RenderState {
    type Source = RenderState;
    
    fn extract_resource(state: &Self::Source) -> Self {
        *state
    }
}
