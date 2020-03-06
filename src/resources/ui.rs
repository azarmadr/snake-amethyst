use amethyst::{
    assets::{Handle},
    ui::{FontAsset},
    renderer::Texture,
};

#[derive(Clone)]
pub struct UiResources {
    pub font_handle: Handle<FontAsset>,
    pub checked_button: Handle<Texture>,
    pub unchecked_button: Handle<Texture>,
    pub settings: UiSettings,
}

impl  UiResources {
    pub fn new(font_handle: Handle<FontAsset>, checked_button: Handle<Texture>,unchecked_button: Handle<Texture>,) -> Self {
        let settings = UiSettings::default();
        
        Self {
            font_handle,
            checked_button,
            unchecked_button,
            settings
        }
    }
}

#[derive(Default, Clone)]
pub struct UiSettings {
    pub fullscreen: bool,
    pub music_on: bool,
    pub sound_on: bool,
}