use bevy::prelude::*;

// Define UI color constants
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
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
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
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

// // Helper functions to create common UI elements
// pub fn create_button_node(width: f32, height: f32, margin: f32) -> Node {
//     Node {
//         width: Val::Px(width),
//         height: Val::Px(height),
//         margin: UiRect::all(Val::Px(margin)),
//         justify_content: JustifyContent::Center,
//         align_items: AlignItems::Center,
//         ..default()
//     }
// }

// pub fn create_button_icon_node(width: f32, left_position: f32) -> Node {
//     Node {
//         width: Val::Px(width),
//         position_type: PositionType::Absolute,
//         left: Val::Px(left_position),
//         ..default()
//     }
// }

// pub fn create_text_font(size: f32) -> TextFont {
//     TextFont {
//         font_size: size,
//         ..default()
//     }
// }

// pub fn create_button_text_style(size: f32) -> (TextFont, TextColor) {
//     (
//         TextFont {
//             font_size: size,
//             ..default()
//         },
//         TextColor(TEXT_COLOR),
//     )
// }

// Slider components and systems will be added here later