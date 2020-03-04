use amethyst::{
    ecs::WorldExt,
    State, StateData, Trans, TransEvent, StateEvent,
    assets::{Handle, ProgressCounter, AssetStorage, Loader},
    renderer::SpriteSheet,
    ui::{TtfFormat,FontAsset},
};

use crate::{
    cgd::GameExecData,
    states::MenuState,
};

#[derive(Default)]
pub struct LoadingState{ 
    counter: ProgressCounter,
    font_handle: Option<Handle<FontAsset>>,
}

impl<'a, 'b> State<GameExecData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self,data: StateData<GameExecData>) {
        let world = data.world;

        let font_handle = { 
            let loader = world.read_resource::<Loader>();
            let font_storage = world.read_resource::<AssetStorage<FontAsset>>();
            loader.load(
                "Prototype.ttf",
                TtfFormat,
                &mut self.counter,
                &font_storage,
            )
        };
        self.font_handle = Some(font_handle);

        //let sh = crate::utilities::decompile_spritesheet(world, "basic_tile.png", (65f32, 130f32), (64f32, 64f32));
        //self.sheet_handle = Some(sh)
    }
    fn update(&mut self,data: StateData<GameExecData>) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);

        if self.counter.is_complete() {
            if let Some(ref font_handle) = &self.font_handle {
                return Trans::Switch(Box::new(MenuState::new(font_handle.clone())));
            }
            
        } 
        Trans::None
    }
}