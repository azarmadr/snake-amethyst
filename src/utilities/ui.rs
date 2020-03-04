use amethyst::{
    ui::{
        UiTransform, Anchor, UiButtonBuilder, FontAsset, UiText,
    },
    assets::{Handle},
    ecs::{World, WorldExt, Entity},
    ecs::prelude::{Builder},
};
use std::collections::HashMap;

/// Generates specified UI. 
///
pub struct UiGenerator {
    builder: UiButtonBuilder<u32, u32>,
    pub widgets: Vec<Entity>,
}
impl UiGenerator {
    const FONT_SIZE: f32 = 50f32;
    const BUTTON_INNER_OFFSET: f32 = 40f32;
    
    pub fn new(font_handle: Handle<FontAsset>) -> Self {
        let widgets = Vec::new();

        let builder = UiButtonBuilder::new("DEFAULT BUTTON")
            .with_anchor(Anchor::Middle)
            .with_font(font_handle)
            .with_font_size(Self::FONT_SIZE);
        
        Self {
            builder,
            widgets,
        }
    }

    pub fn generate_buttons(&mut self,world: &mut World, buttons: Vec<&'static str>) {
        let length = buttons.len();

        for (idx, text) in buttons.iter().enumerate() {
            let (x_size, y_size) = Self::acquire_size(text.len());
            let y = Self::acquire_y_position(world, length as u32, idx as u32);

            let (_, button) = self.builder.clone()
                .with_position(0f32, y)
                .with_size(x_size, y_size)
                .with_text(String::from(*text))
                .with_text_color([1f32,1f32,1f32,0.5f32])
                .with_hover_text_color([1f32,1f32,1f32,1f32])
                .build_from_world(world);

            self.widgets.push(button.image_entity);  
        }
    }

    fn acquire_y_position(world: &mut World, num_of_buttons: u32,idx: u32) -> f32 {
        let starting_y_offset = num_of_buttons as f32 * Self::BUTTON_INNER_OFFSET;
        let y_step = (starting_y_offset * 2f32) / (num_of_buttons - 1) as f32;

        let y = starting_y_offset - y_step * idx as f32;

        y
    }

    fn acquire_size(text_len: usize) -> (f32, f32) {
        let size_width = 0.6 * ( text_len as f32 * Self::FONT_SIZE);
        let size_height = Self::FONT_SIZE;

        (size_width, size_height)
    }

}