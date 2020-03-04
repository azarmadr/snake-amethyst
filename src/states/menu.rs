use amethyst::{
    ecs::WorldExt,
    State, StateData, Trans, TransEvent, StateEvent,
    assets::{Handle, ProgressCounter, AssetStorage, Loader},
    renderer::SpriteSheet,
    ui::{TtfFormat,FontAsset, UiButtonBuilder, Anchor, UiText},
};
use crate::{
    cgd::GameExecData,
    utilities::UiGenerator,
};


pub struct MenuState {
    generator: UiGenerator,
}
impl MenuState {
    pub fn new(font_handle: Handle<FontAsset>) -> Self {
        let generator = UiGenerator::new(font_handle.clone());
        
        Self {
            generator,
        }
    }
}

impl<'a, 'b> State<GameExecData<'a, 'b>, StateEvent> for MenuState {
    fn on_start(&mut self,data: StateData<GameExecData>) {
        println!("Switched to `MenuState`");

        let world = data.world;
        
        crate::utilities::initialize_camera(world);
        self.generator.generate_buttons(world, vec!["NEW GAME", "OPTIONS", "QUIT"]);
    }
    fn update(&mut self,data: StateData<GameExecData>) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }
}