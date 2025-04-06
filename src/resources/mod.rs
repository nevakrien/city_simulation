use bevy::prelude::*;

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

pub trait Slidble{
    fn as_fraction(&self) -> f32;
    fn from_fraction(fraction: f32) -> Self;
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);


impl Slidble for Volume {
     fn as_fraction(&self) -> f32 {
        self.0 as f32 / 10.0
    }
    
     fn from_fraction(fraction: f32) -> Self {
        Self((fraction * 10.0) as u32)
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
