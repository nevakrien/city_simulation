use std::fmt;
use bevy::prelude::*;

use serde::{Deserialize, Serialize};


pub trait Slidble{
    fn as_fraction(&self) -> f32;
    fn from_fraction(fraction: f32) -> Self;
}


// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource,Component,Serialize,Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource,Component,Serialize,Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Volume(pub u32);


impl Slidble for Volume {
     fn as_fraction(&self) -> f32 {
        self.0 as f32/100.0
    }
    
     fn from_fraction(fraction: f32) -> Self {
        Self((fraction * 100.0) as u32)
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:3}", self.0)
    }
}


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