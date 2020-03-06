mod menu;
mod gameplay;
mod loading;
mod options;

pub use self::{
    loading::LoadingState,
    menu::MenuState,
    gameplay::GameplayState,
    options::OptionsState,
};