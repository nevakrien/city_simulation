#![allow(clippy::type_complexity)]//Queries shouldnt trigger clippy they are clearer that way

use std::{fmt::Display, marker::PhantomData};
use bevy::prelude::*;

use crate::globals::Slidble;

//colors

pub const COLOR_GRAY: Color = Color::srgb(0.5, 0.5, 0.5);
pub const COLOR_RED: Color = Color::srgb(1.0, 0.0, 0.0);
pub const COLOR_MAROON: Color = Color::srgb(0.5, 0.0, 0.0);
// pub const COLOR_CRIMSON: Color = Color::srgb(0.86, 0.08, 0.24);
pub const COLOR_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);

// Define UI color constants
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);



// Generic system that takes a component as a parameter, and will despawn all entities with that component
#[allow(dead_code)/*its not dead*/]
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
        // commands.entity(entity).despawn();
    }
}

//BUTONS

// Tag component to enable the basic button system
#[derive(Component)]
pub struct BasicButton;

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<BasicButton>),>,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
pub fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<BasicButton>)>,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();

    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            *previous_button_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

// Commented out old slider implementation
// ...

#[derive(Component)]
pub struct Slider<T: Resource> {
    /// The entity for the handle node
    pub handle: Entity,

    /// If you want to store the fraction or something else, you can.
    pub fraction: f32,
    pub _phantom: PhantomData<T>,
}



/// The handle node
#[derive(Component)]
pub struct SliderHandle<T>(pub PhantomData<T>);

#[derive(Component)]
pub struct SliderBar;


pub fn spawn_slider_system<R: Resource+Slidble, C: Component+Copy>(
    mut commands: Commands,
    node_config: Node,
    handle_node: Option<Node>,
    tag: C,
    parent: Option<Entity>,
    setting: Option<Res<R>>, // Add this to access the current resource value, optional to avoid system dependencies
) -> Entity {
    // Get the initial fraction from the current resource value if available
    let initial_fraction = setting.map_or(0.0, |res| res.as_fraction());
    
    // 1) Spawn the bar node with the provided node configuration
    let bar_entity = commands
        .spawn((
            // Use the provided node configuration
            node_config.clone(),
            BackgroundColor(COLOR_GRAY),
            SliderBar,
            Button, // Make it interactive
            Interaction::default(),
            tag,
            Name::new("UI Slider Bar"),
        ))
        .insert(RelativeCursorPosition::default())
        .id();
    
    // Get handle node, modifying only the left position to reflect initial value
    let mut custom_handle = handle_node.unwrap_or(Node {
        width: Val::Px(20.0),
        height: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(0.0),
        top: Val::Px(-5.0), // Slightly taller than the bar
        ..default()
    });
    
    // Get bar width for calculations
    let bar_width = match node_config.width {
        Val::Px(px) => px,
        _ => 200.0,
    };
    
    // Get handle width
    let handle_width = match custom_handle.width {
        Val::Px(px) => px,
        _ => 20.0,
    };
    
    // Calculate initial left position
    let max_left = bar_width - handle_width;
    custom_handle.left = Val::Px(initial_fraction * max_left);
    
    // 2) Spawn the handle node with adjusted position
    let handle_entity = commands
        .spawn((
            custom_handle,
            BackgroundColor(Color::WHITE),
            SliderHandle::<R>(PhantomData),
            tag,
            Name::new("UI Slider Handle"),
        ))
        .id();
    
    // Parent the handle under the bar, so it's a child in the UI tree
    commands.entity(bar_entity).add_child(handle_entity);
    
    // 3) Insert the Slider<R> on the bar, storing the handle's Entity
    commands.entity(bar_entity).insert(Slider::<R> {
        handle: handle_entity,
        fraction: initial_fraction,
        _phantom: PhantomData,
    });
    
    // If parent entity is provided, make the slider its child
    if let Some(parent_entity) = parent {
        commands.entity(parent_entity).add_child(bar_entity);
    }
    
    // Return the bar entity so it can be used elsewhere
    bar_entity
}

use bevy::ui::RelativeCursorPosition;

pub fn drag_slider_system<T: Resource + Slidble>(
    // Query the bar entities with interaction and relative cursor position
    mut bar_query: Query<(
        &Interaction, 
        &RelativeCursorPosition, 
        &Node, 
        &mut Slider<T>
    ), Without<SliderHandle<T>>>,
    // Query for handle entities
    mut handle_query: Query<&mut Node, With<SliderHandle<T>>>,
    // The resource we're updating
    mut setting: ResMut<T>,
) {
    // Check each bar
    for (interaction, relative_cursor, bar_node, mut slider) in &mut bar_query {
        // Only process if the bar is being clicked/dragged
        if *interaction == Interaction::Pressed {
            // Get the normalized position if available
            if let Some(normalized) = relative_cursor.normalized {
                // // The x coordinate is already normalized between 0.0 and 1.0
                // // Debugging - print the normalized value to see what we're getting
                // println!("Normalized cursor position: {:?}", normalized);
                
                let fraction = normalized.x.clamp(0.0, 1.0);
                
                // Store the fraction in the slider component
                slider.fraction = fraction;
                
                // Convert fraction to resource value 
                *setting = T::from_fraction(fraction);
                
                // Update handle position
                if let Ok(mut handle_node) = handle_query.get_mut(slider.handle) {
                    let bar_width = match bar_node.width {
                        Val::Px(px) => px,
                        _ => 200.0,
                    };
                    
                    let handle_width = match handle_node.width {
                        Val::Px(px) => px,
                        _ => 20.0,
                    };
                    
                    // Calculate max left position (bar width - handle width)
                    let max_left = bar_width - handle_width;
                    let left_offset = fraction * max_left;
                    
                    // // Print debugging info
                    // println!("Handle position calculation: fraction={}, max_left={}, left_offset={}", 
                    //          fraction, max_left, left_offset);
                    
                    // Update handle position
                    handle_node.left = Val::Px(left_offset);
                }
            } else {
                // println!("No normalized cursor position available");
            }
        }
    }
}



// Marker component for our text display
#[derive(Component)]
pub struct ReasourceText<R: Resource + Display>(pub PhantomData<R>);

/// Creates a text entity that displays a Slidble resource's value
/// 
/// This function automatically handles the component setup and initial formatting
pub fn create_setting_text<R: Resource + Display>(
    commands: &mut ChildBuilder,
    resource: &Res<R>,
) -> Entity {

    // Spawn the entity with all required components
    commands
        .spawn((
            ReasourceText::<R>(PhantomData),
            Text::new(format!("{}", resource.as_ref())),
        ))
        .id()
}

// System to update the text when the resource changes
pub fn update_resource_text<R: Resource + Display>(
    mut text_query: Query<&mut Text, With<ReasourceText<R>>>,
    resource: Res<R>,
) {    
    for mut text in text_query.iter_mut() {
        text.0 = format!("{}", resource.as_ref());
    }
}