use amethyst::{
    Trans, StateEvent,
    ui::{
        UiTransform, Anchor, FontAsset, UiText, ScaleMode, Interactable, UiImage,
    },
    core::transform::Parent,
    assets::{Handle},
    ecs::{World, WorldExt, Entity},
    ecs::prelude::{Builder},
};

use std::collections::HashMap;
use crate::{
    cgd::GameExecData,
    resources::UiResources,
};

/// Generates specified UI. 
///
pub struct UiGenerator;

impl UiGenerator {
    const FONT_SIZE: f32 = 50f32;
    const BUTTON_INNER_OFFSET: f32 = 40f32;
    const HOVERED_COLOR: [f32;4] = [1f32,1f32,1f32, 1f32];
    const DEFAULT_COLOR: [f32;4] = [1f32,1f32,1f32, 0.5];

    pub fn generate_buttons(world: &mut World,ui_res: UiResources,buttons: Vec<(&'static str,UiEffect, bool)>) -> HashMap<Entity, UiEffect> {
        let length = buttons.len();
        let mut widgets = HashMap::new();

        for (idx, (text, ui_effect, is_check)) in buttons.iter().enumerate() {
            let (x_size, y_size) = Self::acquire_size(text.len());
            let y = Self::acquire_y_position(length as u32, idx as u32);

            let text_str = String::from(*text);

            let ui_text = UiText::new(
                ui_res.font_handle.clone(),
                text_str.clone(),
                Self::DEFAULT_COLOR,
                Self::FONT_SIZE,
            );
    
            let ui_trans = UiTransform::new(
                text_str.clone(),
                Anchor::Middle,
                Anchor::Middle,
                0f32,
                y,
                0f32,
                x_size,
                y_size,
            );

            let entity = world.create_entity()
            .with(ui_text)
            .with(ui_trans)
            .with(Interactable)
            .build();

            if *is_check {
                let ui_image = UiImage::Texture(ui_res.checked_button.clone());
                let ui_image_trans = UiTransform::new(
                    text_str.clone(),
                    Anchor::Middle,
                    Anchor::Middle,
                    Self::BUTTON_INNER_OFFSET + x_size / 2f32,
                    0f32,
                    0f32,
                    64f32,
                    64f32,
                );
                
                world.create_entity()
                .with(ui_image)
                .with(ui_image_trans)
                .with(Interactable)
                .with(Parent::new(entity))
                .build();
            }

            widgets.insert(entity, *ui_effect);
        }
        widgets
    }
    fn acquire_y_position(num_of_buttons: u32,idx: u32) -> f32 {
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

    pub fn set_hover_color(txt: &mut UiText) {
        txt.color = Self::HOVERED_COLOR;
    }
    pub fn unset_hover_color(txt: &mut UiText) {
        txt.color = Self::DEFAULT_COLOR;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UiEffect {
    NewGame,
    Options,
    Quit,
    None,
}

use crate::states::{GameplayState, OptionsState};

impl<'a,'b> UiEffect {
    pub fn as_trans(ui_effect: Self,ui_res: UiResources) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        match ui_effect {
            Self::NewGame => Trans::Push(Box::new(GameplayState)),
            Self::Options => Trans::Push(Box::new(OptionsState::new(ui_res.clone()))),
            Self::Quit => Trans::Quit, 
            Self::None => Trans::None,
        }
    }
}