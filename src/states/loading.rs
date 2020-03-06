use amethyst::{
    ecs::WorldExt,
    State, StateData, Trans, TransEvent, StateEvent,
    assets::{Handle, ProgressCounter, AssetStorage, Loader},
    ui::{TtfFormat,FontAsset},
    renderer::Texture,
    renderer::formats::texture::ImageFormat,
};

use crate::{
    cgd::GameExecData,
    states::MenuState,
    resources::UiResources,
};

#[derive(Default)]
pub struct LoadingState{ 
    counter: ProgressCounter,
    ui_res: Option<UiResources>,
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

        let (checked, unchecked) = { 
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            let checked = loader.load(
                "checkbox_checked.png",
                ImageFormat::default(),
                &mut self.counter,
                &texture_storage,
            ); 
            let unchecked = loader.load(
                "checkbox_unchecked.png",
                ImageFormat::default(),
                &mut self.counter,
                &texture_storage,
            );
            (checked, unchecked)
        };
        self.ui_res = Some(UiResources::new(font_handle, checked, unchecked));
    }

    fn update(&mut self,data: StateData<GameExecData>) -> Trans<GameExecData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);

        if self.counter.is_complete() {
            if let Some(ref ui_res) = &self.ui_res {
                return Trans::Switch(Box::new(MenuState::new(ui_res.clone())));
            }
            
        } 
        Trans::None
    }
}