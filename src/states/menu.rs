use amethyst::{
    ecs::WorldExt,
    State, StateData, Trans, TransEvent, StateEvent,
    assets::{Handle, ProgressCounter, AssetStorage, Loader},
    ui::{TtfFormat,FontAsset, UiButtonBuilder, Anchor, UiText,UiEvent, UiEventType},
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
    fn handle_event(&mut self, _: StateData<GameExecData>, event: StateEvent,) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                println!("{:?}", self.generator.widgets.contains(&target));
                Trans::None
            },
            _ => Trans::None,
        }
    }

    fn update(&mut self,data: StateData<GameExecData>) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }

    fn on_stop(&mut self,data: StateData<GameExecData>) {
        let world = data.world;

        world.delete_entities(&self.generator.widgets[..]);
        self.generator.widgets.clear();
    }
}